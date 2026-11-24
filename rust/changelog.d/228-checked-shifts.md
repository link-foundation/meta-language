Draft standard checked left/right shifts with independent u32 count ownership,
all five immediate Option consumers, count-overflow guards and original eager
operand/default order. Reuse existing shift semantics for valid counts,
including discarded high bits and signed right extension. The 1,995 original
source programs and both-runtime product/parity controls remain unexecuted.
