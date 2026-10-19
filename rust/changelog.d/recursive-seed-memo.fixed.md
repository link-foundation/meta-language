Large JavaScript native-grammar parses can reuse completed recursive calls while
their exact bounded seed dependencies remain valid, avoiding repeated work under
the existing memory, memo, step and nesting limits. Seed changes invalidate those
entries without releasing mandatory memory charges. Prefix lookups also reuse
one physical page location while preserving their complete logical contents.
Conservative trivia prefix checks avoid entering impossible named extras only
when anonymous padding and scanner expectations prove the check applicable.
Complete generated-source resource fit and native parity remain CI obligations.
