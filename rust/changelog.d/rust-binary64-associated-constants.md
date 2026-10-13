Admit exactly `f64::NAN`, `f64::INFINITY`, and `f64::NEG_INFINITY` when their
owner resolves to the unshadowed primitive type. The constants preserve binary64
source evidence for existing numeric method chains and checked algorithm
definition round trips. Custom, qualified, shadowed and `f32` owners retain
their existing refusals; NaN payload and sign are outside this value model.
