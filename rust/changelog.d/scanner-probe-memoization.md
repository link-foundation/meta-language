External scanner priority and trivia probes reuse the same context and state
cache as token scans. Long HCL heredoc identifiers and repeated scanner probes
retain their original parse budgets and syntax trees.
