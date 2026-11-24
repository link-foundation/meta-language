import { renameFunction } from './ir.js';
import { collectSourceIdentifierTexts, readSourceIdentifierSupply } from './source-identifiers.js';
import { acceptFormalLocalCellElement } from './callable-emission-rules.js';

const metadata = new Set(['type', 'domain', 'from', 'to', 'span']);
const effectNodes = new Set(['cellAllocate', 'cellRead', 'cellWrite']);
const blocked = Symbol('escaping or conditional cell state');

function containsCell(node) {
  const pending = [node];
  const seen = new WeakSet();
  while (pending.length) {
    const current = pending.pop();
    if (!current || typeof current !== 'object' || seen.has(current)) continue;
    seen.add(current);
    if (current.type?.kind === 'cell' || effectNodes.has(current.k)) return true;
    for (const [field, value] of Object.entries(current)) if (!metadata.has(field)) pending.push(value);
  }
  return false;
}

function lowerLocalCells(entry) {
  const { params, body } = renameFunction(entry, name => name, new Set());
  const prefix = readSourceIdentifierSupply(collectSourceIdentifierTexts({ params, body }));
  const state = new Map();
  const bindings = [];
  let next = 0;

  function cellName(cell) {
    if (cell.k !== 'var' || !state.has(cell.name)) throw blocked;
    return cell.name;
  }

  function bind(name, value, span) {
    bindings.push({ name, value, span });
    return { k: 'var', name, type: value.type, span };
  }

  function read(node) {
    if (!node || typeof node !== 'object') return node;
    if (node.k === 'cellRead') return state.get(cellName(node.cell));
    if (node.type?.kind === 'cell' || effectNodes.has(node.k)) throw blocked;
    // A closure has its own invocation point. Its state reads cannot be
    // substituted with the state at closure creation.
    if (node.k === 'lambda') {
      if (containsCell(node.body)) throw blocked;
      return node;
    }
    if (Array.isArray(node)) return node.map(read);
    return Object.fromEntries(Object.entries(node).map(([field, value]) =>
      [field, metadata.has(field) ? value : read(value)]));
  }

  function lower(node) {
    if (node.k === 'let') {
      if (node.value.k === 'cellAllocate') {
        if (!acceptFormalLocalCellElement(node.value.type.element.kind)) throw blocked;
        state.set(node.name, bind(node.name, lower(node.value.value), node.span));
        const result = lower(node.body);
        state.delete(node.name);
        return result;
      }
      bind(node.name, lower(node.value), node.span);
      return lower(node.body);
    }
    if (node.k === 'cellWrite') {
      const name = cellName(node.cell);
      const value = lower(node.value);
      state.set(name, bind(prefix + 'localCell' + (++next), value, node.span));
      return lower(node.body);
    }
    return read(node);
  }

  try {
    let result = lower(body);
    for (const binding of bindings.reverse()) result = { k: 'let', ...binding, body: result, type: result.type };
    return { ...entry, params, body: result };
  } catch (error) {
    if (error !== blocked) throw error;
    return entry;
  }
}

// The checked cell must be allocated by this declaration. Its writes must
// occur in ordered let initializers or tails and no reference can escape.
export function prepareFormalLocalCells(program) {
  if (program.sourceLanguage !== 'JavaScript') return program;
  const declarations = new Map(program.declarations);
  let changed = false;
  for (const [name, entry] of program.declarations) {
    if (entry.k !== 'fn' || !containsCell(entry.body)) continue;
    const lowered = lowerLocalCells(entry);
    if (lowered === entry) continue;
    declarations.set(name, lowered);
    changed = true;
  }
  if (!changed) return program;
  const items = entries => entries.map(entry => entry.k === 'module'
    ? { ...entry, items: items(entry.items) }
    : declarations.get(entry.fullName) ?? entry);
  return { ...program, declarations, items: items(program.items) };
}
