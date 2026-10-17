Rust file consumers can resolve logical project-data paths to Links Notation,
read the exact physical bytes for hashing, and deserialize through the shared
structured-data codec. Conflicting JSON/LiNo storage is rejected; explicitly
requested external JSON retains its original file boundary. Native unit
fixture readers use the same path resolver.
