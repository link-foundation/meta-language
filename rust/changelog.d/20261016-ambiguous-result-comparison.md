Skip repeated syntax-tree equality comparisons after a native grammar result is
already marked ambiguous, matching the existing JavaScript executor. Result
ordering, ambiguity flags, parser steps and resource limits are unchanged.
