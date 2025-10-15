### Contributing to WeaveFFI

This repo uses Conventional Commits for all commits. Keep it simple: we do not use scopes.

## Conventional Commits

Use the form:

```
<type>: <subject>

[optional body]

[optional footer(s)]
```

Subject rules:

- Imperative mood, no trailing period, ≤ 72 characters
- UTF‑8 allowed; avoid emoji in the subject

Accepted types:

- `build` – build system or external dependencies (e.g., package.json, tooling)
- `chore` – maintenance (no app behavior change)
- `ci` – continuous integration configuration (workflows, pipelines)
- `docs` – documentation only
- `feat` – user-facing feature or capability
- `fix` – bug fix
- `perf` – performance improvements
- `refactor` – code change that neither fixes a bug nor adds a feature
- `revert` – revert of a previous commit
- `style` – formatting/whitespace (no code behavior)
- `test` – add/adjust tests only

Examples:

```text
feat: add SwiftPM scaffolding for Swift bindings
fix: correct C string ownership in Kotlin generator
docs: document memory management and error mapping
style: format generated TypeScript definitions
chore: update Gradle wrapper and Android build scripts
ci: add workflow to build WASM and publish npm package
perf: speed up header parser for large C APIs
refactor: extract template engine from codegen core
test: add fixtures for async callback-to-Promise mapping
revert: revert "perf: speed up header parser for large C APIs"
```

Breaking changes:

- Use `!` after the type or a `BREAKING CHANGE:` footer.

```text
feat!: switch JS generator from callbacks to Promises

BREAKING CHANGE: JS bindings now return Promises instead of using callbacks; update call sites.
```
