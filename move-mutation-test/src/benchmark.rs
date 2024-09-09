// Copyright © Eiger
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use std::time::{Duration, Instant};

/// A benchmark for a specific operation.
#[derive(Debug, Clone)]
pub struct Benchmark {
    /// Start time of the operation.
    pub start_time: Instant,
    /// Duration of the operation.
    pub elapsed: Duration,
}

impl Benchmark {
    /// Creates a new benchmark.
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            elapsed: Duration::new(0, 0),
        }
    }

    /// Starts the benchmark.
    pub fn start(&mut self) {
        self.start_time = Instant::now();
    }

    /// Stops the benchmark.
    pub fn stop(&mut self) {
        self.elapsed = self.start_time.elapsed();
    }
}

/// A collection of benchmarks for the mutation testing.
pub struct Benchmarks {
    /// Overall benchmark for the mutation test.
    pub total_duration: Benchmark,
    /// Benchmark for the mutator.
    pub mutator: Benchmark,
    /// Benchmark for the mutation test.
    pub mutation_test: Benchmark,
    /// Benchmark for mutation test results.
    pub mutation_test_results: Vec<Benchmark>,
}

impl Benchmarks {
    /// Creates a new collection of benchmarks.
    pub fn new() -> Self {
        Self {
            total_duration: Benchmark::new(),
            mutator: Benchmark::new(),
            mutation_test: Benchmark::new(),
            mutation_test_results: Vec::new(),
        }
    }

    /// Displays the benchmarks with the `RUST_LOG` info level.
    pub fn display(&self) {
        info!(
            "In total, mutation testing took {} msecs",
            self.total_duration.elapsed.as_millis()
        );
        info!(
            "Generating mutants took {} msecs",
            self.mutator.elapsed.as_millis()
        );
        info!(
            "Mutation testing took {} msecs",
            self.mutation_test.elapsed.as_millis()
        );
        if !self.mutation_test_results.is_empty() {
            info!(
                "Min mutation testing time for a mutant: {} msecs",
                self.mutation_test_results
                    .iter()
                    .map(|f| f.elapsed.as_millis())
                    .min()
                    .unwrap()
            );
            info!(
                "Max mutation testing time for a mutant: {} msecs",
                self.mutation_test_results
                    .iter()
                    .map(|f| f.elapsed.as_millis())
                    .max()
                    .unwrap()
            );
            info!(
                "Average mutation testing time for each mutant: {} msecs",
                self.mutation_test_results
                    .iter()
                    .map(|f| f.elapsed.as_millis())
                    .sum::<u128>()
                    / self.mutation_test_results.len() as u128
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time::Duration};

    #[test]
    fn benchmark_records_correct_elapsed_time() {
        let mut benchmark = Benchmark::new();
        benchmark.start();
        thread::sleep(Duration::from_millis(100));
        benchmark.stop();
        assert!(benchmark.elapsed >= Duration::from_millis(100));
    }

    #[test]
    fn benchmark_does_not_record_time_before_start() {
        let mut benchmark = Benchmark::new();
        thread::sleep(Duration::from_millis(100));
        benchmark.start();
        thread::sleep(Duration::from_millis(100));
        benchmark.stop();
        assert!(benchmark.elapsed < Duration::from_millis(200));
    }

    #[test]
    fn benchmark_does_not_record_time_after_stop() {
        let mut benchmark = Benchmark::new();
        benchmark.start();
        thread::sleep(Duration::from_millis(100));
        benchmark.stop();
        thread::sleep(Duration::from_millis(100));
        assert!(benchmark.elapsed < Duration::from_millis(200));
    }

    #[test]
    fn benchmarks_records_multiple_benchmarks() {
        let mut benchmarks = Benchmarks {
            total_duration: Benchmark::new(),
            mutator: Benchmark::new(),
            mutation_test: Benchmark::new(),
            mutation_test_results: Vec::new(),
        };

        benchmarks.total_duration.start();
        thread::sleep(Duration::from_millis(100));
        benchmarks.total_duration.stop();

        benchmarks.mutator.start();
        thread::sleep(Duration::from_millis(100));
        benchmarks.mutator.stop();

        benchmarks.mutation_test.start();
        thread::sleep(Duration::from_millis(100));
        benchmarks.mutation_test.stop();

        assert!(benchmarks.total_duration.elapsed >= Duration::from_millis(100));
        assert!(benchmarks.mutator.elapsed >= Duration::from_millis(100));
        assert!(benchmarks.mutation_test.elapsed >= Duration::from_millis(100));
    }
}
