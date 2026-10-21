// Host transport only: the shared automatically translated rule interprets
// complete actual native CST facts. No alternate parser or source rewrite.
import {parseNative} from '../native-grammar-parser.js';
import {readDeclarationDeferredBinding} from './declaration-dependency-rules.js';

export function readDeferredDeclarationBinding(source) {
  const bytes = Buffer.from(source, 'utf8');
  const root = parseNative('native-javascript', source);
  const nodes = [], terms = [], fields = [], parents = [], starts = [], ends = [], texts = [], errors = [];
  const visit = (node, field, parent) => {
    const at = nodes.length;
    nodes.push(node);terms.push(node.term);fields.push(field ?? '');parents.push(parent);
    starts.push(node.start);ends.push(node.end);
    texts.push(node.children.length === 0 ? bytes.subarray(node.start, node.end).toString('utf8') : '');
    errors.push(node.isError || node.isMissing || node.hasError ? 1 : 0);
    for (const child of node.children) visit(child.node, child.field, at);
  };
  visit(root, '', -1);
  const selected = readDeclarationDeferredBinding(terms, fields, parents, starts, ends, texts, errors, bytes.length);
  if (selected.length !== 2) return null;
  const [kind, at] = selected;
  if ((kind !== 1 && kind !== 2) || !Number.isSafeInteger(at) || at < 0 || at >= nodes.length) return null;
  const node = nodes[at];
  if (node.term !== 'identifier' || node.children.length !== 0 || node.start < 0 || node.end > bytes.length || node.end <= node.start) return null;
  const name = bytes.subarray(node.start, node.end).toString('utf8');
  if (name !== texts[at]) return null;
  return {kind, name, nameRange: {start: node.start, end: node.end}, coordinateUnit: 'UTF-8 bytes'};
}
