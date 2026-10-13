- Admit zero-argument `abs`, `floor`, `ceil` and `trunc` methods on proved direct
  primitive `f64` receivers through shared linked expression templates. Results
  retain the actual source receiver proof for chaining and evaluate the receiver
  once. Unary negation now preserves an already proved primitive `f64` origin,
  including canonical emitted `-0f64` bindings. Explicit `.len() as f64`
  expressions now retain the same primitive receiver proof in both runtimes. Unproved, indirected, custom,
  shadowed and `f32` receivers, and separate rounding, sign and transcendental
  models remain explicit source boundaries.
