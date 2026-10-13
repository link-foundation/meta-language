- Add an explicit `immutable fresh scalar vector results 1` Rust source profile
  for functions whose returning tails construct fresh owned vectors of direct
  scalar Copy values. The typed-links creation APIs preserve the selected
  profile and caller provenance through serialization and source-free emission.
  Legacy admission and its vector-result refusal remain available; local vector
  moves, by-value vector calls, borrowed results and named result forwarding
  still require separate source proofs.
