import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { readNetworkProjectionSourceModule } from '../scripts/generate-network-projection-rules.mjs';
import {
  ByteRange, LinkFlags, LinkMetadata, LinkNetwork, LinkType,
  NetworkProjection, Point, SourceSpan,
} from '../src/index.js';
import { acceptNetworkProjection, acceptNetworkProjectionLink } from '../src/network-projection-rules.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';

const fixture = () => readProjectData(new URL('../../parity/fixtures/network-projection-expectations.lino', import.meta.url));
const identifiers = links => links.map(link => link.id().asU64());

function roleNetwork() {
  const network = new LinkNetwork();
  for (const role of fixture().linkTypes) {
    const metadata = role === null ? LinkMetadata.new() : LinkMetadata.new().withLinkType(role);
    network.insertLink([], metadata);
  }
  return network;
}

test('all four network projections preserve the complete independent membership matrix', () => {
  const network = roleNetwork(), before = network.toLino();
  for (const entry of fixture().cases) {
    assert.deepEqual(identifiers(network.projectedLinks(entry.projection)), entry.included, entry.projection);
    assert.deepEqual(identifiers(network.projected_links(entry.projection)), entry.included, entry.projection);
    for (const link of network.projectedLinks(entry.projection)) {
      assert.equal(link, network.link(link.id()), 'the result is the actual source Link');
      assert.equal(link.metadata(), network.link(link.id()).metadata());
    }
  }
  assert.equal(NetworkProjection.Lossless, 'Lossless');
  assert.deepEqual(identifiers(network.projectedLinks()), fixture().cases[1].included);
  assert.deepEqual(identifiers(network.projected_links()), fixture().cases[1].included);
  assert.equal(network.toLino(), before);
});

test('abstract syntax retains original concrete provenance, field layout, spans and recovery flags', () => {
  const network = new LinkNetwork();
  const span = new SourceSpan(new ByteRange(0, 2), new Point(0, 0), new Point(0, 2));
  const token = network.insertSourceToken('JavaScript', 'id', span, LinkFlags.missing().withExtra());
  const trivia = network.insertLink([token], LinkMetadata.new().withLinkType(LinkType.Trivia).withSpan(span));
  const syntax = network.insertLink([token, trivia], LinkMetadata.new().withLinkType(LinkType.Syntax)
    .withTerm('identifier').withLanguage('JavaScript').withNamed().withSpan(span).withFlags(LinkFlags.containingError()));
  const field = network.insertFieldRelation(syntax, 'name', token);
  const label = network.link(field).references()[1];
  const region = network.insertLink([syntax, token], LinkMetadata.new().withLinkType(LinkType.Region).withSpan(span));
  const provenance = network.insertLink([syntax, token, trivia], LinkMetadata.new().withLinkType(LinkType.Reference));
  const original = network.link(syntax), originalSpan = original.metadata().span;
  const before = network.toLino();
  const abstract = network.projectedLinks(NetworkProjection.AbstractSyntax);
  assert.deepEqual(identifiers(abstract), [syntax, label, field, region, provenance].map(Number));
  const actual = abstract.find(link => link.id().equals(syntax));
  assert.equal(actual, original);
  assert.equal(actual.metadata().span, originalSpan);
  assert.deepEqual(actual.metadata().span, span);
  assert.equal(actual.metadata().flags.hasError, true);
  assert.deepEqual(network.link(field).references(), [syntax, label, token]);
  assert.equal(network.link(label).metadata().term, 'name');
  assert.deepEqual(actual.references(), [token, trivia]);
  for (const reference of actual.references()) {
    assert.ok(network.link(reference), 'excluded concrete links remain resolvable in the complete source network');
    assert.ok(!abstract.includes(network.link(reference)));
  }
  assert.equal(network.link(token).metadata().flags.isMissing, true);
  assert.equal(network.link(token).metadata().flags.isExtra, true);
  assert.equal(network.toLino(), before);
});

test('ordinary parsing retains source bytes and concrete links before and after explicit abstract selection', () => {
  const source = 'const value = 1; // café\n';
  const network = LinkNetwork.parse(source, 'JavaScript');
  const before = network.toLino(), size = network.len();
  const concrete = network.projectedLinks();
  assert.ok(concrete.some(link => link.metadata().linkType === LinkType.Token));
  assert.ok(concrete.some(link => link.metadata().linkType === LinkType.Trivia));
  assert.ok(concrete.some(link => link.metadata().linkType === LinkType.Syntax));
  const abstract = network.projectedLinks(NetworkProjection.AbstractSyntax);
  assert.ok(abstract.length > 0);
  assert.ok(abstract.every(link => ![LinkType.Token, LinkType.Trivia].includes(link.metadata().linkType)));
  assert.ok(abstract.some(link => link.references().some(reference =>
    [LinkType.Token, LinkType.Trivia].includes(network.link(reference)?.metadata().linkType))),
  'selected original links retain concrete provenance references');
  assert.equal(network.reconstructText(), source);
  assert.equal(network.len(), size);
  assert.equal(network.toLino(), before);
  assert.deepEqual(identifiers(network.projectedLinks()), identifiers(concrete));
  assert.deepEqual(identifiers(network.projectedLinks(NetworkProjection.Lossless)), identifiers(network.links()));
});

test('invalid projection selections refuse without changing the source network', () => {
  const network = roleNetwork(), before = network.toLino();
  for (const selection of ['', 'AST', 'abstract syntax', null, {}, 0, false]) {
    assert.throws(() => network.projectedLinks(selection), TypeError);
    assert.throws(() => network.projected_links(selection), TypeError);
  }
  assert.deepEqual(network.projectedLinks(undefined), network.projectedLinks());
  assert.equal(network.toLino(), before);
});

test('both complete projection decisions self-translate to all four targets and execute against every fixture cell', () => {
  const source = readNetworkProjectionSourceModule();
  const names = ['acceptNetworkProjection', 'acceptNetworkProjectionLink'];
  const surface = parseJavaScript(source, { referenceSemantics: false });
  assert.deepEqual(surface.items.filter(item => item.k === 'fn').map(item => item.name).sort(), names);
  const program = checkProgram(surface);
  assert.equal(program.declarations.size, names.length);
  for (const [target, emit] of [['JavaScript', emitJavaScript], ['Rust', emitRust], ['Lean', emitLean], ['Rocq', emitRocq]]) {
    const result = emit(program);
    assert.ok(result.text.length > 0, target);
    const mappings = result.mappings.filter(mapping => mapping.kind === 'function');
    assert.deepEqual(mappings.map(mapping => mapping.source).sort(), names, target);
    if (target === 'JavaScript' || target === 'Rust') assert.equal(result.definitions.length, names.length);
    for (const mapping of mappings) {
      const spelling = mapping.target.split('.').at(-1);
      const declaration = target === 'JavaScript' ? '^\\s*(?:export\\s+)?function\\s+'
        : target === 'Rust' ? '^\\s*pub\\s+fn\\s+'
          : target === 'Lean' ? '^\\s*(?:(?:partial|noncomputable|unsafe)\\s+)?def\\s+'
            : '^\\s*(?:Definition|Fixpoint|Function|with)\\s+';
      assert.match(result.text, new RegExp(declaration + spelling + '\\b', 'mu'), target + ':' + mapping.source);
    }
  }
  const translated = vm.runInNewContext(emitJavaScript(program).text + '\n({acceptNetworkProjection,acceptNetworkProjectionLink});', {}, { timeout: 1000 });
  for (const entry of fixture().cases) {
    assert.equal(acceptNetworkProjection(entry.projection), true);
    assert.equal(translated.acceptNetworkProjection(entry.projection), true);
    fixture().linkTypes.forEach((role, index) => {
      const expected = entry.included.includes(index + 1);
      assert.equal(acceptNetworkProjectionLink(entry.projection, role ?? ''), expected);
      assert.equal(translated.acceptNetworkProjectionLink(entry.projection, role ?? ''), expected);
    });
  }
  for (const invalid of ['', 'AST', 'abstract syntax']) {
    assert.equal(translated.acceptNetworkProjection(invalid), false);
    for (const role of fixture().linkTypes) assert.equal(translated.acceptNetworkProjectionLink(invalid, role ?? ''), false);
  }
});
