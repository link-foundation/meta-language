Draft shared source compilation for receiver and actual-argument reads, throws,
and lexical catch bindings. Handler removal preserves nested catch/rethrow
behavior; recurrence through an active handler explicitly refuses.

Add independent original-source controls and native compiler/complete-machine
trace comparisons for later execution. JavaScript frontend try/catch/finally
support remains unfinished; no behavioral verification is claimed.
