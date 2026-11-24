# Scalar vector indexing beside readonly source regions

The checked signature3 region visitor now admits direct scalar indexing of an
actual standard Vec, including a shared Vec parameter. It preserves the existing
canonical element owner and direct scalar Copy domain. Nominal, String, boxed,
nested borrowed carriers and mutable element capabilities are not smuggled into
this domain. Historical source scalar-index operand checks still require the
actual usize owner and preserve exact conversion and checked bounds behavior.

Copying a scalar produces a scalar without a loan of the vector allocation.
Taking a single shared reference instead keeps the actual owned vector storage
root or the existing borrowed parameter regions. It never manufactures static
lifetime or borrows a parameter's local reference binder in place of its actual
referent region. Referent positions remain separate from proof storage epochs.

Five additional literal originals cover ordinary scalar/Boolean values, moving a
vector after copying its element, borrowing an element through a lifetime-bound
Vec parameter, and copying through a shared parameter before moving the vector.
Four extra refusal inputs cover equal-width u64 versus usize, moving non-Copy
String elements, forged static lifetime, and a saved borrow after vector move.
All eight earlier mixed positives and seven earlier refusals remain unchanged.
The group now declares thirteen positives and eleven refusals.

The existing CI-only original/product driver imports the complete positive list
without another runner. Its total inventory is now 356 authored positive rows in
eighteen groups: 292 literal originals and 64 actual-source scenarios. This is
not observed evidence, requirement completion or a claim of full module support.
Neither tests, source generation, validation gates nor compilation ran locally.

The source position graph scanner also enqueues field and standard-sum argument
edges iteratively, avoiding host variadic argument limits before its existing
finite graph inspection bound. This changes no represented language semantics.
