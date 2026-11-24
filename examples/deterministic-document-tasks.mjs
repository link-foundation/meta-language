import assert from 'node:assert/strict';

// Pass the public module namespace from either an installed package or a
// checkout. Each example returns the source-linked observations it verified.
export function verifyDocumentTasks(api, { checkGrammarAssets = false } = {}) {
  const observations = {
    json: verifyJson(api),
    extraction: verifyExtraction(api),
    grounding: verifyGrounding(api),
    morphology: verifyMorphology(api),
    html: verifyHtml(api),
  };
  if (checkGrammarAssets) observations.fallbackGrammars = verifyFallbackGrammarAssets(api);
  return observations;
}

function verifyJson({ validateJson, recoverJson, importJsonSchema, createGrammarParser }) {
  const schema = {
    type: 'object',
    required: ['status', 'evidence'],
    properties: { status: { const: 'met' }, evidence: { type: 'string' } },
  };
  const invalid = '{"status":"unknown","evidence":"café';
  const report = validateJson(invalid, schema);
  const end = Buffer.byteLength(invalid);
  const faults = report.errors.map(({ keyword, pointer, span, linkId }) => {
    assert.ok(report.network.link(linkId));
    return [keyword, pointer, span.byteRange.start, span.byteRange.end];
  });
  assert.equal(report.valid, false);
  assert.equal(report.network.reconstructText(), invalid);
  assert.deepEqual(faults, [['syntax', '/evidence', end, end], ['const', '/status', 10, 19]]);

  const truncated = invalid.replace('"unknown"', '"met"');
  const unchanged = recoverJson(truncated, { schema });
  assert.equal(unchanged.text, truncated);
  assert.deepEqual(unchanged.edits, []);
  const repaired = recoverJson(truncated, { schema, allow: ['closeOpenContainers'] });
  assert.equal(repaired.sourceNetwork.reconstructText(), truncated);
  const edits = repaired.edits.map(({ kind, inserted, span, linkId }) => {
    assert.ok(repaired.sourceNetwork.link(linkId).metadata().flags.hasRecoveryIssue());
    return [kind, inserted, span.start, span.end];
  });
  const tail = Buffer.byteLength(truncated);
  assert.deepEqual(edits, [
    ['closeOpenContainers', '"', tail, tail], ['closeOpenContainers', '}', tail, tail],
  ]);
  assert.deepEqual(repaired.value, { status: 'met', evidence: 'café' });
  assert.equal(validateJson(repaired.text, schema).valid, true);
  const parser = createGrammarParser(importJsonSchema(schema));
  assert.equal(parser.parseTree(repaired.text).ok, true);
  assert.equal(parser.parseTree('{"status":"unknown","evidence":"café"}').ok, false);
  return { faults, edits, repaired: repaired.text };
}

function verifyExtraction({ naturalLanguageValues, createGrammarParser, importEbnf }) {
  const text = '💬 Rent is 1,234.56 USD and room 42 is open.';
  const bytes = Buffer.from(text);
  const value = naturalLanguageValues(text, 'English').find(({ term }) => term.startsWith('value:money '));
  assert.equal(value.term, 'value:money {"amount":1234.56,"currency":"USD"}');
  assert.equal(bytes.subarray(value.start, value.end).toString(), '1,234.56 USD');
  const parser = createGrammarParser(importEbnf('room = "room ", number ; number = "42" ;'));
  assert.equal(parser.parseTree(text).ok, false);
  const matches = parser.findAll(text, { wordBoundaries: true, captures: ['number'] });
  assert.equal(matches.length, 1);
  const [match] = matches;
  assert.equal(text.slice(match.start, match.end), 'room 42');
  assert.equal(bytes.subarray(match.tree.start, match.tree.end).toString(), 'room 42');
  assert.deepEqual(match.captures.number.map(({ start, end }) => text.slice(start, end)), ['42']);
  return { value: value.term, valueRange: [value.start, value.end], matchRange: [match.tree.start, match.tree.end] };
}

function verifyGrounding({ locate }) {
  const text = 'The file says “Café au lait”—paid.';
  const fragment = '"Cafe\u0301 au lait"-paid';
  const matches = locate(text, fragment, { normalize: ['nfkc', 'quotes', 'dashes'] });
  assert.deepEqual(matches.map(({ text, distance, normalizations }) => [text, distance, normalizations]), [
    ['“Café au lait”—paid', 0, ['nfkc', 'quotes', 'dashes']],
  ]);
  const [{ start, end }] = matches;
  assert.deepEqual([start, end], [14, 33]);
  assert.equal(text.slice(start, end), matches[0].text);
  return matches[0];
}

function verifyMorphology({ analyzeMorphology, identifyLanguage }) {
  return [
    ['The engineers are working.', 'engineers', 'English', 'engineer'],
    ['Инженеры работают над проектом.', 'инженеры', 'Russian', 'инженер'],
  ].map(([text, word, language, lemma]) => {
    const identified = identifyLanguage(text);
    assert.equal(identified.language, language);
    const readings = analyzeMorphology(word, identified.language);
    const reading = readings.find(item => item.lemma === lemma && item.upos === 'NOUN');
    assert.ok(reading, word);
    return { language: identified.language, word, lemma: reading.lemma, upos: reading.upos };
  });
}

function verifyHtml({ LinkNetwork, LinkType }) {
  const html = '<article lang="en">Café <em>au lait</em></article>';
  const bytes = Buffer.from(html);
  const network = LinkNetwork.parse(html, 'HTML');
  assert.equal(network.reconstructText(), html);
  const isSyntax = link => link.metadata().linkType === LinkType.Syntax;
  const document = network.links().find(link => isSyntax(link) && link.metadata().term === 'document');
  const children = link => link.references().map(id => network.link(id)).filter(isSyntax);
  const child = (link, term) => children(link).find(link => link.metadata().term === term);
  const source = link => {
    const { start, end } = link.metadata().span.byteRange;
    return bytes.subarray(start, end).toString();
  };
  const element = child(document, 'element');
  const startTag = child(element, 'start_tag');
  const attribute = child(startTag, 'attribute');
  const name = source(child(startTag, 'tag_name'));
  const attributeName = source(child(attribute, 'attribute_name'));
  const attributeValue = source(child(child(attribute, 'quoted_attribute_value'), 'attribute_value'));
  const textLink = child(element, 'text');
  const text = source(textLink);
  const nested = source(child(element, 'element'));
  const { start, end } = textLink.metadata().span.byteRange;
  const characterRange = [bytes.subarray(0, start).toString().length, bytes.subarray(0, end).toString().length];
  assert.deepEqual([name, attributeName, attributeValue, text, nested], ['article', 'lang', 'en', 'Café', '<em>au lait</em>']);
  assert.deepEqual([start, end], [19, 24]);
  assert.deepEqual(characterRange, [19, 23]);
  return { name, attributeName, attributeValue, text, nested, byteRange: [start, end], characterRange };
}

function verifyFallbackGrammarAssets({ LinkNetwork, LinkType }) {
  const unavailable = LinkNetwork.languageParserAvailability().filter(({ available }) => !available);
  assert.deepEqual(unavailable, [], 'all installed standalone parsers have their grammar assets');
  const fixtures = [
    ['Markdown', '# *Title*\n\n- item\n', [['atx_heading', 0, 10], ['emphasis', 2, 9], ['list_item', 11, 18]]],
    ['YAML', 'key: value\n', [['block_mapping_pair', 0, 10]]],
    ['Kotlin', 'val x = 1\n', [['property_declaration', 0, 9]]],
  ];
  return fixtures.map(([language, source, expected]) => {
    const network = LinkNetwork.parse(source, language);
    assert.equal(network.reconstructText(), source, language);
    assert.equal(network.verifyFullMatch().isClean(), true, language);
    const syntax = network.links().map(link => link.metadata()).filter(({ linkType }) => linkType === LinkType.Syntax);
    const spans = expected.map(([term]) => {
      const node = syntax.find(metadata => metadata.term === term);
      assert.ok(node?.span, `${language} ${term}`);
      return [term, node.span.byteRange.start, node.span.byteRange.end];
    });
    assert.deepEqual(spans, expected, language);
    return { language, spans };
  });
}
