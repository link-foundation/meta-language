### Changed

- Added shared Links Notation programs for reference allocation, reads, writes,
  identity, array mutation and Map/Set mutation. The JavaScript reference heap
  executes their linked instructions and serializes aliases, cycles, sparse
  arrays and property descriptors. Rust receives the same operation data;
  generated Rust execution and cross-runtime comparator effects still require
  implementation and CI evidence.
