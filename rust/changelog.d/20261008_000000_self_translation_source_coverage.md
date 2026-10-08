---
bump: patch
---

The self-translation report now accounts for every UTF-8 source byte, publishes
byte counts by item status and layout, and refuses modules with omitted code or
invalid item ranges instead of reporting empty translations as zero gaps.
