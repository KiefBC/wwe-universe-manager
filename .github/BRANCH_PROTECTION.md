# Branch Protection Setup Guide

This guide explains how to set up GitHub branch protection rules to ensure all PRs pass tests before merging.

## Overview

WWE Universe Manager uses GitHub Actions workflows to automatically test all code changes. Branch protection rules enforce that these tests must pass before any code can be merged into protected branches.

## Required Branch Protection Rules

### Main Branch (`main`)

1. **Navigate to Repository Settings**
   - Go to your repository on GitHub
   - Click **Settings** tab
   - Click **Branches** in the left sidebar

2. **Add Branch Protection Rule**
   - Click **Add rule** button
   - Set **Branch name pattern**: `main`

3. **Configure Protection Settings**
   
   ✅ **Require a pull request before merging**
   - ✅ Require approvals: `1` (or more for team projects)
   - ✅ Dismiss stale PR approvals when new commits are pushed
   - ✅ Require review from code owners (if you have a CODEOWNERS file)

   ✅ **Require status checks to pass before merging**
   - ✅ Require branches to be up to date before merging
   - ✅ **Required status checks** (add these):
     - `test / Test Suite`
     - `security / Security Audit`  
     - `coverage / Code Coverage`
     - `docs / Documentation`
     - `build-matrix / Build on ubuntu-latest`
     - `dependency-check / Dependency Check`
     - `pr-ready / PR Ready for Review` (from pr-checks.yml)

   ✅ **Require conversation resolution before merging**

   ✅ **Require signed commits** (recommended for security)

   ✅ **Require linear history** (optional, keeps history clean)

   ✅ **Include administrators** (applies rules to repository admins too)

   ❌ **Allow force pushes** (keep disabled for protection)

   ❌ **Allow deletions** (keep disabled for protection)

4. **Click "Create"** to save the rule

### Development Branch (`development`)

Follow the same steps as above, but with these differences:

1. Set **Branch name pattern**: `development`
2. **Require approvals**: `0` or `1` (can be less strict than main)
3. Use the same **Required status checks** as main branch

## Workflow Integration

The branch protection works with these GitHub Actions workflows:

### 1. Main CI Workflow (`.github/workflows/ci.yml`)
- **Triggers**: Push to `main`/`development`, PRs to these branches
- **Jobs**: Complete test suite, security audit, coverage, documentation, multi-platform builds
- **Status Check Names**: 
  - `test / Test Suite`
  - `security / Security Audit`
  - `coverage / Code Coverage`
  - `docs / Documentation`
  - `build-matrix / Build on ubuntu-latest`
  - `dependency-check / Dependency Check`

### 2. PR Checks Workflow (`.github/workflows/pr-checks.yml`)
- **Triggers**: PRs to `main`/`development`
- **Jobs**: Fast pre-flight checks, quick tests, security, docs
- **Status Check Names**:
  - `pre-checks / Pre-flight Checks`
  - `quick-tests / Quick Test Suite`
  - `security-check / Security & Dependencies`
  - `docs-check / Documentation Check`
  - `pr-ready / PR Ready for Review`

### 3. Release Workflow (`.github/workflows/release.yml`)
- **Triggers**: Git tags matching `v*.*.*`
- **Purpose**: Builds and publishes releases
- **Not used for branch protection**

## Verification

To verify your branch protection is working:

1. **Create a test PR** with failing tests:
   ```rust
   #[test]
   fn test_should_fail() {
       assert_eq!(1, 2); // This will fail
   }
   ```

2. **Check PR status**:
   - PR should show failing status checks
   - Merge button should be disabled
   - Error message should indicate which checks failed

3. **Fix the test** and push:
   ```rust
   #[test]
   fn test_should_pass() {
       assert_eq!(1, 1); // This will pass
   }
   ```

4. **Verify protection works**:
   - Status checks should now pass
   - Merge button should become available
   - PR can be merged successfully

## Common Issues and Solutions

### Issue: Status checks not appearing
**Solution**: Ensure the workflow has run at least once on the default branch.

### Issue: Required status checks missing
**Solution**: Check that workflow job names exactly match the protection rules.

### Issue: Checks pass but merge still blocked
**Solution**: Verify all required checks are listed and spelled correctly in branch protection settings.

### Issue: Administrators can bypass rules
**Solution**: Ensure "Include administrators" is checked in branch protection settings.

## Workflow Status Check Names Reference

Copy these exact names when setting up required status checks:

```
Main CI Workflow (ci.yml):
- test / Test Suite
- security / Security Audit
- coverage / Code Coverage
- docs / Documentation
- build-matrix / Build on ubuntu-latest
- build-matrix / Build on windows-latest
- build-matrix / Build on macos-latest
- dependency-check / Dependency Check

PR Checks Workflow (pr-checks.yml):
- pre-checks / Pre-flight Checks
- quick-tests / Quick Test Suite
- security-check / Security & Dependencies
- docs-check / Documentation Check
- pr-ready / PR Ready for Review
```

**Recommended Minimal Set** (for faster PR feedback):
```
- pr-ready / PR Ready for Review
- test / Test Suite
- security / Security Audit
```

## Advanced Configuration

### Auto-merge for dependabot PRs
Add this to your branch protection if using Dependabot:

```yaml
# .github/workflows/dependabot-auto-merge.yml
name: Dependabot auto-merge
on: pull_request

permissions:
  contents: write
  pull-requests: write

jobs:
  dependabot:
    runs-on: ubuntu-latest
    if: ${{ github.actor == 'dependabot[bot]' }}
    steps:
      - name: Dependabot metadata
        id: metadata
        uses: dependabot/fetch-metadata@v1
        with:
          github-token: "${{ secrets.GITHUB_TOKEN }}"
      - name: Auto-merge Dependabot PRs
        if: ${{steps.metadata.outputs.update-type == 'version-update:semver-patch'}}
        run: gh pr merge --auto --merge "$PR_URL"
        env:
          PR_URL: ${{github.event.pull_request.html_url}}
          GITHUB_TOKEN: ${{secrets.GITHUB_TOKEN}}
```

### Repository Rulesets (Beta)
GitHub is moving toward "Rulesets" for more advanced protection. These can be configured at **Settings > Rules > Rulesets** for more granular control.

## Summary

With these branch protection rules in place:

✅ **No code can be merged without passing tests**
✅ **All PRs require review before merging**  
✅ **Security audits run automatically**
✅ **Code coverage is tracked and enforced**
✅ **Documentation stays up to date**
✅ **Multi-platform compatibility is verified**

This ensures the highest code quality and prevents breaking changes from reaching your main branches.