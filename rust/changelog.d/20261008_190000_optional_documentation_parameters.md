### Fixed

- Read bracketed optional JSDoc parameter names in both runtimes while preserving declared types and actual function defaults. Documentation defaults remain metadata. Shared fixtures check execution and exact source restoration.
- Anchor generated Rust style replacements to their complete named function so adding another frontend decision cannot move a closing branch into the wrong function.
