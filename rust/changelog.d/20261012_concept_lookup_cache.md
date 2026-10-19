Reduce repeated native grammar parsing and concept-register scans when looking
up shared concepts. Shipped immutable concept records use derived source-name
indexes, and native grammar rule summaries are cached lazily. Returned collections
remain independent, custom record mutations remain visible, and JavaScript
invalidates its rule summaries when grammar text changes. Concept identities,
ambiguities, distinctions and equivalence requirements are unchanged.
