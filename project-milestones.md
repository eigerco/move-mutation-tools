# Extension Project for Move Mutation Testing Tools

## Overview

This proposal describes an extension project to enhance the Move Mutation Testing Suite. The objective is to ensure the tools remain fully compatible with the latest Move language features—including support for function values—while significantly improving performance, developer experience, and maintainability.

These updates are designed to make mutation testing more practical, faster to use in day-to-day development, and easier to integrate into standard release and continuous integration processes. They will help foster broader adoption across the Aptos developer ecosystem, strengthening the quality of smart contract code deployed on the network.

## 

## Scope and Milestones

### Milestone 1: Language compatibility update & automation

**Description:**  
Refactor and update mainly `move-mutator`, but it may also require interacting with `move-mutation-test` and `move-spec-test` tools to ensure compatibility with the current Move language version. This work will primarily involve updating dependency versions and, if needed, making changes to parser or AST transformations to align with recent Aptos-core crate updates.

This milestone will begin with a short analysis of language changes between the current and implemented versions. Tests are needed to check if there is no need to introduce changes to the existing codebase.

The second part involves replacing the existing manual release process with an automated, repeatable, and maintainable GitHub Actions workflow. This automation will ensure that binaries are consistently built, tested, and uploaded as GitHub Releases in a form directly consumable by developers via the `aptos` CLI extension mechanism (e.g., `aptos update move-mutation-test`).

Define and implement GitHub Actions workflows for:

* Building release binaries across supported platforms.  
* Running automated tests on built artifacts.  
* Packaging and uploading artifacts to GitHub Releases.

Document the release process to ensure maintainers can maintain long-term sustainability.

**Deliverables:**

* Fully updated mutation testing tools compatible with the latest Move language features.  
* Validated example runs on real Aptos Move packages demonstrating functional support for function values and other new features.  
* Production-ready GitHub Actions workflows are integrated into the repository.  
* Maintainer-facing documentation for managing releases and troubleshooting automation.

### Milestone 2:  In-depth case studies

**Description:**  
Showcase the power of mutation testing \- run the updated test suite on several real Move codebases. Generate the reports and prepare blog posts and presentations with analysis of the findings.

**Deliverables:**

* Real case analysis.  
* Blog post.

### Milestone 3: Performance Improvements

**Description:**  
The third milestone targets specific, high-impact performance optimizations designed to make mutation testing viable for larger codebases. The first part of this milestone involves implementing a feature that allows immediate termination of test execution upon the first failure when testing a mutant. This ensures mutants are marked as killed as soon as possible and avoids wasting resources running the full test suite unnecessarily. Achieving this may require integration with, or contributions to, the Aptos-core Move unit test runner to support early exit behavior if it is not already available.

The second part of this milestone focuses on redesigning the test execution strategy to use a two-phase approach. In the first phase, only the tests belonging to the module containing the mutant will run, maximizing the chance of quickly detecting the mutation. If the mutant survives this targeted testing, the remaining tests in the package will then run. This approach leverages existing filtering options in the `aptos move test` CLI, and may require contributing a new `--filter-ignore` feature to exclude previously executed tests efficiently. These improvements will reduce redundant test execution and make mutation testing significantly faster and more practical for large-scale projects. This milestone is estimated to require approximately three weeks of development time.

**Deliverables:**

* New configuration option in the testing tools.  
* Integration with Move’s unit test runner (including upstream contribution if needed).  
* Updated test runner logic in mutation testing tools.  
* Optional contributions to aptos-core Move CLI for additional filtering support.

### Milestone 4: Developer Experience Improvements

**Description:**  
Collect and report statistics on how effective each mutation operator is at generating meaningful (i.e., killed) mutants. This analysis will help identify which operators contribute most to uncovering weaknesses in the test suite and which may be less useful.

Based on the effectiveness data, introduce configurable running modes (e.g., light, medium, heavy):

* Light mode: Runs only the most effective operators, providing a quick feedback loop and faster results.  
* Medium mode: Includes a broader set of operators, balancing speed and thoroughness.  
* Heavy mode: Runs all available operators for maximum coverage and rigor.

**Deliverables:**

* Operator effectiveness analysis and reporting.  
* Configurable modes in the CLI.
