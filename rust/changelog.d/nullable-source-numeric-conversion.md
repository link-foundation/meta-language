Draft lowering of checked nullable numeric operands through shared ToNumber.
Retain null/undefined distinctions, Boolean conversion and signed zero across
arithmetic, ordering, bitwise operations and negation. Independent source and
native trace controls remain deferred.
