---
bump: patch
---

### Fixed
- Documents with many embedded regions parse in linear time again. Both runtimes recorded grammar provenance by scanning every link of the network once per region, so a Markdown file with a fenced block in every section parsed in quadratic time: at 1024 sections a release build spent 58 µs per byte instead of 1.2. Provenance is now recorded once per region language. The JavaScript runtime also stops re-encoding the whole text for every region, resolves region points with a line index instead of walking the text from its start, and builds the UTF-8 input string of the tree-sitter parse once.
- `rust/tests/unit/parse_scaling.rs` and `js/tests/parse-scaling.test.js` guard Markdown and HTML documents with a region in every unit, and fail on the old quadratic behavior.
