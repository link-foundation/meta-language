---
bump: minor
---

### Added
- `ParserRegistry` maps language keys to `Arc<dyn LanguageParser>` with the
  built-in parser as a fallback. User registrations shadow the built-in
  dispatch for the same (case-insensitive) key; unregistered keys still route
  through the built-in set.
- `LinkNetwork::parse_with_registry` dispatches a parse through a registry.
- `examples/custom_parser_registry.rs` documents registering a custom parser.
