---
applyTo: "CHANGELOG.md"
---

# Changelog Guidelines

## Format

Follow [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format.

## Sections

- **Added** — New features
- **Changed** — Changes in existing functionality
- **Fixed** — Bug fixes
- **Removed** — Removed features
- **Security** — Vulnerability fixes
- **Infrastructure** — Azure/deployment changes

## Rules

- Group by release version (use `## [Unreleased]` for in-progress work)
- Most recent version first
- Each entry is one line, starts with a verb in past tense
- Reference PR numbers: `(#N)`
- User-facing language — describe the impact, not the implementation

## Example

```markdown
## [Unreleased]

### Added
- WebSocket rate limiting at 20 msg/s per client (#5)
- 6 starter beat patterns (four-on-floor, breakbeat, etc.) (#5)
- Real-time beat grid UI with Tone.js audio (#4)

### Fixed
- Deadlock on concurrent room join (#5)
- Playhead drift after BPM change (#4)
```
