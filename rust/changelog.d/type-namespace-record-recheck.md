Selected Rust type namespace archives now pass the shared whole-program checker
using paths anchored to their actual declarations. Owned flags, source spans,
empty import contexts and explicitly undefined fields are checked before
projection can normalize them. Genuine optional absence remains inspectable
as the `undefined` absence policy.
