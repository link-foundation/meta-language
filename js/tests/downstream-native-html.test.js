import assert from 'node:assert/strict';
import test from 'node:test';
import { LinkNetwork, LinkType } from 'meta-language';

test('the package entry point exposes native HTML elements, attributes, text and UTF-8 spans', () => {
  const source = '<article lang="fr" hidden>été <em>🙂</em> &amp;<br></article>\n';
  const network = LinkNetwork.parse(source, 'HTML');

  assert.ok(network.parseGrammars().some(({ id }) => id === 'native-html'));
  assert.ok(network.verifyFullMatch().isClean());
  assert.equal(network.reconstructText(), source);

  const isHtmlNode = (link) => {
    const { linkType, language, named } = link.metadata();
    return linkType === LinkType.Syntax && language === 'HTML' && named;
  };
  const children = (link) => link.references().map((id) => network.link(id)).filter(isHtmlNode);
  const kind = (link) => link.metadata().term;
  const raw = (link) => network.capturedText(link.id());
  const child = (link, term) => children(link).find((node) => kind(node) === term);
  const [document] = network.links().filter((link) => isHtmlNode(link) && kind(link) === 'document');
  const [article] = children(document);
  assert.equal(kind(article), 'element');

  const [opening, text, emphasis, entity, lineBreak, closing] = children(article);
  assert.deepEqual(children(article).map(kind), ['start_tag', 'text', 'element', 'entity', 'element', 'end_tag']);
  assert.equal(raw(child(opening, 'tag_name')), 'article');
  assert.equal(raw(child(closing, 'tag_name')), 'article');
  const attributes = children(opening).filter((link) => kind(link) === 'attribute');
  assert.deepEqual(attributes.map((link) => children(link).map(kind)), [
    ['attribute_name', 'quoted_attribute_value'], ['attribute_name'],
  ]);
  assert.deepEqual(attributes.map((link) => raw(child(link, 'attribute_name'))), ['lang', 'hidden']);
  assert.equal(raw(child(child(attributes[0], 'quoted_attribute_value'), 'attribute_value')), 'fr');
  assert.equal(raw(text), 'été');
  assert.equal(raw(entity), '&amp;');
  assert.deepEqual(children(lineBreak).map(kind), ['start_tag']);
  assert.equal(raw(child(child(lineBreak, 'start_tag'), 'tag_name')), 'br');

  const emphasisOpening = child(emphasis, 'start_tag');
  assert.equal(raw(child(emphasisOpening, 'tag_name')), 'em');
  const emoji = child(emphasis, 'text');
  const span = emoji.metadata().span;
  assert.deepEqual([span.byteRange.start, span.byteRange.end], [36, 40]);
  assert.deepEqual([span.start.row, span.start.column, span.end.row, span.end.column], [0, 36, 0, 40]);
  const bytes = Buffer.from(source, 'utf8');
  const toStringOffset = (offset) => bytes.subarray(0, offset).toString('utf8').length;
  const stringOffsets = [toStringOffset(span.byteRange.start), toStringOffset(span.byteRange.end)];
  assert.deepEqual(stringOffsets, [34, 36]);
  assert.equal(source.slice(...stringOffsets), '🙂');
  assert.equal(raw(emoji), '🙂');
  assert.deepEqual([document.metadata().span.end.row, document.metadata().span.end.column], [1, 0]);
});
