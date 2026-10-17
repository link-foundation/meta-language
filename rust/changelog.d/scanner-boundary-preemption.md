Scanner lookahead retains a longer parse that already shifted the same virtual
boundary token. Later virtual tokens keep their existing continuation behavior,
and competing lexer continuations that bypass the boundary remain preempted.
Repeated Ruby root statements retain their exact concrete syntax trees within
the existing parser budgets.
