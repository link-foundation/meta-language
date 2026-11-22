import {
  readOrdinarySourceArchiveField,
  readSourceDefinitionCatalogByteLimit,
  readSourceDefinitionCatalogNodeLimit,
} from './source-definition-catalog-rules.js';

// A checked source module owns a bounded traversal unit. The same declaration
// and lexical binding tables remain visible while processing every unit.
const unitLimit = readSourceDefinitionCatalogNodeLimit(false);
export const ordinarySourceArchiveNodeLimit = readSourceDefinitionCatalogNodeLimit(true);
export const ordinarySourceArchiveByteLimit = readSourceDefinitionCatalogByteLimit(true);
export const ordinarySourceArchiveField = readOrdinarySourceArchiveField();

export function readTypedProgramOwnerTraversal(program, sourceOwners, message) {
  const root = undefined;
  const entries = sourceOwners && new WeakMap();
  if (entries) {
    for (const {name, path} of sourceOwners.rows) entries.set(program.declarations.get(name), path);
  }
  const units = new Map();
  let owner = root;
  let total = 0;

  function enter(path) {
    owner = path;
  }

  function visit(path = owner) {
    const count = (units.get(path) ?? 0) + 1;
    units.set(path, count);
    if (count > unitLimit || ++total > (entries ? ordinarySourceArchiveNodeLimit : unitLimit)) {
      throw new TypeError(path === root ? message : message + ' in ' + path);
    }
  }

  function childOwner(container, child, inherited) {
    if (entries && (container === program.items || container === program.declarations)) {
      return entries.get(child) ?? inherited;
    }
    return inherited;
  }

  return {enter, visit, childOwner, ownerOf: value => entries?.get(value)};
}
