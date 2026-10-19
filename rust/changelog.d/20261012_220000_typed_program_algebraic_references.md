Add native typed-program Links adapters for owned data type and constructor
references and inspectable match binding records. Reload uses the existing
pattern checker, preserves lexical scope and constructor ownership, and refuses
unsupported fields instead of discarding them.
