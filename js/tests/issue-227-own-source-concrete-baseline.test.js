// Deferred controls: authoring this file does not execute these source programs.
// This bounded pair exercises actual production files, not the full own-source
// inventory or a claim of native parsing/semantic-translation parity.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { LinkNetwork } from '../src/network.js';
import { LinkType, NetworkProjection } from '../src/primitives.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';

const controls = [
  {
    path: 'js/src/network-projection-rules.js', language: 'JavaScript',
    declarations: ['function_declaration', 'acceptNetworkProjection', 'acceptNetworkProjectionLink'],
    repeated: 'projection',
  },
  {
    path: 'rust/src/network_projection.rs', language: 'Rust',
    declarations: ['enum_item', 'NetworkProjection', 'label', 'includes'],
    repeated: 'Self',
  },
];
const root = new URL('../../', import.meta.url);
const identifiers = links => links.map(link => link.id().asU64());

function assertSelectedVersionProfile() {
  // These controls select the source-hook profiles declared in vision.md.
  // Other Node majors/minors are uncovered; they fail rather than skip cells.
  const [major, minor] = process.versions.node.split('.').map(Number);
  assert.ok(major === 24 || major === 22 && minor >= 15,
    `uncovered own-source CST control runtime: Node ${process.versions.node}`);
  const manifest = JSON.parse(readFileSync(new URL('js/package.json', root), 'utf8'));
  assert.equal(manifest.dependencies['web-tree-sitter'], '0.27.0',
    'changed oracle dependency requires a separately reviewed control profile');
  // Rust input is the repository edition-2024 source; no Rust compiler runs.
  const cargo = readFileSync(new URL('rust/Cargo.toml', root), 'utf8');
  assert.match(cargo, /^edition = "2024"$/m);
  assert.match(cargo, /^rust-version = "1\.90"$/m);
}

function assertConcreteSource(network, bytes, control) {
  assert.equal(network.verifyFullMatch().isClean(), true,
    `${control.path}: original production source has no parser recovery issues`);
  assert.deepEqual(Buffer.from(network.reconstructText(), 'utf8'), bytes,
    `${control.path}: complete original UTF-8 bytes`);
  const concrete = network.projectedLinks();
  assert.deepEqual(identifiers(concrete),
    identifiers(network.projectedLinks(NetworkProjection.ConcreteSyntax)));
  for (const role of [LinkType.Syntax, LinkType.SourceToken, LinkType.Trivia, LinkType.Field]) {
    assert.ok(concrete.some(link => link.metadata().linkType === role), `${control.path}: ${role}`);
  }
  const syntax = concrete.filter(link => link.metadata().linkType === LinkType.Syntax);
  assert.ok(syntax.some(link => link.references().some(reference =>
    network.link(reference)?.metadata().linkType === LinkType.Syntax)),
  'the actual source has nested grammar structure, beyond a root with tokens');
  for (const spelling of control.declarations) {
    assert.ok(syntax.some(link => link.metadata().term === spelling) ||
      concrete.some(link => link.metadata().linkType === LinkType.SourceToken &&
        link.metadata().term === spelling), `${control.path}: actual declaration ${spelling}`);
  }
  const tokens = concrete.filter(link => link.metadata().linkType === LinkType.SourceToken &&
    link.metadata().language === control.language && !link.metadata().flags.isMissing);
  for (const token of tokens) {
    const { span, term } = token.metadata();
    assert.ok(span, 'physical token retains its actual source interval');
    assert.equal(term, bytes.subarray(span.byteRange.start, span.byteRange.end).toString('utf8'));
  }
  const repeated = tokens.filter(link => link.metadata().term === control.repeated);
  assert.ok(repeated.length >= 2, 'retain independently occurring equal spellings');
  assert.equal(new Set(identifiers(repeated)).size, repeated.length);
  assert.equal(new Set(repeated.map(link => link.metadata().span.byteRange.start)).size,
    repeated.length, 'equal source text never collapses distinct occurrence spans');
  const before = network.toLino();
  const abstract = network.projectedLinks(NetworkProjection.AbstractSyntax);
  assert.ok(abstract.every(link => ![LinkType.SourceToken, LinkType.Trivia].includes(link.metadata().linkType)));
  assert.equal(network.toLino(), before, 'explicit abstract selection retains the full original graph');
  assert.deepEqual(identifiers(network.projectedLinks()), identifiers(concrete),
    'omitted selection remains concrete after explicit abstract selection');
}

for (const control of controls) {
  test(`deferred own-source CST and lossless transports: ${control.path}`, () => {
    assertSelectedVersionProfile();
    const bytes = readFileSync(new URL(control.path, root));
    assert.ok(bytes.length > 0 && bytes.length <= 16384, 'bounded isolated source control');
    const source = new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(bytes);
    const network = LinkNetwork.parse(source, control.language);
    assertConcreteSource(network, bytes, control);
    const text = network.toLino();
    const binary = encodeLinkNetworkBinary(network);
    let restoredText, restoredBinary;
    const parse = LinkNetwork.parse;
    try {
      LinkNetwork.parse = () => { throw new Error('snapshot reload must not invoke source parsing'); };
      restoredText = LinkNetwork.fromLino(text);
      restoredBinary = decodeLinkNetworkBinary(binary);
    } finally {
      LinkNetwork.parse = parse;
    }
    for (const restored of [restoredText, restoredBinary]) {
      assert.equal(restored.toLino(), text, 'every original identity, reference and metadata field survives');
      assertConcreteSource(restored, bytes, control);
      const reparsed = LinkNetwork.parse(restored.reconstructText(), control.language);
      assert.equal(reparsed.toLino(), text, 'same original complete source reparses deterministically');
    }
    assert.deepEqual(encodeLinkNetworkBinary(restoredBinary), binary,
      'complete binary snapshot reproduces its own bytes');
  });
}
