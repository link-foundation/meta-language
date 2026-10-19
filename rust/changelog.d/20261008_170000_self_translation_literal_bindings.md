---
bump: patch
---

- Translate sibling reads of immutable primitive literal constants, respecting local shadowing and temporal dead zones.
- Generate constant eligibility, storage selection and declaration rendering rules from JavaScript for both runtimes.
- Share executable constant-capture fixtures and compare exact emitted output and lossless restoration.
