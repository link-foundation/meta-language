# JSON Schema decoder conformance

The experiment emits both decoder formats from the schemas and expected results
in `parity/fixtures/json-schema-and-recovery.lino` and the shared exclusive array
and object fixtures. It checks acceptance with a byte vocabulary and the
provider grammar engines. It requires no model weights.

Run from the repository root with Node and Python 3.12. The example uses a
virtual environment and puts temporary data in a separate directory.

```sh
decoder_scratch=$(mktemp -d)
python3.12 -m venv "$decoder_scratch/venv"
decoder_python="$decoder_scratch/venv/bin/python"
"$decoder_python" -m pip install llguidance==1.9.1 outlines==1.3.3 numpy==2.4.6 apache-tvm-ffi==0.1.12
"$decoder_python" -m pip install --no-deps xgrammar==0.2.9
node experiments/issue-245/json-schema-decoders.mjs > "$decoder_scratch/fixtures.json"
export OPENBLAS_NUM_THREADS=1
ulimit -Sv 550000
ulimit -s 8192
"$decoder_python" experiments/issue-245/json-schema-decoders.py llguidance "$decoder_scratch/fixtures.json"
"$decoder_python" experiments/issue-245/json-schema-decoders.py outlines "$decoder_scratch/fixtures.json"
xgrammar_package=$("$decoder_python" -c 'from importlib.metadata import distribution; print(distribution("xgrammar").locate_file("xgrammar"))')
"$decoder_python" experiments/issue-245/json-schema-decoders.py xgrammar "$decoder_scratch/fixtures.json" --xgrammar-package "$xgrammar_package"
```

For llama.cpp use the `llama-gbnf-validator` executable from the `b12633` source
tree, or link its `tests/test-gbnf-validator.cpp` to the prebuilt release's
`libllama` and `libggml-base`. Pass that executable's path as
`--llama-validator` with `llama.cpp` as the provider. The validator reads the
exact emitted GBNF and the unmodified fixture bytes.
[Pinned llama.cpp validator](https://github.com/ggml-org/llama.cpp/blob/b12633/tests/test-gbnf-validator.cpp)

Run `node experiments/issue-245/json-schema-linked-alternatives.mjs` to check
the array reference with `items: false` beside `$ref`. Before the correction,
the decoder accepted `[true]` while document validation rejected it. The
`items` keyword uses the prefix declared in the same schema object, which is
empty in this case. The fixture retains all four inputs and checks the
diagnostic for an item intersection that contains another alternative.
[JSON Schema Core, draft 2020-12, section 10.3.1.2](https://json-schema.org/draft/2020-12/json-schema-core#section-10.3.1.2)

The same experiment checks a reference to `{ minItems: 1 }` with a sibling
`maxItems: 2`. Before the change, the decoder accepted `[1,2,3]` while
validation rejected it. The grammar now combines both bounds for arrays and
still admits other JSON types. Additional fixtures exercise type-scoped
assertions across mixed-type references and inclusive and exclusive branches.

Object reference and alternative controls exercise properties supplied by
different schema objects. Before the change, the decoder accepted
`{"active":1,"kind":"record"}` even though the reference sibling requires
a Boolean `active` property. The fixture checks both property orders, escaped
names, additional properties, and impossible requirements under a closed
layout. A property with distinct regular patterns in two layers enforces both
patterns. Before the correction, a property with patterns `^a` and `z$`
admitted `{"value":"ax"}`. The checks cover direct and escaped spellings,
property references, and constraints on additional property values.

The array fixture with a string prefix beside a reference to a Boolean prefix
previously admitted `[true]` and `[false,0]`. The decoder now rejects both and
continues to admit `[]`. New cases combine compatible schemas at each array
position through local references and alternatives, apply the appropriate tail
to each position, and ignore positions beyond an existing length bound or a
forbidden item. A property with disjoint declared types is also forbidden,
including when its name is escaped in the input.

The scalar item intersections combine the patterns `^a` and `z$` and numeric
bounds from both sides of a reference. An untyped string constraint still
allows a nonstring item. The bounded integer example admits `[false,2,4]` and
rejects `[true,1]` and `[true,5]`. Document validation also admits equivalent
decimal spellings for integer values.

For a required `flag` property, a local reference uses `oneOf` branches that
accept `false` twice. The old decoder admitted `{"flag":false}` from the
property's enum. The constrained grammar now retains `true` alone. Further
cases check numeric bounds on enumerated array items, member permutations for
enumerated objects, and a required property whose constants contradict each
other. The bounded array enumeration keeps its `/items/enum` diagnostic,
and the decoder and validator agree on all eight original documents.

The nested collection cases combine required fields of objects in an array and
intersect item types with length bounds in an object property. Before the change,
the decoder admitted `[{"active":true}]` despite a nested required `label`.
The fixture also checks an impossible required array and diagnostic fallback
when mutually recursive layouts reach the shared construction bound.

The exclusive array cases distinguish a closed prefix of length one from a
reference chain that requires at least two elements. Both branches apply the
sibling Boolean item schema. The decoder previously admitted `["x"]` through
the short branch. The fixture also checks an independent `null` branch.

Run `node experiments/issue-245/json-schema-exclusive-array-lengths.mjs` to
check two string array branches whose length intervals meet at three. The
decoder previously admitted a three-item array even though both branches
accept it. The array with a common Boolean item schema retains all five
original inputs and their validator expectations. Its decoder now rejects
`[true,false]`, which matches both length intervals, and `[null]`, which fails
the common item schema. A separate control with different item patterns
retains both original diagnostic pointers. Further cases cover three-way
overlap, linked closed prefixes, unbounded intervals, and duplicate
alternatives with no generatable document.

The same experiment checks `oneOf` branches for string arrays and number
arrays. The decoder previously admitted `[]` because either item rule permits
zero items. Both branches match it, so the decoder now omits that spelling.
The fixtures cover an empty array accepted by one branch, overlapping lengths
within a shared item layout, disjoint layouts connected by local references,
and a common item rule inherited by an unrestricted branch.

Run `node experiments/issue-245/json-schema-exclusive-object-property.mjs`
to check overlapping property enumerations in objects with an unrestricted
text field. The previous grammar accepted the shared choice. It now rejects
that choice and accepts each value covered by a single branch. The original
shared fixture likewise rejects an absent optional tag and a tag accepted by
both branches. A local reference and its alias resolve to the same object
schema, so their exclusive combination has no generatable document. Both
original documents for that schema remain in the fixture and fail validation.

Run `node experiments/issue-245/json-schema-exclusive-object-siblings.mjs`
to check the same exclusive property when the parent schema adds object rules.
The decoder previously admitted `{"source":"record","mode":"shared","evidence":"quote: text"}`
although both branches matched. The fixture checks the parent pattern and
required fields, escaped names and values, and the additional-property schema
when the parent leaves the exclusive property undeclared. It also checks when
the parent excludes every value accepted by exactly one branch.

The required prefix cases share a length range and distinguish branches by
their first string value. One minimum length comes from a sibling of the
local reference. The decoder previously admitted `["selected",null]` and now
applies the common Boolean tail rule. A control with optional prefix positions
retains the exclusivity diagnostic because both branches match the empty array.

The decimal cases use overlapping `oneOf` intervals with limits from linked
schemas and their containing schema. Before the change, the decoder admitted
`-0.7501`, outside the common minimum, and `2.5`, inside both branches. The
shared fixture also checks a single accepted endpoint, an excluded zero,
adjacent intervals covering every number, and two identical branches with
no generatable number. Document validation checks equivalent exponent
spellings where the decoder reports a spelling restriction.

A mixed integer and number alternative previously admitted `-1.00` even
though both linked branches accept its value. The fixture also checks
`-2.00`, which only the integer branch accepts, and fractional values that
only the number branch accepts. Separate controls exercise unbounded
alternatives, signed zero, and a mixed alternative with no generatable value.

Adding an independent `null` alternative previously admitted `0` and `-1.00`
through overlapping number and integer branches. The importer now checks
exclusivity within each applicable JSON instance type. Another fixture exercises
two overlapping string branches, two overlapping numeric branches, a Boolean
branch, and sibling constraints for the string and numeric instances.

Combining a numeric range with an enum previously rejected `-1.00` when the
enum alone admitted its value. The fixture now checks that spelling and rejects
`2.500` because both branches admit it. A second fixture checks finite values
from a local reference, removes a value outside a sibling bound, and counts a
value repeated within the same `anyOf` branch once.

In `{ type: 'boolean', oneOf: [true, { const: true }] }`, the decoder
previously admitted `true` through both branches. The fixture now accepts
only `false`, including when an independent string branch remains available.
Related fixtures check a local reference with simultaneous `anyOf` and
`oneOf` siblings, and a null value that matches both branches of its schema.
