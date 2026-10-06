# Cache cleanup: processes that exit while they are listed

`js/tests/cache-cleanup.test.js` ("cleanup keeps a target whose built binary is
still running") failed once under `npm test` while a `cargo test` ran
alongside: the second cleanup kept `rust/target` after the fixture binary had
exited.

The Linux process reader read `/proc/<pid>/cwd` of every process and treated a
Cargo-family process with an unreadable working directory as one that "may
use" every target. A cargo build keeps starting and ending `rustc`s, and one
that exits between its `cmdline` and `cwd` reads has already released its
working directory.

- `churn.sh SECONDS` keeps short-lived processes named `rustc` running.
- `stress.mjs` lists processes for 15 s and counts `rustc` entries with an
  unreadable working directory.
- `errno.mjs` shows `readlink` of `/proc/<pid>/cwd` failing with `ENOENT` for a
  pid that is gone.

Before the fix: `rustc entries: 4637; with an unreadable cwd: 3, of which
already gone: 3`. After: `rustc entries: 19895; with an unreadable cwd: 0`.

Linux's `proc_cwd_link` returns `ENOENT` once a task has released its `fs`
(and for a pid that is gone) and `EACCES` when the reader may not look, so the
reader now drops `ENOENT` and zombies and keeps the conservative answer for
everything else. The regression test builds a fake `/proc` to cover each case.
