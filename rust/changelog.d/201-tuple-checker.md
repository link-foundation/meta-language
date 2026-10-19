Add positional product construction and constant projections to the native checked IR. Preserve the explicit frozen product value model, reject unsupported element types and unsafe indices, and retain ordinary contextual array tuples.

Keep the selected immutable product model separate from its frozen flag, so a contradictory flag edit is rejected without changing legacy tuple behavior.

Refuse whole-tuple equality until an explicit comparison model preserves the difference between Rust value equality and JavaScript reference identity; positional scalar comparisons remain supported.
