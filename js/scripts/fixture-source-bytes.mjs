// Legacy fixture provenance hashes describe their original JSON source bytes.
// LiNo is the only stored fixture value; this separate links record describes
// whitespace and exceptional scalar spellings needed to reproduce those bytes.
import { readFileSync } from 'node:fs';
import { relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import { isDeepStrictEqual } from 'node:util';
import { readProjectData, resolveProjectDataPath, writeProjectData } from './project-data.mjs';

const repository = fileURLToPath(new URL('../../', import.meta.url));
const layoutFile = new URL('../../parity/fixtures/issue-195-json-source-layouts.lino', import.meta.url);
let layouts;

function tokens(source) {
  const pattern = /"(?:\\.|[^"\\])*"|-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?|true|false|null|[{}\[\]:,]|[ \t\r\n]+/gy;
  const records = [];
  let position = 0;
  while (position < source.length) {
    pattern.lastIndex = position;
    const match = pattern.exec(source);
    if (!match) throw new Error(`fixture JSON source has an invalid token at ${position}`);
    records.push(match[0]);
    position = pattern.lastIndex;
  }
  return records;
}

/** Derive formatting records without retaining an opaque JSON document. */
export function createFixtureSourceLayout(source) {
  const value = JSON.parse(source);
  const canonical = tokens(JSON.stringify(value));
  const whitespace = [];
  const spellings = [];
  let index = 0;
  for (const token of tokens(source)) {
    if (/^[ \t\r\n]+$/u.test(token)) whitespace.push([index, token]);
    else {
      if (token !== canonical[index]) {
        if (!isDeepStrictEqual(JSON.parse(token), JSON.parse(canonical[index]))) throw new Error('fixture source token value changed');
        spellings.push([index, token]);
      }
      index += 1;
    }
  }
  if (index !== canonical.length) throw new Error('fixture source structural token count changed');
  const layout = { tokenCount: index, whitespace, spellings };
  if (renderFixtureSourceBytes(value, layout).toString('utf8') !== source) throw new Error('fixture JSON source layout is not lossless');
  return layout;
}

/** Recreate a legacy byte oracle from decoded links and formatting links. */
export function renderFixtureSourceBytes(value, layout) {
  const canonical = tokens(JSON.stringify(value));
  if (!layout || canonical.length !== layout.tokenCount) throw new Error('fixture source token count differs from its provenance');
  const whitespace = new Map();
  for (const [index, text] of layout.whitespace) {
    if (!Number.isSafeInteger(index) || index < 0 || index > canonical.length || whitespace.has(index) || !/^[ \t\r\n]+$/u.test(text)) throw new Error('invalid fixture source whitespace record');
    whitespace.set(index, text);
  }
  const spellings = new Map();
  for (const [index, text] of layout.spellings) {
    if (!Number.isSafeInteger(index) || index < 0 || index >= canonical.length || spellings.has(index)
      || /^[{}\[\]:,]$/u.test(canonical[index]) || !isDeepStrictEqual(JSON.parse(text), JSON.parse(canonical[index]))) throw new Error('invalid fixture source scalar spelling');
    spellings.set(index, text);
  }
  return Buffer.from(canonical.map((token, index) => `${whitespace.get(index) ?? ''}${spellings.get(index) ?? token}`).join('') + (whitespace.get(canonical.length) ?? ''), 'utf8');
}

/** Read source-language files normally and preserve legacy fixture hash bytes. */
export function readFixtureSourceBytes(file) {
  const logical = file instanceof URL ? fileURLToPath(file) : resolve(file);
  const actual = resolveProjectDataPath(logical);
  if (!logical.endsWith('.json') || !actual.endsWith('.lino')) return readFileSync(actual);
  layouts ??= readProjectData(layoutFile);
  const name = relative(repository, logical).split(sep).join('/');
  if (!Object.hasOwn(layouts, name)) throw new Error(`fixture has no legacy source byte contract: ${name}`);
  return renderFixtureSourceBytes(readProjectData(logical), layouts[name]);
}

/** Regenerate a structured fixture and the layout of its provenance source. */
export function writeFixtureSourceData(file, value, source) {
  if (!isDeepStrictEqual(JSON.parse(source), value)) throw new Error('fixture source and structured values differ');
  const logical = file instanceof URL ? fileURLToPath(file) : resolve(file);
  const name = relative(repository, logical).split(sep).join('/');
  const layout = createFixtureSourceLayout(source);
  if (renderFixtureSourceBytes(value, layout).toString('utf8') !== source) throw new Error('fixture source ordering differs');
  const current = readProjectData(layoutFile);
  current[name] = layout;
  writeProjectData(file, value);
  writeProjectData(layoutFile, current);
  layouts = current;
}
