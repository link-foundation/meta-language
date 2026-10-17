---
bump: minor
---

Add a descriptor driven indentation scanner family that generates executable
Links Notation shared by both runtimes. Preserve nested blocks, tabs, reset
characters, blank lines, comments, end of input and bounded indentation counts.
Keep the nearest actual lexed continuation when virtual scanner tokens
intervene, so a pending dedent closes a block before later ordinary tokens.
