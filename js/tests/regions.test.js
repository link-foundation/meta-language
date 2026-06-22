import assert from 'node:assert/strict';
import { test } from 'node:test';

import {
  EmbeddedRegion,
  RegionDetectionPolicy,
  detectEmbeddedRegions,
  sniffLanguage,
} from '../src/regions.js';

const encoder = new TextEncoder();

function byteLength(value) {
  return encoder.encode(value).length;
}

// Byte offset of a substring within a string (UTF-8), mirroring Rust str::find.
function byteFind(text, needle) {
  const index = text.indexOf(needle);
  return index < 0 ? -1 : byteLength(text.slice(0, index));
}

function byteRFind(text, needle) {
  const index = text.lastIndexOf(needle);
  return index < 0 ? -1 : byteLength(text.slice(0, index));
}

function languages(regions) {
  return regions.map((region) => region.language());
}

test('RegionDetectionPolicy exposes the Rust enum variants', () => {
  assert.deepEqual(
    { ...RegionDetectionPolicy },
    {
      NameDriven: 'NameDriven',
      ContentDriven: 'ContentDriven',
      Both: 'Both',
    },
  );
  assert.ok(Object.isFrozen(RegionDetectionPolicy));
});

test('EmbeddedRegion exposes language and span accessors', () => {
  const regions = detectEmbeddedRegions('hello', 'txt', RegionDetectionPolicy.Both);
  assert.equal(regions.length, 1);
  const region = regions[0];
  assert.ok(region instanceof EmbeddedRegion);
  assert.equal(region.language(), 'txt');
  assert.equal(region.span().byteRange.start, 0);
  assert.equal(region.span().byteRange.end, byteLength('hello'));
});

test('txt parse exposes whole buffer as a single region', () => {
  const source = 'Plain text region\nUTF-8 line: café\n';
  const regions = detectEmbeddedRegions(source, 'txt', RegionDetectionPolicy.Both);
  assert.equal(regions.length, 1);
  assert.equal(regions[0].language(), 'txt');
  assert.equal(regions[0].span().byteRange.start, 0);
  assert.equal(regions[0].span().byteRange.end, byteLength(source));
});

test('markdown name-driven detection captures fenced and HTML regions', () => {
  const source = 'Intro\n```rust\nfn main() {}\n```\n<strong>HTML</strong>\n';
  const regions = detectEmbeddedRegions(source, 'Markdown', RegionDetectionPolicy.Both);
  const langs = languages(regions);

  assert.ok(langs.includes('rust'));
  assert.ok(langs.includes('HTML'));
  assert.ok(regions.every((region) => region.span().byteRange.end <= byteLength(source)));

  const rust = regions.find((region) => region.language() === 'rust');
  // Fence content starts right after the ```rust\n line and ends before closing fence line.
  assert.equal(rust.span().byteRange.start, byteFind(source, 'fn main'));
  assert.equal(rust.span().byteRange.end, byteRFind(source, '```\n'));

  const html = regions.find((region) => region.language() === 'HTML');
  assert.equal(html.span().byteRange.start, byteFind(source, '<strong>'));
  assert.equal(html.span().byteRange.end, byteFind(source, '</strong>') + byteLength('</strong>'));
});

test('content-driven markdown sniffs sql from fenced content', () => {
  const markdown = '# Query\n```\nSELECT 1;\n```\n';
  const regions = detectEmbeddedRegions(
    markdown,
    'Markdown',
    RegionDetectionPolicy.ContentDriven,
  );
  assert.ok(languages(regions).some((language) => language === 'sql-ansi'));
});

test('content-driven detection falls back to txt region with exact span', () => {
  const markdown = 'Notes\n```\nplain prose\ncafe au lait\n```\n';
  const regions = detectEmbeddedRegions(
    markdown,
    'Markdown',
    RegionDetectionPolicy.ContentDriven,
  );

  assert.equal(regions.length, 1);
  assert.equal(regions[0].language(), 'txt');
  assert.equal(regions[0].span().byteRange.start, byteFind(markdown, 'plain prose'));
  assert.equal(regions[0].span().byteRange.end, byteRFind(markdown, '```'));
});

test('content-driven markdown sniffs JavaScript from fenced content', () => {
  const markdown = 'Intro\n```\nconst value = 1;\n```\n';
  const regions = detectEmbeddedRegions(
    markdown,
    'Markdown',
    RegionDetectionPolicy.ContentDriven,
  );
  assert.ok(languages(regions).includes('JavaScript'));
});

test('name-driven markdown keeps explicit language tag', () => {
  const markdown = 'Intro\n```JavaScript\nconst value = 1;\n```\n<section><em>HTML</em></section>\n';
  const regions = detectEmbeddedRegions(markdown, 'Markdown', RegionDetectionPolicy.Both);
  const langs = languages(regions);
  assert.ok(langs.includes('JavaScript'));
  assert.ok(langs.includes('HTML'));

  const section = regions.find((region) => region.language() === 'HTML');
  // Region spans from the outer <section> open to its matching </section>.
  assert.equal(section.span().byteRange.start, byteFind(markdown, '<section>'));
  assert.equal(
    section.span().byteRange.end,
    byteFind(markdown, '</section>') + byteLength('</section>'),
  );
});

test('name-driven policy drops fences without a language tag', () => {
  const markdown = 'Intro\n```\nplain prose\n```\n';
  const regions = detectEmbeddedRegions(markdown, 'Markdown', RegionDetectionPolicy.NameDriven);
  assert.equal(regions.length, 0);
});

test('html detection covers script, style, and style attributes', () => {
  const html =
    '<script>const x = 1;</script><style>.x { color: red; }</style><p style="color: blue">text</p>';
  const regions = detectEmbeddedRegions(html, 'HTML', RegionDetectionPolicy.Both);
  const langs = languages(regions);

  assert.ok(langs.includes('JavaScript'));
  assert.equal(langs.filter((language) => language === 'CSS').length, 2);

  const js = regions.find((region) => region.language() === 'JavaScript');
  assert.equal(js.span().byteRange.start, byteFind(html, 'const x'));
  assert.equal(js.span().byteRange.end, byteFind(html, '</script>'));

  const styleElement = regions.find(
    (region) => region.language() === 'CSS' && region.span().byteRange.start === byteFind(html, '.x'),
  );
  assert.ok(styleElement);
  assert.equal(styleElement.span().byteRange.end, byteFind(html, '</style>'));

  const styleAttribute = regions.find(
    (region) =>
      region.language() === 'CSS' &&
      region.span().byteRange.start === byteFind(html, 'color: blue'),
  );
  assert.ok(styleAttribute);
  assert.equal(
    styleAttribute.span().byteRange.end,
    byteFind(html, 'color: blue') + byteLength('color: blue'),
  );
});

test('html element detection is case insensitive', () => {
  const html = '<SCRIPT>const y = 2;</SCRIPT>';
  const regions = detectEmbeddedRegions(html, 'html', RegionDetectionPolicy.Both);
  assert.equal(regions.length, 1);
  assert.equal(regions[0].language(), 'JavaScript');
  assert.equal(regions[0].span().byteRange.start, byteFind(html, 'const y'));
  assert.equal(regions[0].span().byteRange.end, byteFind(html, '</SCRIPT>'));
});

test('point rows and columns track newlines in fenced regions', () => {
  const source = 'Intro\n```rust\nfn main() {}\n```\n';
  const regions = detectEmbeddedRegions(source, 'Markdown', RegionDetectionPolicy.NameDriven);
  const rust = regions.find((region) => region.language() === 'rust');
  // "fn main() {}" begins on the third line (row index 2), column 0.
  assert.equal(rust.span().start.row, 2);
  assert.equal(rust.span().start.column, 0);
});

test('unknown language yields no regions', () => {
  const regions = detectEmbeddedRegions('whatever', 'python', RegionDetectionPolicy.Both);
  assert.deepEqual(regions, []);
});

test('sniffLanguage recognizes supported signatures', () => {
  assert.equal(sniffLanguage('  fn main() {}'), 'rust');
  assert.equal(sniffLanguage('def foo():'), 'Python');
  assert.equal(sniffLanguage('<div></div>'), 'HTML');
  assert.equal(sniffLanguage('function f() {}'), 'JavaScript');
  assert.equal(sniffLanguage('const x = 1;'), 'JavaScript');
  assert.equal(sniffLanguage('let y = 2;'), 'JavaScript');
  assert.equal(sniffLanguage('SELECT 1;'), 'sql-ansi');
  assert.equal(sniffLanguage('plain prose'), null);
});
