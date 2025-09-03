use crate::compiler::compile_package;
use anyhow::{bail, Error};
use codespan::Span;
use legacy_move_compiler::compiled_unit::{CompiledUnit, NamedCompiledModule};
use move_binary_format::{
    access::ModuleAccess,
    file_format::{CodeOffset, FunctionDefinitionIndex},
};
use move_bytecode_source_map::source_map::SourceMap;
use move_coverage::coverage_map::CoverageMap;
use move_ir_types::location::Loc as IrLoc;
use move_model::model::Loc;
use move_package::BuildConfig;
use std::{collections::BTreeMap, path::Path};

const COVERAGE_MAP_NAME: &str = ".coverage_map.mvcov";

/// Contains all covered spans in the project.
#[derive(Debug, Default)]
pub(crate) struct Coverage {
    /// List of all covered spans for all functions for all modules.
    // The key is a qualified function name (e.g. "vector::append").
    all_covered_spans: BTreeMap<String, Vec<Span>>,
}

impl Coverage {
    /// Compute coverage for the project.
    pub(crate) fn compute_coverage(
        &mut self,
        build_config: &BuildConfig,
        package_path: &Path,
    ) -> anyhow::Result<()> {
        info!("computing coverage");

        let coverage_file = package_path.join(COVERAGE_MAP_NAME);
        if !coverage_file.exists() {
            bail!(
                "Coverage map not found, please run `aptos move test --coverage` for the package"
            );
        }

        let coverage_map = CoverageMap::from_binary_file(&coverage_file)
            .map_err(|e| Error::msg(format!("failed to retrieve the coverage map: {e}")))?;

        let mut coverage_config = build_config.clone();
        coverage_config.test_mode = false;
        let package = compile_package(coverage_config, package_path)?;

        let root_modules: Vec<_> = package
            .root_modules()
            .map(|unit| match &unit.unit {
                CompiledUnit::Module(NamedCompiledModule {
                    module, source_map, ..
                }) => (module, source_map),
                _ => unreachable!("Should all be modules"),
            })
            .collect();

        let all_covered_spans = compute_function_covered_spans(&coverage_map, root_modules);

        trace!("all covered spans: {all_covered_spans:?}");
        self.all_covered_spans = all_covered_spans;
        Ok(())
    }

    /// Check if the location is covered by the unit test.
    /// Returns true if the location is covered, false if uncovered.
    pub(crate) fn check_location(&self, associated_fn_name: String, loc: &Loc) -> bool {
        let span = loc.span();

        let Some(covered_spans) = self.all_covered_spans.get(&associated_fn_name) else {
            trace!("location has no coverage since {associated_fn_name} has no covered spans");
            return false;
        };

        for covered_span in covered_spans {
            if spans_overlap(span, *covered_span) {
                trace!("{associated_fn_name} has coverage for the given location");
                return true;
            }
        }

        // Span doesn't overlap with any covered span, so it's uncovered
        trace!("{associated_fn_name} has no coverage for the given location");
        false
    }
}

/// Compute per-function covered spans with function names preserved.
/// Returns a map from qualified function names (e.g., "vector::append") to their covered spans.
/// Only functions with some covered code are included in the result.
fn compute_function_covered_spans(
    coverage_map: &CoverageMap,
    root_modules: Vec<(&move_binary_format::CompiledModule, &SourceMap)>,
) -> BTreeMap<String, Vec<Span>> {
    let unified_exec_map = coverage_map.to_unified_exec_map();
    let mut function_covered_map = BTreeMap::new();

    for (module, source_map) in root_modules.iter() {
        let module_name = module.self_id();
        let module_map = unified_exec_map
            .module_maps
            .get(&(*module_name.address(), module_name.name().to_owned()));
        if let Some(module_map) = module_map {
            for (function_def_idx, function_def) in module.function_defs().iter().enumerate() {
                let fn_handle = module.function_handle_at(function_def.function);
                let fn_name = module.identifier_at(fn_handle.name).to_owned();
                let function_def_idx = FunctionDefinitionIndex(function_def_idx as u16);

                // Calculate covered locations for this specific function
                let covered_ir_locs: Vec<IrLoc> = match &function_def.code {
                    None => vec![], // Native functions have no covered locations to track
                    Some(code_unit) => match module_map.function_maps.get(&fn_name) {
                        None => vec![], // Function has no coverage data - no covered locations
                        Some(function_coverage) => {
                            // Extract only covered locations (execution count > 0)
                            let covered_locs: Vec<IrLoc> = (0..code_unit.code.len())
                                .filter_map(|code_offset| {
                                    if let Ok(loc) = source_map.get_code_location(
                                        function_def_idx,
                                        code_offset as CodeOffset,
                                    ) {
                                        // If execution count > 0, it's covered
                                        if function_coverage
                                            .get(&(code_offset as u64))
                                            .unwrap_or(&0)
                                            > &0
                                        {
                                            Some(loc)
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            minimize_ir_locations(covered_locs)
                        },
                    },
                };

                // Only include functions that have covered locations
                if !covered_ir_locs.is_empty() {
                    // Convert IrLoc to Span for easier comparison with move_model::Loc.span()
                    let covered_spans: Vec<Span> = covered_ir_locs
                        .into_iter()
                        .map(|ir_loc| Span::new(ir_loc.start(), ir_loc.end()))
                        .collect();

                    let qualified_fn_name = format!("{}::{}", module.self_name(), fn_name);
                    function_covered_map.insert(qualified_fn_name, covered_spans);
                }
            }
        }
    }

    function_covered_map
}

/// Helper function to minimize IR locations by merging overlapping/adjacent ones
fn minimize_ir_locations(mut locs: Vec<IrLoc>) -> Vec<IrLoc> {
    locs.sort();
    let mut result = vec![];
    let mut locs_iter = locs.into_iter();
    if let Some(mut current_loc) = locs_iter.next() {
        for next_loc in locs_iter {
            if !current_loc.try_merge(&next_loc) {
                result.push(current_loc);
                current_loc = next_loc;
            }
        }
        result.push(current_loc);
    }
    result
}

/// Check if two spans overlap
/// Two spans overlap if one starts before the other ends
fn spans_overlap(span1: Span, span2: Span) -> bool {
    span1.start() < span2.end() && span2.start() < span1.end()
}
