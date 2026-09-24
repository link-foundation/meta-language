import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import {
  LinkNetwork,
  LinkType,
  ProgramRepresentation,
  analyzeProgram,
} from '../src/index.js';

const evidence = JSON.parse(
  await readFile(new URL('../../parity/fixtures/issue-195-evidence.json', import.meta.url)),
);
const encoder = new TextEncoder();

test('pinned external corpora and representative projects exercise every four-language frontend', () => {
  for (const fixture of evidence.externalConformance) {
    assert.match(fixture.provenance.url, /^https:\/\/github\.com\//u);
    assert.ok(fixture.provenance.revision);
    assert.ok(fixture.provenance.license);
    assertStructuredSource(fixture.source, fixture.language, fixture.root, fixture.requiredTerms);
  }

  for (const fixture of evidence.representativeProjects) {
    const project = { root: '/project', files: fixture.files, dependencies: fixture.dependencies };
    const program = analyzeProgram(fixture.source, fixture.language, project);
    assert.equal(program.emit(), fixture.source, `${fixture.language} project reconstruction`);
    assert.equal(program.network.verifyFullMatch().isClean(), true, `${fixture.language} project CST`);
    assert.ok(program.bindings.length > 0, `${fixture.language} project bindings`);
    assert.ok(program.sourceMappings.length > 0, `${fixture.language} project mappings`);
    assert.equal(
      program.diagnostics.some(({ kind }) => kind === 'missing-project-context'),
      false,
      `${fixture.language} project context`,
    );
  }
});

test('shared embedded fixtures connect host boundaries to target grammar roots', () => {
  for (const fixture of evidence.embedded) {
    const network = LinkNetwork.parse(fixture.source, fixture.parseLanguage);
    const language = fixture.regionLanguage ?? fixture.target;
    const start = byteFind(fixture.source, fixture.regionSource);
    const end = start + encoder.encode(fixture.regionSource).length;
    const region = network.links().find((link) => {
      const metadata = link.metadata();
      return metadata.linkType === LinkType.Region &&
        metadata.language.toLowerCase() === language.toLowerCase() &&
        metadata.span?.byteRange.start === start &&
        metadata.span?.byteRange.end === end;
    });
    assert.ok(region, `${fixture.host} -> ${fixture.target} region`);
    const root = region.references()
      .map((reference) => network.link(reference))
      .find((link) => link?.metadata().linkType === LinkType.Syntax &&
        link.metadata().language.toLowerCase() === language.toLowerCase() &&
        link.metadata().term === fixture.root);
    assert.ok(root, `${fixture.host} -> ${fixture.target} grammar root`);
    assert.equal(root.metadata().span.byteRange.start, start);
    assert.equal(root.metadata().span.byteRange.end, end);
    assert.equal(network.reconstructText(), fixture.source);
  }
});

test('seeded generative, fuzz, and metamorphic cases obey independent source oracles', () => {
  const { seed, casesPerLanguage, unicodeIdentifiers } = evidence.generative;
  const random = lcg(seed);
  for (const language of ['JavaScript', 'Rust', 'Lean', 'Rocq']) {
    for (let index = 0; index < casesPerLanguage; index += 1) {
      const number = random() % 10_000;
      const stem = unicodeIdentifiers[random() % unicodeIdentifiers.length];
      const identifier = `${stem}${index}`;
      const source = generatedSource(language, identifier, number, index);
      const network = LinkNetwork.parse(source, language);
      assert.equal(network.reconstructText(), source, `${language} generated reconstruction ${index}`);
      assert.equal(network.verifyFullMatch().isClean(), true, `${language} generated parse ${index}`);
      assertUtf8TokenSpans(network, source, language, index);

      const prefix = comment(language, `metamorphic ${number}`);
      const prefixed = `${prefix}${source}`;
      const transformed = LinkNetwork.parse(prefixed, language);
      assert.equal(transformed.reconstructText(), prefixed, `${language} metamorphic ${index}`);
      assert.equal(transformed.verifyFullMatch().isClean(), true, `${language} metamorphic parse ${index}`);

      const program = analyzeProgram(source, language);
      const edited = program.insert(source.length, comment(language, `edit ${index}`));
      assert.equal(
        edited.emit(),
        `${source}${comment(language, `edit ${index}`)}`,
        `${language} edit oracle ${index}`,
      );
      const restored = ProgramRepresentation.fromSnapshot(edited.snapshot());
      assert.equal(restored.emit(), edited.emit(), `${language} snapshot oracle ${index}`);

      const malformed = `${source}\0`;
      const recovered = LinkNetwork.parse(malformed, language);
      assert.equal(recovered.reconstructText(), malformed, `${language} malformed retention ${index}`);
      assert.equal(recovered.verifyFullMatch().isClean(), false, `${language} malformed diagnostic ${index}`);
    }
  }
});

function assertStructuredSource(source, language, root, requiredTerms) {
  const network = LinkNetwork.parse(source, language);
  assert.equal(network.reconstructText(), source, `${language} external reconstruction`);
  assert.equal(network.verifyFullMatch().isClean(), true, `${language} external parse`);
  const syntax = network.links()
    .filter((link) => link.metadata().linkType === LinkType.Syntax)
    .map((link) => link.metadata().term);
  assert.ok(syntax.includes(root), `${language} ${root}`);
  for (const term of requiredTerms) assert.ok(syntax.includes(term), `${language} ${term}`);
}

function assertUtf8TokenSpans(network, source, language, index) {
  const byteLength = encoder.encode(source).length;
  const tokens = network.links().filter((link) => link.metadata().linkType === LinkType.SourceToken);
  assert.ok(tokens.length > 1, `${language} generated tokens ${index}`);
  for (const token of tokens) {
    const { start, end } = token.metadata().span.byteRange;
    assert.ok(start >= 0 && end >= start && end <= byteLength, `${language} byte span ${index}`);
  }
}

function byteFind(source, needle) {
  const index = source.indexOf(needle);
  assert.notEqual(index, -1, `fixture region ${JSON.stringify(needle)}`);
  return encoder.encode(source.slice(0, index)).length;
}

function lcg(seed) {
  let state = seed >>> 0;
  return () => {
    state = (Math.imul(state, 1_664_525) + 1_013_904_223) >>> 0;
    return state;
  };
}

function generatedSource(language, identifier, number, index) {
  if (language === 'JavaScript') {
    return `const ${identifier} = ${number};\nfunction f${index}(value) { return value + ${identifier}; }\n`;
  }
  if (language === 'Rust') {
    return `const ${identifier}: u64 = ${number};\nfn f${index}(value: u64) -> u64 { value + ${identifier} }\n`;
  }
  if (language === 'Lean') {
    return `def ${identifier} : Nat := ${number}\ndef f${index} (value : Nat) : Nat := value + ${identifier}\n`;
  }
  return `Definition ${identifier} : nat := ${number}.\nDefinition f${index} (value : nat) : nat := value + ${identifier}.\n`;
}

function comment(language, text) {
  return language === 'Lean' ? `-- ${text}\n` : language === 'Rocq' ? `(* ${text} *)\n` : `// ${text}\n`;
}
