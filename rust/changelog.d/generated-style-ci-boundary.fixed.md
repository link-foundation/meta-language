Check both automatically generated global-concept modules in the existing CI
compiler-style gate, retaining the original proof module, compiler flags and raw
feedback artifacts. Frontend and concept style commands now refuse local native
compilation before creating scratch; explicit offline feedback replay remains
available. Local guard checks invoke no Rust compiler.
