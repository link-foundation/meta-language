# JSON Schema decoder conformance

The experiment emits both decoder formats from the schemas and expected results
in `parity/fixtures/json-schema-and-recovery.lino`. It checks acceptance with a
byte vocabulary and the provider grammar engines. It requires no model weights.

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
