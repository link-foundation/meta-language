Release the original complete Rust CST before reloading its full archives, then
release each decoded graph before the next replay. Only encoded archives leave
creation. These authenticated source archives select their actual link count
through the binary codec's existing record-limit option; default limits and the
byte cap stay unchanged. CI phase logs record counts and heap use. Three tiny
JavaScript checks cover exact source replay, whole-record preservation, resource
refusals and phase ordering; full memory fit remains a CI check.
