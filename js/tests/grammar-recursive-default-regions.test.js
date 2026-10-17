import assert from 'node:assert/strict';
import test from 'node:test';
import { LinkNetwork } from '../src/network.js';
import { ByteRange, LinkFlags, LinkMetadata, LinkType, ParseConfiguration, Point, SourceSpan } from '../src/primitives.js';
import { parseEmbeddedProgrammingLanguage, parseProgrammingLanguage } from '../src/programming-language-parser.js';
import { detectEmbeddedRegions } from '../src/regions.js';
import { readEmbeddedRegionAncestor, readEmbeddedRegionDisposition } from '../src/translation/embedded-region-rules.js';
import { readProjectData } from '../scripts/project-data.mjs';
import { recursiveEmbeddedRegionCases } from './support/recursive-embedded-region-cases.js';

function recursiveDefaultRegionLinks(network) {
  return network.links().filter(link => link.metadata().linkType === LinkType.Region);
}

// Compare every concrete node, token, flag, ordered child, field and absolute
// source coordinate, excluding only allocated LinkIds. The independent parse
// uses the registered parser directly, without network region attachment.
function recursiveConcreteProjection(network, roots) {
  const fields = new Map();
  for (const field of network.fieldRelations()) {
    const key = field.parent.asU64();
    if (!fields.has(key)) fields.set(key, []);
    fields.get(key).push(field);
  }
  const project = id => {
    const link = network.link(id);
    assert.ok(link, 'every retained CST reference resolves');
    const metadata = link.metadata();
    assert.ok([LinkType.Syntax, LinkType.SourceToken].includes(metadata.linkType));
    const references = link.references();
    const span = metadata.span;
    return { type: metadata.linkType, term: metadata.term, language: metadata.language, named: metadata.named,
      flags: { ...metadata.flags },
      span: span ? { start: span.byteRange.start, end: span.byteRange.end,
        startPoint: { row: span.start.row, column: span.start.column },
        endPoint: { row: span.end.row, column: span.end.column } } : null,
      fields: (fields.get(id.asU64()) ?? []).map(field => ({ label: field.label,
        child: references.findIndex(reference => reference.equals(field.child)) })),
      children: references.map(project) };
  };
  return roots.map(project);
}

function assertRecursiveDefaultRegions(network, fixture) {
  const regions = recursiveDefaultRegionLinks(network);
  assert.equal(regions.length, fixture.regions.length, 'FIFO retains every authored region exactly once');
  const root = regions[0].references()[0];
  const configuration = ParseConfiguration.default().withRegionDetectionPolicy(fixture.policy);
  for (const [index, expected] of fixture.regions.entries()) {
    const region = regions[index];
    const metadata = region.metadata();
    assert.equal(metadata.language, expected.language);
    assert.deepEqual({ start: metadata.span.byteRange.start, end: metadata.span.byteRange.end },
      { start: expected.startByte, end: expected.endByte });
    assert.deepEqual({ ...metadata.span.start }, expected.startPoint);
    assert.deepEqual({ ...metadata.span.end }, expected.endPoint);
    assert.equal(Buffer.from(fixture.source).subarray(expected.startByte, expected.endByte).toString('utf8'), expected.source);
    const parent = expected.parent < 0 ? root : regions[expected.parent].id();
    assert.equal(region.references()[0].asU64(), parent.asU64(), 'nested regions name their actual containing Region');
    assert.ok(region.references().length > 2, 'each accepted region owns its complete registered-parser CST');
    const direct = new LinkNetwork();
    const parsed = parseEmbeddedProgrammingLanguage(expected.source, expected.language);
    const { outer } = direct._insertProgrammingLanguage(parsed, expected.language, configuration, metadata.span);
    assert.deepEqual(recursiveConcreteProjection(network, region.references().slice(2)),
      recursiveConcreteProjection(direct, outer), 'the whole child CST survives attachment with one absolute offset');
  }
  const directHost = new LinkNetwork();
  const { root: directRoot } = directHost._insertProgrammingLanguage(
    parseProgrammingLanguage(fixture.source, fixture.host), fixture.host, configuration);
  assert.deepEqual(recursiveConcreteProjection(network, [root]), recursiveConcreteProjection(directHost, [directRoot]),
    'descendant Regions never replace or concatenate the original host CST children');
  assert.equal(network.reconstructText(), fixture.source);
  assert.equal(network.renderSource(fixture.host), fixture.source, 'same-language descendants do not duplicate host output');
  assert.equal(network.verifyFullMatch().isClean(), true);
}

const recursiveCases = recursiveEmbeddedRegionCases();
for (const fixture of recursiveCases.cases) test(`default recursive CST attachment preserves ${fixture.name}`, () => {
  const configuration = ParseConfiguration.default().withRegionDetectionPolicy(fixture.policy);
  const direct = detectEmbeddedRegions(fixture.source, fixture.host, fixture.policy);
  assert.deepEqual(direct.map(region => [region.language(), region.span().byteRange.start, region.span().byteRange.end]),
    fixture.regions.filter(region => region.parent < 0).map(region => [region.language, region.startByte, region.endByte]),
    'the original direct detector remains unchanged');
  const network = LinkNetwork.parse(fixture.source, fixture.host, configuration);
  assertRecursiveDefaultRegions(network, fixture);
  const restored = LinkNetwork.fromLino(network.toLino());
  assert.equal(restored.toLino(), network.toLino());
  assertRecursiveDefaultRegions(restored, fixture);
});

function embeddedDiagnosticFields(network, diagnostic) {
  return Object.fromEntries(network.fieldRelations().filter(field => field.parent.equals(diagnostic.id()))
    .map(field => [field.label, network.link(field.child).metadata().term]));
}

test('default recursive adapter limits retain refused boundaries counters and exact source after reload', () => {
  const child = 'const value = "π";';
  const html = `<script>${child}</script>\n`;
  const source = `\`\`\`html\n${html}\`\`\`\n`;
  const htmlBytes = Buffer.byteLength(html), childBytes = Buffer.byteLength(child);
  for (const [limits, reason, observed, limit] of [
    [{ depthLimit: 1 }, 'embeddedRegionDepthLimit', 2, 1],
    [{ countLimit: 1 }, 'embeddedRegionCountLimit', 2, 1],
    [{ byteLimit: htmlBytes }, 'embeddedRegionByteLimit', htmlBytes + childBytes, htmlBytes],
  ]) {
    const network = LinkNetwork.parse(source, 'Markdown', ParseConfiguration.default().withEmbeddedRegionLimits(limits));
    for (const current of [network, LinkNetwork.fromLino(network.toLino())]) {
      const regions = recursiveDefaultRegionLinks(current);
      assert.deepEqual(regions.map(region => region.metadata().language), ['HTML', 'JavaScript']);
      assert.ok(regions[0].references().length > 2);
      assert.equal(regions[1].references().length, 2, 'refusal retains a boundary rather than inventing a CST');
      assert.equal(regions[1].references()[0].asU64(), regions[0].id().asU64());
      const diagnostics = current.links().filter(link => link.metadata().term === 'embedded region diagnostic');
      assert.equal(diagnostics.length, 1);
      assert.equal(diagnostics[0].metadata().definition, reason);
      assert.equal(diagnostics[0].references()[0].asU64(), regions[1].id().asU64());
      assert.deepEqual(embeddedDiagnosticFields(current, diagnostics[0]), {
        depth: '2', 'region count': '1', 'child parse bytes': String(htmlBytes),
        observed: String(observed), limit: String(limit),
      });
      assert.equal(current.verifyFullMatch().isClean(), false);
      assert.equal(current.reconstructText(), source);
      assert.equal(current.renderSource('Markdown'), source);
    }
  }
  const admitted = LinkNetwork.parse(source, 'Markdown', ParseConfiguration.default()
    .withEmbeddedRegionLimits({ depthLimit: 2, countLimit: 2, byteLimit: htmlBytes + childBytes }));
  assert.equal(admitted.verifyFullMatch().isClean(), true);
  assert.ok(recursiveDefaultRegionLinks(admitted)[1].references().length > 2);
  const rootOnly = LinkNetwork.parse('plain café', 'txt', ParseConfiguration.default()
    .withEmbeddedRegionLimits({ depthLimit: 0, countLimit: 0, byteLimit: 0 }));
  assert.equal(recursiveDefaultRegionLinks(rootOnly).length, 1);
  assert.equal(rootOnly.verifyFullMatch().isClean(), true, 'the pre-existing whole-root alias spends no child parse budget');
  assert.equal(rootOnly.reconstructText(), 'plain café');
});

test('default recursive attachment shares adapter counters across siblings and preserves source rendering fragments', () => {
  const child = 'const nested = 1;';
  const sibling = 'const sibling = 2;\n';
  const source = `\`\`\`html\n<script>${child}</script>\n\`\`\`\n\n\`\`\`JavaScript\n${sibling}\`\`\`\n`;
  const network = LinkNetwork.parse(source, 'Markdown', ParseConfiguration.default().withEmbeddedRegionLimits({ countLimit: 2 }));
  const regions = recursiveDefaultRegionLinks(network);
  assert.deepEqual(regions.map(region => region.metadata().language), ['HTML', 'JavaScript', 'JavaScript']);
  assert.ok(regions[0].references().length > 2);
  assert.ok(regions[1].references().length > 2, 'the original direct sibling is parsed before descendants');
  assert.equal(regions[2].references().length, 2, 'a child does not get a fresh count budget');
  const diagnostic = network.links().find(link => link.metadata().definition === 'embeddedRegionCountLimit');
  assert.equal(embeddedDiagnosticFields(network, diagnostic).observed, '3');
  assert.equal(network.renderSource('JavaScript'), sibling);
  assert.equal(network.renderSource('Markdown'), source);
  assert.equal(network.reconstructText(), source);
  const constructed = new LinkNetwork();
  constructed.insertSourceToken('fragment', 'first');
  constructed.insertSourceToken('fragment', 'second');
  assert.equal(constructed.renderSource('fragment'), 'firstsecond', 'unspanned constructed tokens retain their existing order');
});

test('synthetic adapter cycle diagnostics preserve actual ancestry and remain verification failures after reload', () => {
  const network = new LinkNetwork();
  const span = new SourceSpan(new ByteRange(0, 2), new Point(0, 0), new Point(0, 2));
  const outer = network.insertLink([], LinkMetadata.new().withLinkType(LinkType.Region).withLanguage('Markdown').withSpan(span));
  const inner = network.insertLink([outer], LinkMetadata.new().withLinkType(LinkType.Region).withLanguage('HTML').withSpan(span));
  const rejected = network.insertLink([inner], LinkMetadata.new().withLinkType(LinkType.Region).withLanguage('Markdown').withSpan(span));
  const ancestor = readEmbeddedRegionAncestor([-1, 0], [0, 1], [0, 0], [2, 2], 1, 0, 0, 2);
  assert.equal(ancestor, 0);
  const reason = readEmbeddedRegionDisposition(0, 2, 1, 0, 0, 2, ancestor, 2, 1, 2, 128, 4096, 16777216, true);
  assert.equal(reason, 'nonConsumingEmbeddingCycle');
  const diagnostic = network._insertEmbeddedRegionDiagnostic(rejected, 'Markdown', span, reason,
    { depth: 2, count: 1, bytes: 2, observed: -1, limit: -1, ancestor: outer });
  network.insertSourceToken('Markdown', 'π', span, LinkFlags.clean());
  for (const current of [network, LinkNetwork.fromLino(network.toLino())]) {
    assert.equal(current.verifyFullMatch().isClean(), false);
    assert.equal(current.link(diagnostic).metadata().definition, reason);
    const ancestorField = current.fieldRelations().find(field => field.parent.equals(diagnostic) && field.label === 'ancestor');
    assert.equal(ancestorField.child.asU64(), outer.asU64());
    assert.equal(Object.hasOwn(embeddedDiagnosticFields(current, current.link(diagnostic)), 'observed'), false);
    assert.equal(current.reconstructText(), 'π');
  }
});

test('shared recursive default region fixture retains every authored source and expected absolute boundary', () => {
  assert.deepEqual(readProjectData(new URL('../../parity/fixtures/recursive-default-embedded-regions.lino', import.meta.url)), recursiveCases);
});

test('default root language aliases retain their original direct txt CST and token views', () => {
  const source = 'plain café\n';
  for (const language of ['txt', 'TXT', 'text', 'TeXt', 'plain text', 'PLAIN TEXT']) {
    const configuration = ParseConfiguration.default();
    const expected = new LinkNetwork();
    const { root } = expected._insertProgrammingLanguage(parseProgrammingLanguage(source, language), language, configuration);
    const languageLink = expected.insertTypedPoint(LinkType.Language, 'txt');
    const span = new SourceSpan(new ByteRange(0, Buffer.byteLength(source)), new Point(0, 0), new Point(1, 0));
    const boundary = expected.insertLink([root, languageLink], LinkMetadata.new()
      .withLinkType(LinkType.Region).withNamed(true).withTerm('txt region').withLanguage('txt').withSpan(span));
    if (language.toLowerCase() !== 'txt') {
      expected._recordGrammars(languageLink, 'txt');
      const { outer } = expected._insertProgrammingLanguage(parseEmbeddedProgrammingLanguage(source, 'txt'), 'txt', configuration, span);
      expected.link(boundary).setReferences([root, languageLink, ...outer]);
    }
    const actual = LinkNetwork.parse(source, language, configuration);
    assert.equal(actual.toLino(), expected.toLino(), 'every original alias link, token and direct CST remains byte-identical');
    assert.equal(recursiveDefaultRegionLinks(actual).length, 1, 'the attached txt view never creates another self-region');
    assert.equal(actual.reconstructText(), source);
    assert.equal(actual.renderSource(language), source);
    assert.equal(actual.verifyFullMatch().isClean(), true);
  }
});
