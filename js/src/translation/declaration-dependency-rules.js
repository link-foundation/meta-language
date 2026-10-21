// Candidate dependency scheduling only. Actual complete bodies, lexical
// owners, types and target emission remain mandatory in the host checker.
import { readRequiredNumberFact } from './fact-column-rules.js';

/** @param {number[]} values @param {number} value @returns {boolean} */
function containsDeclarationDependency(values, value) {
  for (const previous of values) if (previous === value) return true;
  return false;
}

/** Dense adjacency facts must describe exactly their actual ordered edges.
 * @param {number[]} starts @param {number[]} counts @param {number[]} targets
 * @returns {boolean} */
export function acceptDeclarationDependencyFacts(starts, counts, targets) {
  if (starts.length !== counts.length) return false;
  let offset = 0;
  for (let node = 0; node < starts.length; node += 1) {
    const start = readRequiredNumberFact(starts, node);
    const count = readRequiredNumberFact(counts, node);
    if (start !== offset || count < 0 || count % 1 !== 0 || count > targets.length - offset) return false;
    offset += count;
  }
  if (offset !== targets.length) return false;
  for (const target of targets) {
    if (!(target >= 0 && target < starts.length) || target % 1 !== 0) return false;
  }
  return true;
}

/** Iterative two-pass SCC partition. Each result is its component's least
 * original node index. No recursion, matrix, source rewriting or callbacks.
 * The reverse walk scans the retained edges, so memory is O(V+E), and the
 * simple immutable fact implementation takes O(V*(V+E)) comparisons.
 * @param {number[]} starts @param {number[]} counts @param {number[]} targets
 * @returns {number[]} */
export function readDeclarationDependencyComponents(starts, counts, targets) {
  if (!acceptDeclarationDependencyFacts(starts, counts, targets)) {
    throw new RangeError('invalid declaration dependency facts');
  }
  let visited = [];
  for (let node = 0; node < starts.length; node += 1) visited.push(0);
  let finished = [];
  for (let root = 0; root < starts.length; root += 1) {
    if (readRequiredNumberFact(visited, root) === 1) continue;
    let pending = [root];
    while (pending.length > 0) {
      const instruction = readRequiredNumberFact(pending, pending.length - 1);
      pending = pending.slice(0, pending.length - 1);
      if (instruction < 0) finished.push(-instruction - 1);
      else if (readRequiredNumberFact(visited, instruction) === 0) {
        visited = visited.slice(0, instruction).concat([1]).concat(visited.slice(instruction + 1));
        pending.push(-instruction - 1);
        const start = readRequiredNumberFact(starts, instruction);
        let edge = start + readRequiredNumberFact(counts, instruction);
        while (edge > start) {
          edge -= 1;
          const target = readRequiredNumberFact(targets, edge);
          if (readRequiredNumberFact(visited, target) === 0) pending.push(target);
        }
      }
    }
  }
  let assigned = [];
  for (let node = 0; node < starts.length; node += 1) assigned.push(0);
  let vertices = [];
  let leaders = [];
  let position = finished.length;
  while (position > 0) {
    position -= 1;
    const root = readRequiredNumberFact(finished, position);
    if (readRequiredNumberFact(assigned, root) === 1) continue;
    let pending = [root];
    let members = [];
    let leader = root;
    let at = 0;
    while (at < pending.length) {
      const current = readRequiredNumberFact(pending, at);
      at += 1;
      if (readRequiredNumberFact(assigned, current) === 1) continue;
      assigned = assigned.slice(0, current).concat([1]).concat(assigned.slice(current + 1));
      members.push(current);
      if (current < leader) leader = current;
      for (let source = 0; source < starts.length; source += 1) {
        if (readRequiredNumberFact(assigned, source) === 1) continue;
        const start = readRequiredNumberFact(starts, source);
        const end = start + readRequiredNumberFact(counts, source);
        for (let edge = start; edge < end; edge += 1) {
          if (readRequiredNumberFact(targets, edge) === current && !containsDeclarationDependency(pending, source)) pending.push(source);
        }
      }
    }
    for (const member of members) {
      vertices.push(member);
      leaders.push(leader);
    }
  }
  let result = [];
  for (let node = 0; node < starts.length; node += 1) {
    for (let at = 0; at < vertices.length; at += 1) {
      if (readRequiredNumberFact(vertices, at) === node) result.push(readRequiredNumberFact(leaders, at));
    }
  }
  return result;
}

/** Initial profile changes only ordinary synchronous hoisted declarations.
 * @param {number} count @param {boolean} ordinary @param {boolean} unique
 * @returns {boolean} */
export function acceptRecursiveDeclarationGroup(count, ordinary, unique) {
  return count > 1 && count % 1 === 0 && ordinary && unique;
}

/** A group publishes only after the whole original group and every original
 * member have actually passed; counts come from actual checked owners.
 * @param {number} members @param {number} declarations @param {number} signatures
 * @param {boolean} flat @param {boolean} emitted @param {boolean} matching
 * @returns {boolean} */
export function acceptRecursiveDeclarationPublication(members, declarations, signatures, flat, emitted, matching) {
  return members > 1 && members % 1 === 0 && declarations === members && signatures === members && flat && emitted && matching;
}

/** Module effects and unknown binders cannot be silently omitted by either
 * new atomic grouping or an existing partial-scope retry. A wider form needs
 * the existing parser/checker to check the COMPLETE original module first.
 * @param {string[]} forms @param {boolean} completeChecked @returns {boolean} */
export function acceptDeclarationModuleScope(forms, completeChecked) {
  if (completeChecked) return true;
  for (const form of forms) {
    if (form !== 'comment' && form !== 'named function declaration'
      && form !== 'named import' && form !== 'primitive literal binding'
      && form !== 'empty class declaration') return false;
  }
  return true;
}

/** A signature describes a binding only after the actual module surface is
 * certified and that original name has exactly one actual declaration/import
 * owner. An isolated checked body alone cannot supply module ownership.
 * @param {boolean} scopeKnown @param {number} ownerCount @returns {boolean} */
export function acceptDeclarationSignatureBinding(scopeKnown, ownerCount) {
  return scopeKnown && ownerCount === 1;
}

// Ownership and publication of complete actual source-generated helpers.
// The existing fact-column import and six decisions remain byte-exact.
// No code name or generated:true marker is an ownership proof.

/** Actual roots own disjoint complete original UTF16 intervals. Roles are
 * 0=the original named fn, 1=generated fn, 2=generated non-output data.
 * Unknown kinds, module paths, absent spans or output carriers map to -1.
 * A refusal returns []; success returns one original owner per declaration.
 * @param {number[]} rootStarts @param {number[]} rootEnds
 * @param {number[]} declarationStarts @param {number[]} declarationEnds
 * @param {number[]} roles @returns {number[]} */
export function readRecursiveDeclarationSourceOwners(rootStarts, rootEnds, declarationStarts, declarationEnds, roles) {
  if (rootStarts.length < 1 || rootStarts.length !== rootEnds.length
    || declarationStarts.length !== declarationEnds.length || declarationStarts.length !== roles.length) return [];
  let previousEnd = -1;
  for (let root = 0; root < rootStarts.length; root += 1) {
    const start = readRequiredNumberFact(rootStarts, root);
    const end = readRequiredNumberFact(rootEnds, root);
    if (!(start >= 0 && start % 1 === 0 && end > start && end % 1 === 0
      && end <= 9007199254740991 && start >= previousEnd)) return [];
    previousEnd = end;
  }
  let owners = [];
  let roots = [];
  for (let declaration = 0; declaration < roles.length; declaration += 1) {
    const start = readRequiredNumberFact(declarationStarts, declaration);
    const end = readRequiredNumberFact(declarationEnds, declaration);
    const role = readRequiredNumberFact(roles, declaration);
    if (!(start >= 0 && start % 1 === 0 && end > start && end % 1 === 0)
      || role !== 0 && role !== 1 && role !== 2) return [];
    let owner = -1;
    for (let root = 0; root < rootStarts.length; root += 1) {
      const rootStart = readRequiredNumberFact(rootStarts, root);
      const rootEnd = readRequiredNumberFact(rootEnds, root);
      if (start >= rootStart && end <= rootEnd) {
        if (owner >= 0) return [];
        owner = root;
        if (role === 0 && (start !== rootStart || end !== rootEnd)) return [];
        if (role !== 0 && (start <= rootStart || end >= rootEnd)) return [];
      }
    }
    if (owner < 0) return [];
    if (role === 0) {
      if (containsDeclarationDependency(roots, owner)) return [];
      roots = roots.concat([owner]);
    }
    owners = owners.concat([owner]);
  }
  return roots.length === rootStarts.length ? owners : [];
}

/** Edges are actual checked declaration references, including calls,
 * function references, constructor/pattern data and recursively nested data
 * type references. Literal strings are never edges. Every auxiliary must be
 * reachable from its own original root without traversing another root, and
 * no declaration may reference an auxiliary owned by a different root.
 * @param {number[]} owners @param {number[]} roles
 * @param {number[]} starts @param {number[]} counts @param {number[]} targets
 * @returns {boolean} */
export function acceptRecursiveDeclarationAuxiliaryClosure(owners, roles, starts, counts, targets) {
  if (owners.length !== roles.length || owners.length !== starts.length
    || !acceptDeclarationDependencyFacts(starts, counts, targets)) return false;
  let rootOwners = [];
  for (let declaration = 0; declaration < roles.length; declaration += 1) {
    const owner = readRequiredNumberFact(owners, declaration);
    const role = readRequiredNumberFact(roles, declaration);
    if (!(owner >= 0 && owner % 1 === 0 && owner < roles.length)
      || role !== 0 && role !== 1 && role !== 2) return false;
    if (role === 0) {
      if (containsDeclarationDependency(rootOwners, owner)) return false;
      rootOwners = rootOwners.concat([owner]);
    }
  }
  if (rootOwners.length < 1) return false;
  for (const owner of owners) if (!containsDeclarationDependency(rootOwners, owner)) return false;
  for (let source = 0; source < roles.length; source += 1) {
    const first = readRequiredNumberFact(starts, source);
    const last = first + readRequiredNumberFact(counts, source);
    for (let at = first; at < last; at += 1) {
      const target = readRequiredNumberFact(targets, at);
      if (readRequiredNumberFact(roles, target) !== 0
        && readRequiredNumberFact(owners, source) !== readRequiredNumberFact(owners, target)) return false;
    }
  }
  for (let root = 0; root < roles.length; root += 1) {
    if (readRequiredNumberFact(roles, root) !== 0) continue;
    const owner = readRequiredNumberFact(owners, root);
    let pending = [root];
    let at = 0;
    while (at < pending.length) {
      const current = readRequiredNumberFact(pending, at);
      at += 1;
      const first = readRequiredNumberFact(starts, current);
      const last = first + readRequiredNumberFact(counts, current);
      for (let edge = first; edge < last; edge += 1) {
        const target = readRequiredNumberFact(targets, edge);
        if (readRequiredNumberFact(owners, target) === owner
          && !containsDeclarationDependency(pending, target)) pending = pending.concat([target]);
      }
    }
    for (let declaration = 0; declaration < roles.length; declaration += 1) {
      if (readRequiredNumberFact(owners, declaration) === owner
        && !containsDeclarationDependency(pending, declaration)) return false;
    }
  }
  return true;
}

/** An actual per-owner inventory retains every fn/data declaration, ordered
 * layout and checked declaration reference under source-owner identities.
 * No source spelling, body or target is rewritten by this comparison.
 * @param {number[]} whole @param {number[]} member @returns {boolean} */
export function acceptRecursiveDeclarationAuxiliaryCorrespondence(whole, member) {
  if (!whole.length || whole.length !== member.length) return false;
  for (let at = 0; at < whole.length; at += 1) {
    if (readRequiredNumberFact(whole, at) !== readRequiredNumberFact(member, at)) return false;
  }
  return true;
}

/** Block rows are [declaration,mappingStart,mappingCount,definitionIndex].
 * Mapping rows are [declaration,position,targetIdentity], position zero is
 * fn/data itself and positions 1..N are the actual ordered constructors.
 * Every complete emitted block and mapping is covered exactly once. Targets
 * are interned exact names including qualification; the first conservative
 * profile refuses equal names even in otherwise disjoint Rust namespaces.
 * @param {number[]} roles @param {number[]} constructorCounts
 * @param {number[]} blocks @param {number[]} mappings
 * @param {number[]} forbiddenTargets @returns {boolean} */
export function acceptRecursiveDeclarationOwnedEmission(roles, constructorCounts, blocks, mappings, forbiddenTargets) {
  if (!roles.length || roles.length !== constructorCounts.length || blocks.length !== roles.length * 4
    || mappings.length % 3 !== 0) return false;
  let declarations = [];
  let targets = [];
  let mappingOffset = 0;
  let occupied = [];
  for (const target of forbiddenTargets) {
    if (!(target >= 0 && target % 1 === 0 && target <= 9007199254740991)
      || containsDeclarationDependency(occupied, target)) return false;
    occupied = occupied.concat([target]);
  }
  for (let block = 0; block < roles.length; block += 1) {
    const declaration = readRequiredNumberFact(blocks, block * 4);
    const start = readRequiredNumberFact(blocks, block * 4 + 1);
    const count = readRequiredNumberFact(blocks, block * 4 + 2);
    const index = readRequiredNumberFact(blocks, block * 4 + 3);
    if (!(declaration >= 0 && declaration < roles.length && declaration % 1 === 0)
      || containsDeclarationDependency(declarations, declaration) || start !== mappingOffset || index !== block) return false;
    declarations = declarations.concat([declaration]);
    const role = readRequiredNumberFact(roles, declaration);
    const constructors = readRequiredNumberFact(constructorCounts, declaration);
    if (role !== 0 && role !== 1 && role !== 2
      || !(constructors >= 0 && constructors % 1 === 0)
      || role !== 2 && constructors !== 0 || count !== constructors + 1
      || count > mappings.length / 3 - start) return false;
    for (let position = 0; position < count; position += 1) {
      const at = (start + position) * 3;
      const target = readRequiredNumberFact(mappings, at + 2);
      if (readRequiredNumberFact(mappings, at) !== declaration
        || readRequiredNumberFact(mappings, at + 1) !== position
        || !(target >= 0 && target % 1 === 0 && target <= 9007199254740991)
        || containsDeclarationDependency(targets, target) || containsDeclarationDependency(forbiddenTargets, target)) return false;
      targets = targets.concat([target]);
    }
    mappingOffset += count;
  }
  return mappingOffset === mappings.length / 3;
}

/** Independent auxiliary publication profile; the original flat rule stays
 * exact. Proof cells are source ownership, closed references, complete actual
 * emission, member correspondence, signatures/root targets, exact original
 * source, and safe actual module scope. No cell is optional or provisional.
 * @param {number} members @param {number} declarations @param {number} signatures
 * @param {number} completed @param {number[]} proofs @returns {boolean} */
export function acceptRecursiveDeclarationOwnedPublication(members, declarations, signatures, completed, proofs) {
  if (!(members > 1 && members % 1 === 0 && members <= 9007199254740991)
    || !(declarations > members && declarations % 1 === 0 && declarations <= 9007199254740991)
    || signatures !== members || completed !== members || proofs.length !== 7) return false;
  for (const proof of proofs) if (proof !== 1) return false;
  return true;
}

// Separate deferred-definition scope profile. All original eleven decisions
// above remain complete and byte-exact; no body or class becomes translated.
import { readRequiredTextFact } from './fact-column-rules.js';

/** @param {string[]} terms @param {number[]} parents @param {number} parent @returns {number[]} */
function readDeferredDeclarationChildren(terms, parents, parent) {
  let children = [];
  for (let at = 0; at < terms.length; at += 1) {
    const term = readRequiredTextFact(terms, at);
    if (readRequiredNumberFact(parents, at) === parent && term !== 'comment' && term !== 'whitespace') children.push(at);
  }
  return children;
}

/** Only actual simple unescaped identifier parameters enter this finite
 * profile. Defaults/rest/patterns remain obligations, even though creating
 * their function is ordinarily deferred.
 * @param {string[]} terms @param {number[]} parents @param {string[]} texts
 * @param {number} owner @returns {boolean} */
function acceptDeferredDeclarationParameters(terms, parents, texts, owner) {
  let names = [];
  for (const child of readDeferredDeclarationChildren(terms, parents, owner)) {
    const term = readRequiredTextFact(terms, child);
    if (term === '(' || term === ')' || term === ',') continue;
    if (term !== 'identifier') return false;
    const text = readRequiredTextFact(texts, child);
    if (text.length === 0 || text.includes('\\') || text === 'eval' || text === 'arguments') return false;
    for (const name of names) if (name === text) return false;
    names.push(text);
  }
  return true;
}

/** Complete actual native CST facts, preorder, one parent per node. The only
 * result is [1,class name row], [2,const name row], or refusal []. Parents and
 * source intervals are checked before interpretation. Bodies stay complete;
 * body execution never occurs while these definitions are initialized.
 * @param {string[]} terms @param {string[]} fields @param {number[]} parents
 * @param {number[]} starts @param {number[]} ends @param {string[]} texts
 * @param {number[]} errors @param {number} sourceBytes @returns {number[]} */
export function readDeclarationDeferredBinding(terms, fields, parents, starts, ends, texts, errors, sourceBytes) {
  if (terms.length < 1 || fields.length !== terms.length || parents.length !== terms.length
    || starts.length !== terms.length || ends.length !== terms.length || texts.length !== terms.length
    || errors.length !== terms.length || !(sourceBytes >= 0 && sourceBytes % 1 === 0 && sourceBytes <= 9007199254740991)) return [];
  for (let at = 0; at < terms.length; at += 1) {
    const parent = readRequiredNumberFact(parents, at);
    const start = readRequiredNumberFact(starts, at), end = readRequiredNumberFact(ends, at);
    if (!(start >= 0 && start % 1 === 0 && end >= start && end % 1 === 0 && end <= sourceBytes)
      || readRequiredNumberFact(errors, at) !== 0) return [];
    if (at === 0) {
      if (parent !== -1 || readRequiredTextFact(terms, at) !== 'program' || end !== sourceBytes) return [];
    } else {
      if (!(parent >= 0 && parent % 1 === 0 && parent < at)) return [];
      if (start < readRequiredNumberFact(starts, parent) || end > readRequiredNumberFact(ends, parent)) return [];
    }
  }
  const top = readDeferredDeclarationChildren(terms, parents, 0);
  if (top.length !== 1) return [];
  let declaration = readRequiredNumberFact(top, 0);
  if (readRequiredTextFact(terms, declaration) === 'export_statement') {
    let selected = -1, exports = 0;
    for (const child of readDeferredDeclarationChildren(terms, parents, declaration)) {
      const term = readRequiredTextFact(terms, child);
      if (term === 'export') exports += 1;
      else if (term === 'class_declaration' || term === 'lexical_declaration') {
        if (selected >= 0) return [];
        selected = child;
      } else if (term !== ';') return [];
    }
    if (exports !== 1 || selected < 0) return [];
    declaration = selected;
  }
  const kind = readRequiredTextFact(terms, declaration);
  let name = -1;
  if (kind === 'lexical_declaration') {
    let constant = 0, binding = -1;
    for (const child of readDeferredDeclarationChildren(terms, parents, declaration)) {
      const term = readRequiredTextFact(terms, child);
      if (term === 'const') constant += 1;
      else if (term === 'variable_declarator') {
        if (binding >= 0) return [];
        binding = child;
      } else if (term !== ';') return [];
    }
    if (constant !== 1 || binding < 0) return [];
    let value = -1, equals = 0;
    for (const child of readDeferredDeclarationChildren(terms, parents, binding)) {
      const term = readRequiredTextFact(terms, child), field = readRequiredTextFact(fields, child);
      if (term === 'identifier' && field === 'name' && name < 0) name = child;
      else if ((term === 'arrow_function' || term === 'function_expression') && field === 'value' && value < 0) value = child;
      else if (term === '=') equals += 1;
      else return [];
    }
    if (name < 0 || value < 0 || equals !== 1) return [];
    let parameters = 0, body = 0;
    for (const child of readDeferredDeclarationChildren(terms, parents, value)) {
      const term = readRequiredTextFact(terms, child), field = readRequiredTextFact(fields, child);
      if (field === 'parameters' && term === 'formal_parameters') {
        if (!acceptDeferredDeclarationParameters(terms, parents, texts, child)) return [];
        parameters += 1;
      } else if (field === 'parameter' && term === 'identifier') {
        const text = readRequiredTextFact(texts, child);
        if (text.length === 0 || text.includes('\\') || text === 'eval' || text === 'arguments') return [];
        parameters += 1;
      } else if (field === 'body') body += 1;
      else if (term === 'function' || term === '=>' || field === 'name' && term === 'identifier') continue;
      else return [];
    }
    if (parameters !== 1 || body !== 1) return [];
  } else if (kind === 'class_declaration') {
    let body = -1, classes = 0;
    for (const child of readDeferredDeclarationChildren(terms, parents, declaration)) {
      const term = readRequiredTextFact(terms, child), field = readRequiredTextFact(fields, child);
      if (term === 'class') classes += 1;
      else if (term === 'identifier' && field === 'name' && name < 0) name = child;
      else if (term === 'class_body' && field === 'body' && body < 0) body = child;
      else if (term !== ';') return [];
    }
    if (classes !== 1 || name < 0 || body < 0) return [];
    let constructors = 0;
    for (const member of readDeferredDeclarationChildren(terms, parents, body)) {
      const term = readRequiredTextFact(terms, member);
      if (term === '{' || term === '}' || term === ';') continue;
      if (term !== 'method_definition') return [];
      let methodName = -1, parameters = 0, methodBody = 0;
      for (const child of readDeferredDeclarationChildren(terms, parents, member)) {
        const part = readRequiredTextFact(terms, child), field = readRequiredTextFact(fields, child);
        if (part === 'property_identifier' && field === 'name' && methodName < 0) methodName = child;
        else if (part === 'formal_parameters' && field === 'parameters') {
          if (!acceptDeferredDeclarationParameters(terms, parents, texts, child)) return [];
          parameters += 1;
        } else if (part === 'statement_block' && field === 'body') methodBody += 1;
        else return [];
      }
      if (methodName < 0 || parameters !== 1 || methodBody !== 1) return [];
      const text = readRequiredTextFact(texts, methodName);
      if (text.length === 0 || text.includes('\\')) return [];
      if (text === 'constructor') constructors += 1;
      if (constructors > 1) return [];
    }
  } else return [];
  const actualName = readRequiredTextFact(texts, name);
  if (actualName.length === 0 || actualName.includes('\\') || actualName === 'eval' || actualName === 'arguments') return [];
  return [kind === 'class_declaration' ? 1 : 2, name];
}

/** These new real forms do not masquerade as an old function or empty class.
 * Complete source classifiers and actual unique-owner counts are mandatory
 * at the host boundary before this operation is called.
 * @param {string[]} forms @returns {boolean} */
export function acceptDeclarationDeferredModuleScope(forms) {
  for (const form of forms) {
    if (form !== 'comment' && form !== 'named function declaration' && form !== 'named import'
      && form !== 'primitive literal binding' && form !== 'empty class declaration'
      && form !== 'deferred named class declaration' && form !== 'deferred function value binding') return false;
  }
  return true;
}
