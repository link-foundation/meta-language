import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { asciiHostPin, replayAsciiHostEdits } from './ascii-text-predicate-host-history.mjs';

const history = new URL('../../parity/self-translation/registered-native-test-source-history.lino', import.meta.url);
const historicalRevision = '7881e1e6f83818e0b25b77e1b3ab15c2d51ca9f3';
const historySha256 = 'c1def0fbbbaa6425f2759c068bf4e62a813da7ae4763b0bab3227ba38256a3a7';
const asciiTestPath = 'rust/tests/unit/translation_ascii_text_predicates.rs';
const asciiTestCurrent = { bytes: 20807, sha256: '3d8386366b15b8236b884a407874d6fa6f6651b2e5b4216af84f386d78dd6d63' };
const asciiTestPredecessor = { bytes: 19997, sha256: 'c2e9b6c14e3f02a44cf5d82d32368b879b5d2d17fe473dc288a85ecd54bdd2a2' };
const asciiTestChanges = [
  {
    before: "        let expected = expected\n            .iter()\n            .map(|point| format!(\"vec!{point:?}\"))\n            .collect::<Vec<_>>()\n            .join(\", \");\n        write!(harness, \"let actual=points(crate::ml_string::Text::from_units(vec!{units:?})).into_iter().map(|point|point.0).collect::<Vec<_>>();let expected:Vec<Vec<u16>>=vec![{expected}];assert_eq!(actual,expected);\").unwrap();\n",
    after: "        write!(harness, \"let actual=points(crate::ml_string::Text::from_units(vec!{units:?})).into_iter().map(|point|point.0).collect::<Vec<_>>();let expected:Vec<Vec<u16>>=vec!{expected:?};assert_eq!(actual,expected);\").unwrap();\n",
  },
  {
    before: "        observe_rust_pattern_with_primitive_parameter(&source, \"true\\nfalse\\n\");\n",
    after: "        observe(&source, \"true\\nfalse\\n\", true);\n",
  },
  {
    before: "    observe_rust_pattern_with_primitive_parameter(&source, expected);\n",
    after: "    observe(&source, expected, true);\n",
  },
  {
    before: "    assert_eq!(result.stdout, expected);\n}\n\nfn observe_rust_pattern_with_primitive_parameter(source: &str, expected: &str) {\n    // Rust 1.99 reports E0282 for the unannotated Pattern callback these\n    // translation fixtures recognize. Keep its exact spelling for translation\n    // and give the independent Rust driver the primitive, even in a shadowing scope.\n    let callback = \".starts_with(|c|\";\n    assert_eq!(source.matches(callback).count(), 1);\n    let driver = source.replace(callback, \".starts_with(|c: ::core::primitive::char|\");\n    observe(&driver, expected, true);\n",
    after: "    assert_eq!(result.stdout, expected);\n",
  },
  {
    before: "        let mut command = versioned_fixture_tool(\"rustc\");\n",
    after: "        let mut command = Command::new(\"rustc\");\n",
  },
  {
    before: "        let rust = run(versioned_fixture_tool(\"rustc\").arg(\"--version\"));\n",
    after: "        let rust = run(Command::new(\"rustc\").arg(\"--version\"));\n",
  },
  {
    before: "use super::issue_195_translation_pairs::{TemporaryDirectory, run, versioned_fixture_tool};\n",
    after: "use super::issue_195_translation_pairs::{TemporaryDirectory, run};\n",
  },
];
let sources;

/** Recover the exact registered predecessor for a historical source check. */
export function restoreRegisteredNativeTestSource(source, path) {
  if (path === asciiTestPath && asciiHostPin(source).sha256 === asciiTestCurrent.sha256) {
    assert.deepEqual(asciiHostPin(source), asciiTestCurrent);
    source = replayAsciiHostEdits(source, asciiTestChanges).source;
    assert.deepEqual(asciiHostPin(source), asciiTestPredecessor);
  }
  if (!sources) {
    const bytes = readFileSync(history);
    assert.equal(asciiHostPin(bytes).sha256, historySha256);
    const packet = readBootstrapStructuredDataLino(bytes.toString('utf8'));
    assert.equal(packet.schemaVersion, 1);
    assert.equal(packet.revision, historicalRevision);
    sources = new Map(packet.sources.map(row => [row.path, row]));
    assert.equal(sources.size, packet.sources.length);
  }
  const record = sources.get(path);
  if (!record) return source;
  const current = asciiHostPin(source);
  if (current.sha256 !== record.current.sha256) return source;
  assert.deepEqual(current, record.current);

  const replay = replayAsciiHostEdits(source, record.changes);
  assert.deepEqual(asciiHostPin(replay.source), record.previous);
  return replay.source;
}
