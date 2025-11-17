# Change: Improve Sync Command UX

## Why
The current sync command output uses emoji-based icons and a traditional numbered list format that doesn't provide clear visual hierarchy or progress indication. Users want consistency with the review command's polished UX that uses:
- Better visual structure using box-drawing characters
- Clear progress indication with animated loaders for in-progress operations
- Consistent terminology ("diffs" instead of "branches")

This change brings the sync command in line with the improved review command UX, making the tool feel consistent and professional.

## What Changes
- Replace emoji-based icons (🔄, 📦, ✅) with box-drawing characters (┌, │, ◇, ◆, └)
- Remove numbered list format (1., 2., 3.)
- Change terminology from "branch(es)" to "diff(s)" in output where applicable
- Add animated spinner/loader for in-progress operations (fetching, syncing, rebasing)
- Simplify final summary message to "All done!" with appropriate box-drawing closure
- Restructure output to show clear visual hierarchy with connecting lines
- Use the newly introduced `MultiStepProcess` class from `src/process.rs` for consistent output

## Impact
- Affected specs: cli-commands
- Affected code: src/lib.rs (sync_stacks, sync_all_stacks, sync_current_stack functions)
- No breaking changes to functionality, only output formatting
- Improves user experience with modern CLI aesthetics and consistency
