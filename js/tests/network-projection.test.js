import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';
import { readNetworkProjectionSourceModule } from '../scripts/generate-network-projection-rules.mjs';
import {
  ByteRange, LinkFlags, LinkMetadata, LinkNetwork, LinkType,
  NetworkProjection, Point, SourceSpan, encodeLinkNetworkBinary, decodeLinkNetworkBinary,
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

// These bounded controls reuse existing source bytes and independent CST rows;
// they do not establish parsing or AST coverage for the whole catalog.
function projectionSyntaxRows(network, root, base = 0) {
  const fields = new Map();
  for (const link of network.links()) {
    if (link.metadata().linkType !== LinkType.Field || link.references().length !== 3) continue;
    const [parent, label, child] = link.references();
    fields.set(`${parent.asU64()}:${child.asU64()}`, network.link(label).metadata().term);
  }
  const rows = [];
  const visit = (link, depth, field) => {
    const { term, named, span, flags } = link.metadata();
    rows.push([depth, field ?? null, term, named ? 1 : 0, span.byteRange.start - base,
      span.byteRange.end - base, `${flags.isError ? 'E' : ''}${flags.isMissing ? 'M' : ''}${flags.isExtra ? 'X' : ''}`]);
    for (const reference of link.references()) {
      const child = network.link(reference);
      if (child.metadata().linkType === LinkType.Syntax) {
        visit(child, depth + 1, fields.get(`${link.id().asU64()}:${reference.asU64()}`));
      }
    }
  };
  visit(network.link(root), 0);
  return rows;
}

function projectionDocumentRoot(network) {
  const syntax = network.links().filter(link => link.metadata().linkType === LinkType.Syntax);
  const children = new Set(syntax.flatMap(link => link.references().map(reference => reference.asU64())));
  const root = syntax.find(link => !children.has(link.id().asU64()));
  assert.ok(root, 'ordinary parsing exposes an actual grammar root');
  return root.id();
}

function assertProjectionReplay(network, source, trees, label) {
  // Verify the ordinary parse before invoking either view selector.
  assert.equal(network.reconstructText(), source, label + ': original source bytes');
  for (const role of [LinkType.Syntax, LinkType.Token, LinkType.Trivia]) {
    assert.ok(network.links().some(link => link.metadata().linkType === role), label + ': original CST role ' + role);
  }
  for (const [root, base, expected] of trees) {
    assert.deepEqual(projectionSyntaxRows(network, root, base), expected, label + ': original independent CST rows');
  }
  const before = network.toLino();
  const concreteIds = identifiers(network.projectedLinks());
  assert.deepEqual(concreteIds, identifiers(network.projectedLinks(NetworkProjection.ConcreteSyntax)));
  for (const role of [LinkType.Syntax, LinkType.Token, LinkType.Trivia]) {
    assert.ok(network.projectedLinks().some(link => link.metadata().linkType === role), label + ': default concrete role ' + role);
  }
  const abstractIds = identifiers(network.projectedLinks(NetworkProjection.AbstractSyntax));
  assert.ok(abstractIds.length > 0);
  assert.notDeepEqual(abstractIds, concreteIds, 'abstract syntax requires a distinct explicit selection');
  assert.equal(network.toLino(), before, label + ': selection leaves every original link unchanged');
  const binary = encodeLinkNetworkBinary(network);
  for (const [encoding, restored] of [
    ['text', LinkNetwork.fromLino(before)],
    ['binary', decodeLinkNetworkBinary(binary)],
  ]) {
    assert.equal(restored.toLino(), before, label + ': ' + encoding + ' complete graph');
    assert.equal(restored.reconstructText(), source, label + ': ' + encoding + ' exact source');
    assert.deepEqual([...new TextEncoder().encode(restored.reconstructText())], [...new TextEncoder().encode(source)], label + ': ' + encoding + ' UTF-8 bytes');
    assert.deepEqual(identifiers(restored.projectedLinks()), concreteIds, label + ': ' + encoding + ' omitted projection remains concrete');
    const abstract = restored.projectedLinks(NetworkProjection.AbstractSyntax);
    assert.deepEqual(identifiers(abstract), abstractIds, label + ': ' + encoding + ' saved abstract IDs');
    for (const link of abstract) {
      assert.equal(link, restored.link(link.id()), 'a view returns its actual restored Link');
      assert.equal(link.metadata(), restored.link(link.id()).metadata());
      assert.ok(![LinkType.Token, LinkType.Trivia].includes(link.metadata().linkType));
      assert.deepEqual(link.references(), network.link(link.id()).references());
      assert.deepEqual(link.metadata(), network.link(link.id()).metadata(), 'spans, fields and recovery flags survive');
      for (const reference of link.references()) assert.ok(restored.link(reference), 'concrete provenance remains resolvable');
    }
    for (const link of restored.links().filter(link => [LinkType.Syntax, LinkType.Field, LinkType.Region].includes(link.metadata().linkType))) {
      assert.ok(abstract.includes(link), 'all original syntax, field and region links remain in the explicit view');
    }
    for (const [root, base, expected] of trees) {
      assert.deepEqual(projectionSyntaxRows(restored, root, base), expected, label + ': ' + encoding + ' independent CST rows');
    }
    assert.deepEqual(identifiers(restored.projectedLinks()), concreteIds, 'AST selection cannot change the default view');
    assert.equal(restored.toLino(), before);
  }
}

test('four priority language CSTs retain original abstract provenance through text and binary replay', () => {
  const inventory = readProjectData(new URL('../../parity/language-grammar-inventory.lino', import.meta.url));
  const oracle = readProjectData(new URL('../../parity/fixtures/default-cst-expected.lino', import.meta.url));
  const native = readProjectData(new URL('../../parity/fixtures/native-default-cst-expected.lino', import.meta.url));
  for (const name of ['JavaScript', 'TypeScript', 'Rust', 'Markdown']) {
    const language = inventory.languages.find(entry => entry.name === name);
    assert.ok(language, name + ': the original inventory source exists');
    const expected = native.languages[name] ?? oracle.languages[name];
    assert.ok(expected, name + ': independently recorded CST expectations exist');
    for (const [part, key] of [['positive', 'source'], ['recovery', 'recoverySource']]) {
      const source = language[key], network = LinkNetwork.parse(source, name);
      assertProjectionReplay(network, source, [[projectionDocumentRoot(network), 0, expected[part]]], name + ' ' + part);
    }
  }
  recordIssue195Observations({
    requirementId: 'I195-CST-EXPLICIT-AST-SELECTION',
    suffix: 'priority-projection-replay',
    fixtureId: 'cst-projection-priority-javascript',
    fixtureFile: 'parity/language-grammar-inventory.lino',
    assertions: ['abstractSyntaxRequiresExplicitSelection', 'concreteSyntaxRemainsDefault', 'abstractProjectionRetainsConcreteProvenance'],
    testName: 'four priority language CSTs retain original abstract provenance through text and binary replay',
  });
});

test('the original Markdown fenced Rust region keeps host and child CST provenance in explicit abstract views', () => {
  const evidence = readProjectData(new URL('../../parity/fixtures/issue-195-evidence.lino', import.meta.url));
  const original = evidence.embedded.find(entry => entry.host === 'Markdown' && entry.target === 'language-selected fenced code');
  assert.ok(original);
  const oracle = readProjectData(new URL('../../parity/fixtures/default-cst-expected.lino', import.meta.url));
  const native = readProjectData(new URL('../../parity/fixtures/native-default-cst-expected.lino', import.meta.url));
  const label = original.host + ' -> ' + original.target;
  for (const [part, key] of [['positive', 'source'], ['recovery', 'recoverySource']]) {
    const source = original[key], network = LinkNetwork.parse(source, original.parseLanguage);
    const expected = oracle.embeddedFixtures[label][part];
    const embedded = native.embeddedFixtures[label]?.[part].embedded ?? expected.embedded;
    const regions = network.links().filter(link => link.metadata().linkType === LinkType.Region && link.metadata().span)
      .filter(link => !(link.metadata().language === original.parseLanguage && link.metadata().span.byteRange.start === 0
        && link.metadata().span.byteRange.end === new TextEncoder().encode(source).length));
    assert.deepEqual(regions.map(link => [link.metadata().language, link.metadata().span.byteRange.start, link.metadata().span.byteRange.end]),
      embedded.map(region => [region.language, region.startByte, region.endByte]));
    const trees = [[projectionDocumentRoot(network), 0, expected.rows]];
    regions.forEach((region, index) => {
      const root = region.references().slice(2).map(id => network.link(id)).find(link => link.metadata().linkType === LinkType.Syntax);
      assert.ok(root, 'the original region owns its actual child grammar root');
      assert.ok(network.link(region.references()[0]), 'the host document reference remains present');
      trees.push([root.id(), embedded[index].startByte, embedded[index].rows]);
    });
    assert.ok(regions.some(region => region.metadata().language === original.regionLanguage));
    assertProjectionReplay(network, source, trees, label + ' ' + part);
  }
  recordIssue195Observations({
    requirementId: 'I195-CST-EXPLICIT-AST-SELECTION',
    suffix: 'embedded-projection-replay',
    fixtureId: 'cst-projection-embedded-javascript',
    fixtureFile: 'parity/fixtures/issue-195-evidence.lino',
    assertions: ['abstractSyntaxRequiresExplicitSelection', 'concreteSyntaxRemainsDefault', 'abstractProjectionRetainsConcreteProvenance'],
    testName: 'the original Markdown fenced Rust region keeps host and child CST provenance in explicit abstract views',
  });
});
