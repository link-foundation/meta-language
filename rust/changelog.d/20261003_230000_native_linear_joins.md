---
bump: patch
---

### Fixed
- The native grammar executor builds the children of a long repetition in linear time and memory. Joining results copied every child before the join, so a repetition of n items cost O(n²): a 737 KB JSON document ran out of memory. A join now links its two parts with their child count, the list is flattened once when it is first read, and a rule node shares the unflattened list. The JavaScript and Rust executors match, and a 10,000-item repetition repaired near its end is tested in both.
