---
bump: minor
---

### Added
- Shared-dialog source-description schema under `docs/schemas/shared-dialog/`:
  a cross-repository definition of replayable shared AI dialogs (ChatGPT static
  HTML, Google AI Mode capture and challenge, and Markdown transcripts) with a
  JSON Schema, equivalent meta-language (LiNo) examples, and a worked formal-ai
  `demo_memory` mapping. New tests validate every JSON example against the
  schema, prove the LiNo examples round-trip losslessly, and assert the
  `demo_memory` mapping preserves provider, source URL, turn role, and content.
