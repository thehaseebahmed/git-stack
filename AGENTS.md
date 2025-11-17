<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# AGENTS.md

## Build/Lint/Test Commands
- Build: `cargo build`
- Release build: `cargo build --release`
- Run: `cargo run`
- Test all: `cargo test`
- Test single: `cargo test <test_name>`
- Lint: `cargo clippy`
- Format: `cargo fmt`

## Code Style Guidelines
- **Imports**: Use `use` statements at the top, group by crate then std
- **Formatting**: Use `cargo fmt` for consistent formatting
- **Types**: Use explicit types for public APIs, let inference handle locals
- **Naming**: snake_case for functions/variables, PascalCase for types
- **Error handling**: Use `Result<T, E>` with `?` operator, avoid unwrap()
- **Documentation**: Document public APIs with `///` comments
- **Testing**: Write unit tests with `#[test]`, integration tests in `tests/`

## Documentation Requirements
- **Always update README.md** when implementing user-facing changes:
  - Update command examples if CLI output format changes
  - Add/update feature descriptions for new functionality
  - Update installation/setup instructions if dependencies change
  - Ensure code examples match actual behavior
- **Include documentation tasks** in `tasks.md` for every change proposal
- **Review documentation** before marking implementation complete
- **Common files to check**: README.md, inline help text, code comments