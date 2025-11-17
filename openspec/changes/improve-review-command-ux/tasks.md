# Implementation Tasks

## 1. Add Progress Indicator Library
- [x] 1.1 Add indicatif crate to Cargo.toml for spinner/progress support
- [x] 1.2 Configure spinner styles for in-progress operations

## 2. Update Review Command Output Format
- [x] 2.1 Replace initial emoji "🔄" with "┌" box-drawing character
- [x] 2.2 Change "branch(es)" terminology to "diff(s)"
- [x] 2.3 Replace "📦 Found X branch(es)" with "◇  Found X diff(s) in stack"
- [x] 2.4 Update "🔍 Checking for existing pull requests" section format
- [x] 2.5 Replace "🚀 Creating missing pull requests..." with "◆  Created missing pull requests" header
- [x] 2.6 Add spinner for operations in progress (checking PRs, creating PRs)
- [x] 2.7 Replace "✅ Review summary:" with "└  All done!"
- [x] 2.8 Remove detailed summary listing (keep it simple)
- [x] 2.9 Add vertical line connectors (│) between sections
- [x] 2.10 Ensure proper indentation for nested items

## 3. Implement Progress Indicators
- [x] 3.1 Add spinner for "Checking for existing pull requests" operation
- [x] 3.2 Add spinner for "Creating missing pull requests" operation
- [x] 3.3 Ensure spinners stop and show final static icon when complete
- [x] 3.4 Use appropriate spinner style (e.g., dots, line)

## 4. Testing
- [x] 4.1 Test output format with stack that has no PRs
- [x] 4.2 Test output format with stack that has some PRs
- [x] 4.3 Test output format with stack that has all PRs
- [x] 4.4 Verify spinner animations work correctly
- [x] 4.5 Test edge cases (single branch stack, empty stack)

## 5. Update Tests
- [x] 5.1 Update integration tests to match new output format
- [x] 5.2 Ensure existing functionality tests still pass
