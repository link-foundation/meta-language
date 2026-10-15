Reject JavaScript link identities that cannot be represented exactly by its
existing Number API, including unsafe integers and fractional or coercing
spellings. Text readers, writers and link-cli operations validate the original
identity before conversion. Allocator exhaustion now throws without replacing
an existing link. Exact whitespace-trimmed decimal strings and ordinary IDs
retain their behavior; the existing binary safe-integer profile is unchanged.
