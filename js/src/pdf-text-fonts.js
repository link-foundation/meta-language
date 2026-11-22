import { readFileSync } from 'node:fs';

const utf16 = new TextDecoder('utf-16be', { ignoreBOM: true });
const encoders = new TextEncoder();
const table = name => readFileSync(new URL(`./data/${name}.lino`, import.meta.url), 'utf8').trimEnd().split('\n')
  .map(line => line.slice(1, -1).split(' '));
const scalar = value => String.fromCodePoint(Number.parseInt(value, 16));
let glyphNames;
let predefinedEncodings;

export function pdfGlyphName(name) {
  glyphNames ??= new Map(table('pdf-glyph-map').map(([glyph, ...values]) => [glyph, values.map(scalar).join('')]));
  return name.split('.', 1)[0].split('_').map(component => {
    if (glyphNames.has(component)) return glyphNames.get(component);
    if (/^uni(?:[0-9A-F]{4})+$/u.test(component)) return decodeUtf16(component.slice(3));
    if (/^u[0-9A-F]{4,6}$/u.test(component) && Number.parseInt(component.slice(1), 16) <= 0x10ffff) {
      const character = scalar(component.slice(1));
      if (!/^[\uD800-\uDFFF]$/u.test(character)) return character;
    }
    return '\ufffd';
  }).join('');
}

export function pdfPredefinedEncoding(name) {
  predefinedEncodings ??= new Map(table('pdf-font-encodings').map(([encoding, ...values]) => [encoding,
    values.map(value => value === '-' ? '\ufffd' : value.split('-').map(scalar).join(''))]));
  return predefinedEncodings.get(name) ?? predefinedEncodings.get('StandardEncoding');
}

const escapeCharacters = new Map([['n', 10], ['r', 13], ['t', 9], ['b', 8], ['f', 12]]);

export function pdfStringBytes(spelling) {
  if (spelling.startsWith('<')) {
    let hex = spelling.slice(1, -1).replace(/[\x00\t\n\f\r ]/gu, '');
    if (hex.length % 2) hex += '0';
    return Buffer.from(hex, 'hex');
  }
  const bytes = [];
  for (let index = 1; index < spelling.length - 1; index += 1) {
    let character = spelling[index];
    if (character === '\\') {
      character = spelling[++index];
      if (escapeCharacters.has(character)) { bytes.push(escapeCharacters.get(character)); continue; }
      if (character === '\n' || character === '\r') {
        if (character === '\r' && spelling[index + 1] === '\n') index += 1;
        continue;
      }
      if (character >= '0' && character <= '7') {
        let octal = character;
        while (octal.length < 3 && spelling[index + 1] >= '0' && spelling[index + 1] <= '7') octal += spelling[++index];
        bytes.push(Number.parseInt(octal, 8) & 255);
        continue;
      }
    } else if (character === '\r' || character === '\n') {
      if (character === '\r' && spelling[index + 1] === '\n') index += 1;
      bytes.push(10);
      continue;
    }
    const value = spelling.codePointAt(index);
    if (value <= 255) bytes.push(value);
    else {
      bytes.push(...encoders.encode(String.fromCodePoint(value)));
      if (value > 65535) index += 1;
    }
  }
  return Uint8Array.from(bytes);
}

function decodeUtf16(hex) {
  return utf16.decode(Buffer.from(hex, 'hex'));
}

const maximumCmapMappings = 65536;

// PDF 2.0, section 9.10.3. Compact bfrange tokens expand into a bounded
// lookup table; the source widths disambiguate variable-length character codes.
export function readPdfUnicodeMap(source) {
  const mapping = new Map();
  const spaces = new Map();
  const tokens = source.matchAll(/%[^\r\n]*|<([0-9a-f\x00\t\n\f\r ]+)>|[\[\]]|(?:begin|end)(?:codespacerange|bfchar|bfrange)/giu);
  function next() {
    let token = tokens.next().value;
    while (token?.[0].startsWith('%')) token = tokens.next().value;
    return token?.[1] === undefined ? token?.[0] : token[1].replace(/[\x00\t\n\f\r ]/gu, '').toUpperCase();
  }
  const code = value => /^(?:[0-9A-F]{2}){1,4}$/u.test(value ?? '');
  const unicode = value => /^(?:[0-9A-F]{4})+$/u.test(value ?? '');
  let remainingRangeWork = maximumCmapMappings;
  let token = next();
  while (token !== undefined && mapping.size < maximumCmapMappings) {
    const kind = token.toLowerCase();
    token = next();
    if (kind === 'begincodespacerange') {
      while (code(token)) {
        const upper = next();
        if (!code(upper) || upper.length !== token.length) { token = upper; break; }
        const ranges = spaces.get(token.length) ?? [];
        if (ranges.length < 256) ranges.push([token, upper]);
        spaces.set(token.length, ranges);
        token = next();
      }
    } else if (kind === 'beginbfchar') {
      while (code(token) && mapping.size < maximumCmapMappings) {
        const result = next();
        if (!unicode(result)) { token = result; break; }
        if (result.length <= 256) mapping.set(token, decodeUtf16(result));
        token = next();
      }
    } else if (kind === 'beginbfrange') {
      while (code(token) && mapping.size < maximumCmapMappings) {
        const upper = next();
        if (!code(upper) || upper.length !== token.length) { token = upper; break; }
        const length = token.length;
        const low = Number.parseInt(token, 16);
        const high = Math.min(Number.parseInt(upper, 16), low + Math.min(remainingRangeWork, maximumCmapMappings - mapping.size) - 1);
        remainingRangeWork -= Math.max(0, high - low + 1);
        const result = next();
        if (result === '[') {
          for (let value = low; value <= high; value += 1) {
            token = next();
            if (!unicode(token)) break;
            if (token.length <= 256) mapping.set(codeKey(value, length), decodeUtf16(token));
          }
          while (token !== undefined && token !== ']' && unicode(token)) token = next();
        } else if (unicode(result) && result.length <= 256) {
          const first = BigInt(`0x${result}`);
          for (let value = low; value <= high; value += 1) {
            const hexadecimal = (first + BigInt(value - low)).toString(16).padStart(result.length, '0');
            if (hexadecimal.length === result.length) mapping.set(codeKey(value, length), decodeUtf16(hexadecimal));
          }
        }
        token = next();
      }
    }
  }
  const widths = [...new Set([...spaces.keys(), ...Array.from(mapping.keys(), key => key.length)])].sort((a, b) => b - a);
  return { mapping, spaces, widths };
}

function codeKey(value, length) {
  return value.toString(16).toUpperCase().padStart(length, '0');
}

export function decodePdfFontText(bytes, font) {
  if (!font) return Buffer.from(bytes).toString('latin1');
  const { cmap, encoding, differences, composite } = font;
  const output = [];
  const hexadecimal = Buffer.from(bytes).toString('hex').toUpperCase();
  for (let position = 0; position < hexadecimal.length;) {
    const mappedLength = cmap?.widths.find(width => cmap.mapping.has(hexadecimal.slice(position, position + width)) &&
      position + width <= hexadecimal.length);
    if (mappedLength) {
      output.push(cmap.mapping.get(hexadecimal.slice(position, position + mappedLength)));
      position += mappedLength;
      continue;
    }
    const width = cmap?.widths.find(width => {
      const code = hexadecimal.slice(position, position + width);
      return code.length === width && cmap.spaces.get(width)?.some(([first, last]) => first <= code && code <= last);
    }) ?? (composite ? 4 : 2);
    const value = Number.parseInt(hexadecimal.slice(position, position + width), 16);
    output.push(composite ? '\ufffd' : differences.get(value) ?? encoding[value] ?? '\ufffd');
    position += width;
  }
  return output.join('');
}
