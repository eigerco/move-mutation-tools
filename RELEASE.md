# Release Process Documentation

This document outlines the release process for Move Mutation Tools, which follows [SemVer](https://semver.org/).

## Release Workflow

### Prerequisites

1. Ensure all tests pass on the main branch
2. Verify compatibility with the target Aptos version
3. Ensure all tools have matching version numbers in their Cargo.toml files

### Step-by-Step Process

#### 1. Prepare the Release

```bash
# Ensure you're on main branch
git checkout main
git pull origin main
```

# Run all tests

This command will take some time, especially if you're building the project for the first time.

```bash
cargo nextest run -r
```

#### 1. Update Version Numbers

Update all three Cargo.toml files to the same release version (example below is with v2.0.0):
- `move-mutation-test/Cargo.toml`
- `move-mutator/Cargo.toml`
- `move-spec-test/Cargo.toml`

#### 2. Create and Push a Tag with the release version

```bash
# Commit version updates
git add -A
git commit -m "chore: Bump version to v2.0.0"
git push origin main

# Create and push tag
git tag -a v2.0.0 -m "Release v2.0.0 - Compatible with Aptos v1.35.0"
git push origin v2.0.0
```

#### 3. Monitor the Release

When you push the tag on the main branch a GitHub Actions workflow will be triggered.
The workflow will automatically:
1. Build binaries for all supported platforms (Linux, macOS Intel/ARM, Windows)
2. Run tests, which compares a pre-generated report with a report generated from each freshly built binary
3. Create a draft GitHub release
4. Upload artifacts to the release

Monitor the workflow at: https://github.com/eigerco/move-mutation-tools/actions

#### 4. Verify the draft Release

Once the workflow completes:
1. Check the draft release page: https://github.com/eigerco/move-mutation-tools/releases
2. Verify all platform artifacts are attached
3. Download the artifact for your architecture and run it on a Move project to test it's functionality

If there's anything you want to edit on the release(or delete it), you have the ability do it, because the release is still a draft.

#### 5. Publish the release (discard draft)

Go to the releases page, click on the edit button, check "Set as the latest release" and then click on "Publish release".

#### 5. Verify the published Release
1. Install the fresh release on your machine via the Aptos CLI.
   ```bash
   aptos update move-mutation-test --target-version v2.0.0
   move-mutation-test --version
   ```
2. Run it on a Move project to test it's functionality
