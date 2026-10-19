// Shared typed data model, limits and scalar validation; no external parser.
import { LinkId, LinkMetadata, LinkType } from './primitives.js';

export const language = 'meta-language structured data';
export const terms = Object.freeze({ root: 'structured data', object: 'object value', array: 'array value', pair: 'key value pair',
  string: 'string value', units: 'Unicode code unit string', unit: 'Unicode code unit', number: 'number value', boolean: 'boolean value', null: 'null value' });
export const defaults = Object.freeze({ nodeLimit: 1_000_000, depthLimit: 256, byteLimit: 128 * 1024 * 1024 });
export const encoder = new TextEncoder();
export const fail = (message) => { throw new TypeError(`structured data: ${message}`); };
export const key = (id) => LinkId.from(id).asU64();
export const metadata = (kind, definition) => LinkMetadata.new().withLinkType(LinkType.Semantic)
  .withLanguage(language).withNamed(true).withTerm(terms[kind]).withDefinition(definition);


export function limitsOf(options) {
  const limits = { ...defaults, ...options };
  for (const name of Object.keys(defaults)) if (!Number.isSafeInteger(limits[name]) || limits[name] < 1) fail(`invalid ${name}`);
  return limits;
}

export function wellFormedString(value) {
  for (let index = 0; index < value.length; index += 1) {
    const unit = value.charCodeAt(index);
    if (unit >= 0xd800 && unit <= 0xdbff) {
      const following = value.charCodeAt(++index);
      if (!(following >= 0xdc00 && following <= 0xdfff)) return false;
    } else if (unit >= 0xdc00 && unit <= 0xdfff) return false;
  }
  return true;
}

