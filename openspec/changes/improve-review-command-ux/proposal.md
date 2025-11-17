# Change: Improve Review Command UX

## Why
The current review command output uses emoji-based icons and a traditional list format that doesn't provide clear visual hierarchy or progress indication. Users want a more polished CLI experience with:
- Better visual structure using box-drawing characters
- Clear progress indication with animated loaders for in-progress operations
- Consistent terminology ("diffs" instead of "branches")

This improves the overall user experience and makes the tool feel more professional and modern.

## What Changes
- Replace emoji-based icons (🔄, 📦, 🚀, ✅) with box-drawing characters (┌, │, ◇, ◆, └)
- Change terminology from "branch(es)" to "diff(s)" in output
- Add animated spinner/loader for in-progress operations
- Simplify final summary message to "All done!" with appropriate box-drawing closure
- Restructure output to show clear visual hierarchy with connecting lines

## Impact
- Affected specs: cli-commands
- Affected code: src/lib.rs (commands::review_stack and create_stack_prs functions)
- No breaking changes to functionality, only output formatting
- Improves user experience with modern CLI aesthetics
