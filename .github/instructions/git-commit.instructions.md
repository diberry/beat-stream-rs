---
applyTo: git_commit_message
---

# Git Commit Message Guidelines

## Format

```
<type>(<scope>): <subject>

[optional body]

[optional footer]
```

## Types

- `feat`: New feature or capability
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Formatting, no code change
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvement
- `test`: Adding or updating tests
- `ci`: CI/CD changes
- `chore`: Maintenance tasks
- `infra`: Infrastructure (Bicep, Docker, deployment)

## Scopes

- `server`: Backend crate changes
- `shared`: Shared types crate
- `frontend`: Frontend assets
- `ws`: WebSocket-specific changes
- `room`: Room management
- `infra`: Azure infrastructure
- `ci`: GitHub Actions workflows
- `deps`: Dependency updates

## Rules

- Subject line: imperative mood, lowercase, no period, max 72 chars
- Body: wrap at 80 chars, explain WHY not WHAT
- Reference issues: `Closes #N` or `Relates to #N`

## Examples

```
feat(ws): add token bucket rate limiter

Implements per-client rate limiting at 20 msg/s using a token bucket
algorithm. Messages exceeding the limit are silently dropped to prevent
broadcast storms.

Closes #12
```

```
fix(room): prevent deadlock on concurrent join

Clone Arc<Room> from DashMap before acquiring RwLock to avoid holding
the DashMap read guard across an await point.
```
