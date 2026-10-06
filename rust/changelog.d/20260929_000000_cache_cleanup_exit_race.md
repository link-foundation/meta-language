---
bump: patch
---

### Fixed
- The cache cleanup no longer keeps every Cargo target because of a `rustc` that exits while the cleanup lists processes. Such a process has already released its working directory, which Linux reports as `ENOENT`, and was taken for a live Cargo process of unknown workspace; exiting processes and zombies are now dropped, while a process whose directory is unreadable for another reason still keeps the targets.
