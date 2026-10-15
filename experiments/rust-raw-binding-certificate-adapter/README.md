# Program-owned raw binding certificate experiment

This isolated experiment applies checked LiNo decorators to five existing
JavaScript modules in a temporary directory. Production sources, declarations,
default parsing and the frozen raw-binding interpretation remain unchanged.
The project architecture remains [the shared vision](../../docs/vision.md).

```js
import { createRustRawBindingCertificateOverlay } from './runtime.mjs';
import { checkRustRawBindingCertificateInvocation } from './certificate.mjs';

const overlay = await createRustRawBindingCertificateOverlay();
try {
  const packet = overlay.api.createTypedProgramLinks(
    'pub fn r#match(r#type:i32)->i32{r#type}fn main(){}', 'Rust',
    { rustRawBindingIdentifierModel: 'Rust raw binding identifier interpretation 1' },
  );
  const program = overlay.api.decodeTypedProgramLinks(packet.network, packet.root);
  checkRustRawBindingCertificateInvocation(program, program.declarations.get('match'), [7n]);
  const emitted = overlay.api.emitTypedProgramLinks(packet.network, packet.root, 'Rust');
  console.log(emitted.code);
} finally {
  overlay.dispose();
}
```

The program owns its complete token/gap interpretation. Its actual main owns a
context pointing to the exact same source-extent object. Actual declaration,
parameter, occurrence and invocation owners retain their identities after
text and binary reloads. Either surviving root/context marker requires the
other; misplaced facts and detached equal-value objects refuse. Deleting all
associated facts does not provide evidence of their previous existence.

Rust emission reconstructs the whole source and applies checked owner edits.
Target mappings use actual emitted identifier spellings, including `r#` when
present. JavaScript emission carries explicit invocation premises; callers
must check actual declaration ownership, exact arity and Boolean/i32 argument
domains. The experiment does not add an invocation wrapper.

The candidate option exists only inside this temporary overlay. Native
`Program`/`Main` facets, full raw-binding source checking and an original-LinkId
certificate before native lowering are still absent. The four automatically
translated predicates in `data/rules.rs` do not implement that native boundary.
The separate native-observation runner checks the unchanged six-selector
raw-binding artifact; it does not establish a native source model.

From the repository root:

```sh
node experiments/rust-raw-binding-certificate-adapter/generate-rules.mjs --check
node experiments/rust-raw-binding-certificate-adapter/generate-overlay.mjs --check
TRANSLATION_CHECK_RUST=0 SELF_TRANSLATION_CHECK_RUST=0 node scripts/with-cache-cleanup.mjs --event test --no-preflight --no-docker -- node --test experiments/rust-raw-binding-certificate-adapter/model.test.mjs
```

The generators authenticate shared LiNo policy, actual JavaScript-to-Rust
output, recorded emitter styles and exact source/decorator hashes. Bounded
tests exercise the decorated public API, saved graph substitutions, coherent
source-free edits, emitted programs, unchanged defaults and zero getter reads.
Native compilation belongs to CI.
