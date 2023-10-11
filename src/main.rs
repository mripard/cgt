#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::unnecessary_wraps)]
#![doc = include_str!("../README.md")]

use colored::Colorize;

use cgt_core::{run_all, DeviceSpecifier, RunResult, Test, TestResult, TestResultWriter};

mod tests;

#[derive(Default)]
struct ConsoleResultWriter {
    num_tests: usize,
    successful_tests: usize,
    failing_tests: usize,
}

impl TestResultWriter for ConsoleResultWriter {
    fn new() -> Self {
        Self::default()
    }

    fn start_suite(&mut self, name: &str, tests: &[Test]) {
        println!("\nRunning {} ({} tests)\n", name, tests.len());
    }

    fn write_test(&mut self, test: &Test) {
        print!("    {}", test.test_name.bold());
        self.num_tests += 1;
    }

    fn write_result(&mut self, _test: &Test, res: &TestResult) {
        match res {
            TestResult::Success => {
                println!("\t{}", "✔".green().bold());
                self.successful_tests += 1;
            }
            TestResult::Failure(e) => {
                println!("\t{}", format!("✘ -> {e}").red().bold());
                self.failing_tests += 1;
            }
        }
    }

    fn end_suite(&mut self) {
        println!(
            "\n{}",
            format!(
                "Test Results: {}; {} passed; {} failed",
                if self.failing_tests > 0 {
                    "failed".red()
                } else {
                    "ok".green()
                },
                self.successful_tests,
                self.failing_tests
            )
            .bold()
        );

        self.num_tests = 0;
        self.successful_tests = 0;
        self.failing_tests = 0;
    }
}

fn main() -> RunResult {
    let mut writer = ConsoleResultWriter::new();

    run_all(
        &mut writer,
        DeviceSpecifier::ModuleName(String::from("vkms")),
    )
}
