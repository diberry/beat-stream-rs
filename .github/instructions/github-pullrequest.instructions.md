---
applyTo: github_pull_request
---

# Pull Request Guidelines

## Title Format

Use the same format as commit messages: `<type>(<scope>): <subject>`

## Description Template

Every PR description should include:

1. **What** — Brief summary of changes
2. **Why** — Motivation or issue reference
3. **How** — Key implementation details
4. **Testing** — How this was verified

## Checklist

Before marking ready for review:

- [ ] `cargo fmt --all` passes
- [ ] `cargo clippy --all-features --workspace -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] New public APIs have `///` documentation
- [ ] No `.unwrap()` in production code paths
- [ ] WebSocket message format matches `crates/shared/src/lib.rs`

## Size Guidelines

- Prefer small, focused PRs (<400 lines changed)
- Split large features into infrastructure → implementation → tests
- Frontend and backend changes may be separate PRs if they can land independently

## Review Process

- All PRs require at least one approval
- CI must pass before merge
- Use squash merge for feature branches
