import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

function pinnedFile(argument, sha256) {
  const source = readFileSync(process.argv[argument]);
  assert.equal(createHash('sha256').update(source).digest('hex'), sha256);
  return source.toString('utf8');
}

// Adobe Glyph List 2.0 at adobe-type-tools/agl-aglfn@4036a9ca80a62f64f9de4f7321a9a045ad0ecfd6.
const adobeGlyphs = pinnedFile(2, 'a3b2f61ced9f3644cc0d4ecde5c59df34ca286c689d9484a43a710a81c466789');
const dingbats = pinnedFile(3, 'f6394e3cb8a447e84a1dad75d4baaf2aa7f45dc104faf369f4720e1a774ef2dc');
// Predefined encoding tables from mozilla/pdf.js@541d197e6b455341697496f24b1e00a88061f3eb.
const encodings = pinnedFile(4, 'eee7b0b49fbf0c27fd1765abeea43621c23da3d1a8a592ad0e5ddfe6743658e3');
const encodingLicense = pinnedFile(5, '0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594');
const adobeHeader = source => source.split('\n').filter(line => line.startsWith('#')).slice(1).join('\n')
  .split('# ---', 1)[0].replace(/^# ?/gmu, '');
const license = `${adobeHeader(adobeGlyphs)}${adobeHeader(dingbats)}Copyright 2012 Mozilla Foundation\n\n${encodingLicense}`;
const glyphs = new Map();
for (const line of `${adobeGlyphs}\n${dingbats}`.split('\n')) {
  if (!line || line.startsWith('#')) continue;
  assert.match(line, /^[A-Za-z0-9]+;[A-F0-9]{4,6}( [A-F0-9]{4,6})*$/u);
  const [name, scalars] = line.split(';');
  assert.ok(!glyphs.has(name));
  glyphs.set(name, scalars);
}
const glyphData = [...glyphs].map(([name, scalars]) => `(${name} ${scalars})`).join('\n');
const encodingData = [...encodings.matchAll(/const (\w+Encoding) = \[([\s\S]*?)\];/gu)].map(([, name, values]) => {
  const glyphNames = [...values.matchAll(/"([^"]*)"/gu)].map(match => match[1]);
  assert.equal(glyphNames.length, 256);
  return `(${name} ${glyphNames.map(glyph => glyphs.get(glyph)?.replaceAll(' ', '-') ?? '-').join(' ')})`;
});
assert.equal(encodingData.length, 7);
for (const directory of ['../src/data/', '../../rust/src/data/']) {
  for (const [name, content] of [['pdf-glyph-map.lino', `${glyphData}\n`], ['pdf-font-encodings.lino', `${encodingData.join('\n')}\n`], ['pdf-glyph-map-license.txt', license]]) {
    const file = new URL(`${directory}${name}`, import.meta.url);
    if (process.argv.includes('--check')) assert.equal(readFileSync(file, 'utf8'), content);
    else writeFileSync(file, content);
  }
}
