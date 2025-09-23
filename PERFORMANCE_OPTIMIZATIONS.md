# Move Mutation Generation Performance Optimizations

## Executive Summary

The current mutation generation process is extremely slow due to several critical bottlenecks. This document outlines performance improvements ranked by impact, with implementation difficulty and confidence scores. The top 3 optimizations alone could provide 10-50x speedup.

---

## Performance Optimizations (Ranked by Impact)

### 1. **Remove/Make Optional Mutant Verification**
- **Impact**: 🔥🔥🔥🔥🔥 (90% time reduction)
- **Difficulty**: 1/10
- **Confidence**: 100%
- **Current Problem**: Every mutant triggers full package copy + compilation
- **Solution**: Make `--verify-mutants` opt-in, not default
- **Implementation**:
```rust
// In move-mutator/src/lib.rs:162
if mutator_configuration.project.verify_mutants {
    // Only verify if explicitly requested
}
```
- **Detailed Explanation**:
  - Currently EVERY generated mutant is verified by copying the entire package to a temp directory and recompiling
  - For 1000 mutants on a 10MB package = 10GB of file I/O + 1000 compilations
  - Most users don't need verification during generation (they'll compile during testing anyway)
  - This single change provides immediate 10-50x speedup with zero complexity

### 2. **In-Memory Compilation for Verification**
- **Impact**: 🔥🔥🔥🔥🔥 (80% verification time reduction)
- **Difficulty**: 7/10
- **Confidence**: 85%
- **Current Problem**: Physical file I/O for each verification
- **Solution**: Virtual filesystem + in-memory source modification
- **Implementation Strategy**:
```rust
pub struct InMemoryCompiler {
    base_package: CompiledPackage,
    cached_deps: HashMap<String, CompiledModule>,
    virtual_fs: VirtualFileSystem,
}

impl InMemoryCompiler {
    fn verify_mutant(&mut self, mutant: &str, file_path: &Path) -> Result<()> {
        // 1. Clone virtual filesystem (cheap)
        let mut vfs = self.virtual_fs.clone();

        // 2. Update single file in memory
        vfs.update_file(file_path, mutant);

        // 3. Compile using cached dependencies
        let result = compile_with_vfs(&vfs, &self.cached_deps);

        // 4. Return verification result
        result.map(|_| ())
    }
}
```
- **Detailed Explanation**:
  - Setup: Copy package ONCE, compile dependencies ONCE
  - Per-mutant: Only recompile changed module using cached deps
  - Eliminates: File I/O, dependency recompilation, temp directory management
  - Technical challenges: Integrating with Move compiler's file system abstraction
  - Memory overhead: ~100MB for cached dependencies (acceptable tradeoff)

### 3. **Lazy/Streaming Mutant Generation**
- **Impact**: 🔥🔥🔥🔥 (60% memory reduction, prevents OOM)
- **Difficulty**: 4/10
- **Confidence**: 90%
- **Current Problem**: All mutants generated and held in memory
- **Solution**: Iterator-based lazy generation
- **Implementation**:
```rust
pub struct MutantStream<'a> {
    env: &'a GlobalEnv,
    modules: Vec<ModuleEnv<'a>>,
    current_module: usize,
    current_function: usize,
    current_operator: usize,
    operators: Vec<Box<dyn MutationOperator>>,
}

impl<'a> Iterator for MutantStream<'a> {
    type Item = MutantInfo;

    fn next(&mut self) -> Option<Self::Item> {
        // Generate one mutant at a time
        // Move to next operator/function/module as needed
        // Return None when exhausted
    }
}

// Usage:
let mutant_stream = MutantStream::new(&env, &config);
for mutant in mutant_stream {
    // Process immediately, write to disk
    // Never hold all mutants in memory
}
```
- **Detailed Explanation**:
  - Current: Generate 10,000 mutants → hold in memory → process
  - New: Generate 1 mutant → process → write → repeat
  - Memory usage: O(1) instead of O(n)
  - Enables: Processing millions of mutants without OOM
  - Side benefit: Can show progress in real-time

### 4. **Parallel Module Processing**
- **Impact**: 🔥🔥🔥🔥 (3-4x speedup on multicore)
- **Difficulty**: 3/10
- **Confidence**: 95%
- **Current Problem**: Sequential module traversal
- **Solution**: Parallel iteration with Rayon
- **Implementation**:
```rust
// Current (move-mutator/src/mutate.rs:27-31)
let mutants = env.get_modules()
    .map(|module| traverse_module_with_check(&module, conf))
    .collect::<Result<Vec<_>, _>>()?
    .concat();

// Optimized
use rayon::prelude::*;
let mutants = env.get_modules()
    .collect::<Vec<_>>()
    .par_iter()
    .map(|module| traverse_module_with_check(module, conf))
    .collect::<Result<Vec<_>, _>>()?
    .into_iter()
    .flatten()
    .collect();
```
- **Detailed Explanation**:
  - AST traversal is CPU-bound and embarrassingly parallel
  - Each module is independent
  - Linear speedup with CPU cores (8 cores = ~7x speedup)
  - Minimal code changes required

### 5. **Incremental Compilation Cache**
- **Impact**: 🔥🔥🔥🔥 (70% compilation time reduction)
- **Difficulty**: 6/10
- **Confidence**: 80%
- **Current Problem**: Recompiling everything for each mutant
- **Solution**: Cache compilation artifacts
- **Implementation**:
```rust
pub struct CompilationCache {
    // Cache compiled dependencies
    compiled_deps: HashMap<ModuleId, CompiledModule>,
    // Cache type checking results
    type_cache: HashMap<NodeId, Type>,
    // Cache name resolution
    name_resolution: NameResolutionCache,
    // Base compilation environment
    base_env: GlobalEnv,
}

impl CompilationCache {
    fn compile_with_cache(&mut self, mutated_source: &str) -> Result<()> {
        // 1. Parse only the changed module
        let changed_ast = parse_module(mutated_source)?;

        // 2. Reuse cached dependencies
        let env = self.base_env.clone_with_module(changed_ast);

        // 3. Type check using cached types for unchanged parts
        type_check_incremental(&env, &self.type_cache)?;

        // 4. Link with cached dependencies
        link_with_deps(&env, &self.compiled_deps)
    }
}
```

### 6. **Smart Verification Sampling**
- **Impact**: 🔥🔥🔥 (90% reduction in verification time)
- **Difficulty**: 3/10
- **Confidence**: 85%
- **Current Problem**: Verifying ALL mutants
- **Solution**: Statistical sampling based on operator confidence
- **Implementation**:
```rust
struct VerificationSampler {
    operator_success_rates: HashMap<String, f64>,
    sampling_rate: f64, // e.g., 0.1 for 10%
}

impl VerificationSampler {
    fn should_verify(&self, operator: &str) -> bool {
        let confidence = self.operator_success_rates.get(operator).unwrap_or(&0.5);
        if confidence > 0.95 {
            // High confidence operators: verify 1%
            rand::random::<f64>() < 0.01
        } else if confidence > 0.8 {
            // Medium confidence: verify 10%
            rand::random::<f64>() < 0.1
        } else {
            // Low confidence: verify 50%
            rand::random::<f64>() < 0.5
        }
    }
}
```

### 7. **Streaming File Output**
- **Impact**: 🔥🔥🔥 (50% memory reduction)
- **Difficulty**: 2/10
- **Confidence**: 95%
- **Current Problem**: Collecting all mutants before writing
- **Solution**: Write immediately as generated
- **Implementation**:
```rust
// Instead of collecting then writing
let all_mutants = generate_all_mutants();
for mutant in all_mutants {
    write_to_disk(mutant);
}

// Stream directly to disk
let mutant_generator = MutantGenerator::new();
for mutant in mutant_generator {
    let path = get_mutant_path(mutant.id);
    fs::write(path, &mutant.source)?;
    report.add_streaming(mutant.metadata);
}
```

### 8. **Coverage Span Optimization with Interval Tree**
- **Impact**: 🔥🔥🔥 (10x faster coverage checks)
- **Difficulty**: 4/10
- **Confidence**: 75%
- **Current Problem**: O(n) coverage span checks
- **Solution**: O(log n) interval tree lookups
- **Implementation**:
```rust
use intervaltree::IntervalTree;

struct OptimizedCoverage {
    // Instead of Vec<Span>, use interval tree
    coverage_tree: IntervalTree<u32, ()>,
}

impl OptimizedCoverage {
    fn is_covered(&self, span: Span) -> bool {
        // O(log n) instead of O(n)
        !self.coverage_tree.query(span.start..span.end).is_empty()
    }
}
```

### 9. **Operator Batching in Single AST Pass**
- **Impact**: 🔥🔥🔥 (30% AST traversal speedup)
- **Difficulty**: 5/10
- **Confidence**: 80%
- **Current Problem**: Multiple operators checked separately
- **Solution**: Apply all applicable operators at once
- **Implementation**:
```rust
fn apply_all_operators(exp: &ExpData, ops: &[Box<dyn MutationOperator>]) -> Vec<MutantInfo> {
    let mut mutants = Vec::new();

    // Single pattern match, multiple operator applications
    match exp {
        ExpData::Call(_, op, _) if is_binary_op(op) => {
            mutants.extend(ops[BINARY].apply(exp));
            mutants.extend(ops[BINARY_SWAP].apply(exp));
        }
        ExpData::Value(_, _) => {
            mutants.extend(ops[LITERAL].apply(exp));
        }
        // ... other patterns
    }

    mutants
}
```

### 10. **Increase Rayon Chunk Size**
- **Impact**: 🔥🔥 (20% parallel processing improvement)
- **Difficulty**: 1/10
- **Confidence**: 90%
- **Current Problem**: Small chunks cause overhead
- **Solution**: Increase from 64 to 256-512
- **Implementation**:
```rust
// move-mutation-test/src/lib.rs:113
const CHUNK_SIZE: usize = 512; // was 64
```

### 11. **AST Node Caching**
- **Impact**: 🔥🔥 (15% traversal speedup)
- **Difficulty**: 5/10
- **Confidence**: 70%
- **Current Problem**: Repeated type/location lookups
- **Solution**: Cache frequently accessed node data

### 12. **Compilation Syntax Pre-check**
- **Impact**: 🔥🔥 (Skip 30% of invalid mutants early)
- **Difficulty**: 6/10
- **Confidence**: 65%
- **Current Problem**: Full compilation for syntax errors
- **Solution**: Lightweight syntax validation

### 13. **Memory-Mapped File Operations**
- **Impact**: 🔥 (10% I/O improvement)
- **Difficulty**: 3/10
- **Confidence**: 60%
- **Current Problem**: Standard file I/O
- **Solution**: mmap for large source files

### 14. **Source Modification with Ropes**
- **Impact**: 🔥 (5% string operation improvement)
- **Difficulty**: 4/10
- **Confidence**: 70%
- **Current Problem**: O(n) string operations
- **Solution**: O(log n) rope operations

### 15. **Buffered File I/O**
- **Impact**: 🔥 (5% I/O improvement)
- **Difficulty**: 2/10
- **Confidence**: 85%
- **Current Problem**: Unbuffered writes
- **Solution**: BufWriter with large buffer

### 16. **Rayon Thread Pool Tuning**
- **Impact**: 🔥 (5% parallel improvement)
- **Difficulty**: 2/10
- **Confidence**: 75%
- **Current Problem**: Default thread configuration
- **Solution**: Optimize for workload

### 17. **Bloom Filter for Duplicate Detection**
- **Impact**: 🔥 (2% deduplication improvement)
- **Difficulty**: 3/10
- **Confidence**: 50%
- **Current Problem**: No duplicate detection
- **Solution**: Probabilistic duplicate filtering

---

## Implementation Roadmap

### Phase 1: Quick Wins (1 day)
1. Remove default verification (Impact: 90%)
2. Increase Rayon chunk size (Impact: 20%)
3. Add `--fast` mode flag

### Phase 2: Core Optimizations (1 week)
1. Lazy mutant generation (Impact: 60%)
2. Parallel module processing (Impact: 300%)
3. Streaming file output (Impact: 50%)

### Phase 3: Advanced Optimizations (2-3 weeks)
1. In-memory compilation (Impact: 80%)
2. Incremental compilation cache (Impact: 70%)
3. Smart verification sampling (Impact: 90%)

### Phase 4: Polish (1 week)
1. Coverage optimization
2. Operator batching
3. Minor I/O improvements

---

## Expected Cumulative Performance Gains

- **Without verification**: 10-50x faster
- **With verification (optimized)**: 5-10x faster
- **Memory usage**: 60-80% reduction
- **Large codebases**: From hours to minutes
- **Small codebases**: From minutes to seconds

---

## Risk Analysis

### High Confidence Optimizations (>90%)
- Remove default verification
- Parallel processing
- Streaming output
- Chunk size increase

### Medium Confidence Optimizations (70-90%)
- Lazy generation
- In-memory compilation
- Smart sampling
- Coverage optimization

### Lower Confidence Optimizations (<70%)
- AST caching (complexity vs benefit)
- Compilation pre-checks (false positives)
- Memory mapping (platform dependencies)

---

## Benchmarking Recommendations

1. Create standard benchmark suite with:
   - Small (10 files), Medium (100 files), Large (1000 files) packages
   - Measure: Time, Memory, Disk I/O

2. Track metrics:
   - Mutants per second
   - Memory peak usage
   - Disk I/O operations
   - CPU utilization

3. Set performance targets:
   - Small package: <1 second
   - Medium package: <10 seconds
   - Large package: <2 minutes