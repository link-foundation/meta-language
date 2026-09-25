import { canonicalLanguageName } from './language-catalog.js';
import { ByteRange, Point, SourceSpan } from './primitives.js';
import { parseProgrammingLanguage } from './programming-language-parser.js';

const TXT_LANGUAGE = 'txt';

const textEncoder = new TextEncoder();
const textDecoder = new TextDecoder();

/// Region detection strategy for mixed-language parsing.
export const RegionDetectionPolicy = Object.freeze({
  NameDriven: 'NameDriven',
  ContentDriven: 'ContentDriven',
  Both: 'Both',
});

/// Embedded region discovered inside a mixed-language document.
export class EmbeddedRegion {
  constructor(language, span) {
    this._language = language;
    this._span = span;
  }

  language() {
    return this._language;
  }

  span() {
    return this._span;
  }
}

/**
 * Detects the embedded-language regions of a document from its host grammar
 * CST. The host is parsed with its registered grammar; see
 * `detectEmbeddedRegionsInTree` for the regions each host delimits.
 */
export function detectEmbeddedRegions(text, language, policy = RegionDetectionPolicy.Both) {
  const host = canonicalLanguageName(language);
  if (host === TXT_LANGUAGE) {
    return [regionFor(text, TXT_LANGUAGE, 0, byteLength(text))];
  }
  if (host !== 'HTML' && host !== 'Markdown') {
    return [];
  }
  return detectEmbeddedRegionsInTree(parseProgrammingLanguage(text, host).tree, text, host, policy);
}

/**
 * Embedded regions a host grammar CST delimits, in source order. Mirrors
 * `detect_embedded_regions` in rust/src/mixed_regions.rs.
 *
 * - HTML: the `raw_text` of a `script_element` in the language its `type`
 *   attribute names (JavaScript when absent), the `raw_text` of a
 *   `style_element` as CSS, and the `attribute_value` of every `style`
 *   attribute as CSS.
 * - Markdown: the `code_fence_content` of a `fenced_code_block` in the
 *   language `policy` selects from its info-string `language` and content,
 *   every `html_block` as HTML, and every inline `html_tag` as HTML, spanning
 *   from an opening tag to the matching closing tag among its siblings.
 *
 * `tree` nodes carry `term`, `named`, `span` and `children`.
 */
export function detectEmbeddedRegionsInTree(tree, text, host, policy = RegionDetectionPolicy.Both) {
  const source = textEncoder.encode(text);
  const nodeText = (node) =>
    textDecoder.decode(source.subarray(node.span.byteRange.start, node.span.byteRange.end));
  const found = [];
  const add = (language, start, end) => found.push({ language, start, end });
  const visit = (node) => {
    if (host === 'HTML') {
      visitHtml(node, nodeText, add);
    } else if (host === 'Markdown') {
      visitMarkdown(node, nodeText, policy, add);
    }
    node.children.forEach(visit);
  };
  visit(tree);
  return found
    .map((region, index) => ({ ...region, index }))
    .sort((left, right) => left.start - right.start || left.index - right.index)
    .map(({ language, start, end }) => regionFor(text, language, start, end));
}

function visitHtml(node, nodeText, add) {
  const content = childOfKind(node, 'raw_text');
  if (node.term === 'script_element' && content) {
    const language = scriptLanguage(attributeValue(node, 'type', nodeText));
    if (language) add(language, content.span.byteRange.start, content.span.byteRange.end);
  } else if (node.term === 'style_element' && content) {
    const type = mimeEssence(attributeValue(node, 'type', nodeText));
    if (type === '' || type === 'text/css') {
      add('CSS', content.span.byteRange.start, content.span.byteRange.end);
    }
  } else if (node.term === 'attribute' && attributeName(node, nodeText) === 'style') {
    const value = attributeValueNode(node);
    if (value) add('CSS', value.span.byteRange.start, value.span.byteRange.end);
  }
}

function visitMarkdown(node, nodeText, policy, add) {
  if (node.term === 'fenced_code_block') {
    const content = childOfKind(node, 'code_fence_content');
    const tagNode = childOfKind(childOfKind(node, 'info_string') ?? { children: [] }, 'language');
    const language = content &&
      fenceLanguage(tagNode ? nodeText(tagNode) : '', nodeText(content), policy);
    if (language) add(language, content.span.byteRange.start, content.span.byteRange.end);
  } else if (node.term === 'html_block') {
    add('HTML', node.span.byteRange.start, node.span.byteRange.end);
  }
  const tags = node.children.filter((child) => child.term === 'html_tag');
  for (const [first, last] of pairHtmlTags(tags.map(nodeText))) {
    add('HTML', tags[first].span.byteRange.start, tags[last].span.byteRange.end);
  }
}

// Void elements never have a closing tag.
const HTML_VOID_ELEMENTS = new Set([
  'area', 'base', 'br', 'col', 'embed', 'hr', 'img', 'input', 'link', 'meta', 'source', 'track', 'wbr',
]);

/**
 * Groups sibling inline HTML tags into elements: an opening tag extends to its
 * matching closing tag (same name, nesting counted); a void, self-closing,
 * unmatched, comment, or declaration tag stands alone.
 */
function pairHtmlTags(texts) {
  const tags = texts.map((text) => {
    const match = /^<(\/?)([A-Za-z][A-Za-z0-9-]*)/u.exec(text);
    if (!match) return { name: null };
    const name = match[2].toLowerCase();
    const closing = match[1] === '/';
    return { name, closing, opening: !closing && !text.endsWith('/>') && !HTML_VOID_ELEMENTS.has(name) };
  });
  const groups = [];
  for (let index = 0; index < tags.length; index += 1) {
    let last = index;
    if (tags[index].opening) {
      let depth = 0;
      for (let next = index; next < tags.length; next += 1) {
        if (tags[next].name !== tags[index].name) continue;
        if (tags[next].opening) depth += 1;
        if (tags[next].closing) depth -= 1;
        if (depth === 0) {
          last = next;
          break;
        }
      }
    }
    groups.push([index, last]);
    index = last;
  }
  return groups;
}

// https://html.spec.whatwg.org/multipage/scripting.html#javascript-mime-type
const JAVASCRIPT_SCRIPT_TYPES = new Set([
  '', 'module', 'application/ecmascript', 'application/javascript', 'application/x-ecmascript',
  'application/x-javascript', 'text/ecmascript', 'text/javascript', 'text/javascript1.0',
  'text/javascript1.1', 'text/javascript1.2', 'text/javascript1.3', 'text/javascript1.4',
  'text/javascript1.5', 'text/jscript', 'text/livescript', 'text/x-ecmascript', 'text/x-javascript',
]);

/**
 * The language of a script element's content from its `type` attribute:
 * JavaScript for classic scripts and modules, JSON for import maps,
 * speculation rules and JSON types, otherwise the registered language the MIME
 * subtype names (such as text/typescript); `null` for other data blocks.
 */
export function scriptLanguage(type) {
  const essence = mimeEssence(type);
  if (JAVASCRIPT_SCRIPT_TYPES.has(essence)) return 'JavaScript';
  if (
    essence === 'importmap' ||
    essence === 'speculationrules' ||
    essence.endsWith('/json') ||
    essence.endsWith('+json')
  ) {
    return 'JSON';
  }
  const subtype = essence.slice(essence.indexOf('/') + 1).replace(/^x-/u, '');
  return essence.includes('/') ? canonicalLanguageName(subtype) ?? null : null;
}

function mimeEssence(type) {
  return (type ?? '').split(';')[0].trim().toLowerCase();
}

function fenceLanguage(tag, content, policy) {
  const named = tag.length > 0 ? canonicalLanguageName(tag) ?? tag : null;
  const sniffed = () => {
    const language = sniffLanguage(content);
    return language === null ? TXT_LANGUAGE : canonicalLanguageName(language) ?? language;
  };
  switch (policy) {
    case RegionDetectionPolicy.NameDriven:
      return named;
    case RegionDetectionPolicy.ContentDriven:
      return sniffed();
    case RegionDetectionPolicy.Both:
      return named ?? sniffed();
    default:
      return null;
  }
}

function childOfKind(node, kind) {
  return node.children.find((child) => child.term === kind);
}

function attributeName(attribute, nodeText) {
  const name = childOfKind(attribute, 'attribute_name');
  return name ? nodeText(name).toLowerCase() : undefined;
}

function attributeValueNode(attribute) {
  return childOfKind(attribute, 'attribute_value') ??
    childOfKind(childOfKind(attribute, 'quoted_attribute_value') ?? { children: [] }, 'attribute_value');
}

/** The value of an element's start-tag attribute, '' when valueless, or undefined. */
function attributeValue(element, name, nodeText) {
  const attribute = (childOfKind(element, 'start_tag')?.children ?? []).find(
    (child) => child.term === 'attribute' && attributeName(child, nodeText) === name,
  );
  if (!attribute) return undefined;
  const value = attributeValueNode(attribute);
  return value ? nodeText(value) : '';
}

export function sniffLanguage(content) {
  const trimmed = content.replace(/^\s+/u, '');
  const upper = trimmed.toUpperCase();

  if (trimmed.includes('fn main')) {
    return 'rust';
  }
  if (trimmed.startsWith('def ')) {
    return 'Python';
  }
  if (trimmed.startsWith('<')) {
    return 'HTML';
  }
  if (
    trimmed.includes('function ') ||
    trimmed.includes('const ') ||
    trimmed.includes('let ')
  ) {
    return 'JavaScript';
  }
  if (upper.startsWith('SELECT ')) {
    return 'sql-ansi';
  }
  return null;
}

function regionFor(text, language, start, end) {
  return new EmbeddedRegion(
    language,
    new SourceSpan(
      new ByteRange(start, end),
      pointAtByte(text, start),
      pointAtByte(text, end),
    ),
  );
}

function pointAtByte(text, byte) {
  let row = 0;
  let column = 0;
  let index = 0;

  for (const character of text) {
    if (index >= byte) {
      break;
    }
    const length = byteLength(character);
    if (character === '\n') {
      row += 1;
      column = 0;
    } else {
      // Columns count UTF-8 bytes, as tree-sitter points do.
      column += length;
    }
    index += length;
  }

  return new Point(row, column);
}

function byteLength(value) {
  return textEncoder.encode(value).length;
}
