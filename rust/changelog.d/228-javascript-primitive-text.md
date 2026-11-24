Draft registered scalar text conversion and checked String/toString lowering
through existing Number and integer formatting definitions. Preserve UTF16,
optional absence, signed-zero formatting and single operand evaluation.
Implicit Symbol text conversion requests a real source-realm TypeError.

Add deferred independent decimal-boundary, BigInt, Boolean, absence and heap
effect controls. Object conversion, explicit Symbol descriptions and console
formatting remain separate requirements; behavior has not yet been verified.
