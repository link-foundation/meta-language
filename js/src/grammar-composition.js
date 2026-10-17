// The mechanical authoring adapter for the existing grammar decorator hook.
// Selection execution and source mappings remain in the grammar runtime.
import { DecoratorError } from './decorators.js';
import { parseLinoStatements } from './lino-serialization.js';
import { parseLinksExpression, renderLinksExpression } from './grammar-links.js';
import { checkGrammarSelectionIncludedRangeData, checkGrammarSelectionSourceViewForm } from './grammar-feature-forms.js';
import { checkGrammarSelection } from './grammar-runtime/grammar-selection.js';
import { readGrammarCompositionFieldOrder, readGrammarCompositionContextIssue,
  acceptGrammarCompositionCandidateProfile, readGrammarCompositionCandidateIssue,
  readGrammarCompositionDisposition } from './translation/grammar-rule-composition-rules.js';

const refuse = message => { throw new DecoratorError('grammar composition: ' + message); };

// Descriptor checks adapt host storage; field roles come from the shared
// policy, and existing source-view validators own selector admission.
function ownFields(value, role) {
  const declaration = readGrammarCompositionFieldOrder(role);
  if (!declaration) refuse('unknown field role');
  if (!value || typeof value !== 'object' || Array.isArray(value)
      || ![Object.prototype, null].includes(Object.getPrototypeOf(value))) refuse(role + ' needs ordinary data');
  const fields = declaration.split('|'), keys = Reflect.ownKeys(value);
  if (keys.length !== fields.length || keys.some(key => !fields.includes(key))) refuse(role + ' needs exactly its declared fields');
  const result = Object.create(null);
  for (const field of fields) {
    const descriptor = Object.getOwnPropertyDescriptor(value, field);
    if (!descriptor || !Object.hasOwn(descriptor, 'value') || !descriptor.enumerable) refuse(role + ' needs enumerable own data fields');
    result[field] = descriptor.value;
  }
  return result;
}

function ownItems(value) {
  if (!Array.isArray(value) || Object.getPrototypeOf(value) !== Array.prototype) refuse('candidates need an ordinary dense array');
  const length = Object.getOwnPropertyDescriptor(value, 'length').value;
  const keys = Reflect.ownKeys(value);
  if (keys.length !== length + 1 || keys[keys.length - 1] !== 'length') refuse('candidates need exactly their indexed fields');
  const result = [];
  for (let index = 0; index < length; index += 1) {
    const key = String(index), descriptor = Object.getOwnPropertyDescriptor(value, key);
    if (keys[index] !== key || !descriptor || !Object.hasOwn(descriptor, 'value') || !descriptor.enumerable) refuse('candidate indices need own data fields');
    result.push(descriptor.value);
  }
  return result;
}

function candidateFacts(selection, required = true) {
  const items = ownItems(selection.items);
  const kinds = [], mappings = [], names = [], selectors = [], keys = [];
  for (const value of items) {
    // The descriptor preflight precedes all property reads.
    if (!value || typeof value !== 'object') refuse('candidate needs an inspectable expression');
    if (!required && !acceptGrammarCompositionCandidateProfile(value.kind, value.mapping)) return null;
    const role = value.kind === 'grammar-field-candidate' ? 'field-candidate' : 'json-candidate';
    const candidate = ownFields(value, role);
    if (typeof candidate.name !== 'string' || typeof candidate.mapping !== 'string') refuse('candidate source and mapping need text');
    if (role === 'field-candidate') {
      if (!checkGrammarSelectionSourceViewForm(value, refuse)) refuse('field candidate is not a source-view form');
      selectors.push(renderLinksExpression(candidate.selector));
    } else selectors.push('');
    kinds.push(candidate.kind);
    mappings.push(candidate.mapping);
    names.push(candidate.name);
    keys.push(renderLinksExpression(value));
  }
  const issue = readGrammarCompositionCandidateIssue(kinds, mappings, names, selectors);
  if (issue) refuse(issue);
  checkGrammarSelection(selection, refuse);
  return JSON.stringify(keys);
}

function readTemplate(source, ruleKind, treeDecorators) {
  if (typeof source !== 'string') refuse('the authoring field needs Links text');
  let template;
  try {
    const statements = parseLinoStatements(source);
    if (statements.length !== 1) refuse('the authoring field needs exactly one expression');
    template = parseLinksExpression(statements[0]);
  } catch (error) {
    if (error instanceof DecoratorError) throw error;
    refuse('invalid authoring Links: ' + error.message);
  }
  checkGrammarSelectionIncludedRangeData(template, refuse);
  const selection = ownFields(template, 'selection');
  const slot = ownFields(selection.item, 'slot');
  const issue = readGrammarCompositionContextIssue(ruleKind, selection.kind, slot.kind, treeDecorators);
  if (issue) refuse(issue);
  const candidates = candidateFacts(selection);
  return { template, candidates };
}

/** Check actual rule data before an opted-in hook reads its record fields. */
export function checkGrammarCompositionRuleData(rule) {
  checkGrammarSelectionIncludedRangeData(rule, refuse);
}

/** Materialize the one authoring slot with the actual original expression.
 * A validated identical outer selection retains its complete existing data.
 */
export function composeGrammarRuleExpression(rule, source, ruleKind, treeDecorators) {
  checkGrammarCompositionRuleData(rule);
  const { template, candidates } = readTemplate(source, ruleKind, treeDecorators);
  const expression = rule.expression;
  if (!expression || typeof expression !== 'object') refuse('the original rule needs its actual expression');
  let actualMode = '', actualCandidates = '';
  if (expression.kind === 'any-grammar' || expression.kind === 'all-grammar') {
    const existing = ownFields(expression, 'selection');
    // Unsupported old profiles remain the actual inner expression. An outer
    // profile eligible for retention is checked completely before comparison.
    const key = candidateFacts(existing, false);
    if (key !== null) { actualMode = existing.kind; actualCandidates = key; }
  }
  if (readGrammarCompositionDisposition(actualMode, actualCandidates, template.kind, candidates) === 1) return expression;
  return { kind: template.kind, item: expression, items: template.items };
}
