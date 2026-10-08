---
bump: patch
---

Preserve CRLF and mixed line endings, Unicode separators and leading or trailing
layout when an unedited self-translation is restored. Verify both original
source bytes and the complete emitted body before restoring the source envelope;
edited output and corrupt provenance cannot restore a stale source.
Generate the shared restoration decision into Rust and check both runtimes
against six shared sources and changed-body and damaged-envelope cases.
