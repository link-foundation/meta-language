JavaScript type inference reaches a finite fixed point for mutually recursive
functions that return equal record shapes. Shared type identities retain
unresolved variable IDs, and inference preserves field, absence and argument
constraints instead of repeatedly swapping equivalent return objects.
