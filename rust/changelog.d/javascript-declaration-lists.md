Support ordered JavaScript constant declaration lists, including mixed array
and object bindings, with one source evaluation and preserved temporal dead
zones. Preserve continuations after partially returning nested branches.

Translate discarded zero-argument pops on proved private arrays into immutable
slice rebinding. Aliases, observed results and unproved writes retain explicit
translation obligations. Native execution and runtime parity await CI.

Generate binary framing from shared field buffers and exact length counters,
retaining archive bytes, error ordering and limits while avoiding accumulated
prefix copies. Complete native round trips and timing await CI.
