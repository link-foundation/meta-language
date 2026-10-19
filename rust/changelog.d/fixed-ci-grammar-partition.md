Grammar corpus, recovery and scaling checks now run in separate CI jobs so all
checks can finish within the existing limits. Every JavaScript test file remains
assigned to exactly one required group, including the new typed-program link
reference and pattern-binding checks.
