// Existing source terms form a topologically ordered construction plan.
// Hosts transport actual operands; this definition owns evaluation and writes.
import { readJavaScriptNumberBitwiseAssignment as readScalarAssignmentBitwiseOperation } from './number-bitwise-rules.js';

/** @param {string} operator @returns {string} */
export function readScalarAssignmentOperation(operator) {
  if (operator === '=') return 'replace';
  if (operator === '+=') return 'plus';
  if (operator === '-=') return 'sub';
  if (operator === '*=') return 'mul';
  if (operator === '/=') return 'div';
  if (operator === '%=') return 'rem';
  if (operator === '**=') return 'exponentiateJavaScriptNumericValue';
  if (operator === '&&=') return 'and';
  if (operator === '||=') return 'or';
  if (operator === '??=') return 'coalesce';
  if (operator === '++') return 'add';
  if (operator === '--') return 'sub';
  return readScalarAssignmentBitwiseOperation(operator);
}

/** Comma evaluates/GetValues the left operand before returning the right value.
 * Uses the same source-term plan schema: two actual operands and one let.
 * @returns {number[]} */
export function planJavaScriptSequenceExpression() {
  return [0,0,0,0, 0,1,0,0, 7,0,0,1];
}
/** Each node is [operation,first,second,third]; references name earlier nodes.
 * 0 operand(cell/right),1 temporary-name(previous/updated),2 cell read,
 * 3 numeric unit,4 binary,5 math,6 coalesce,7 let,8 cell write,9 conditional,10 unary negation.
 * The final node is the root; operand one never appears for an update.
 * @param {string} operator @param {boolean} postfix @returns {number[]} */
export function planScalarAssignmentExpression(operator, postfix) {
  const operation = readScalarAssignmentOperation(operator);
  if (operation === '' || postfix && operator !== '++' && operator !== '--') return [];
  // 0 cell,1 previous binder,2 updated binder,3 previous cell value.
  let plan = [0,0,0,0, 1,0,0,0, 1,1,0,0, 2,0,0,0];
  if (operation === 'replace') {
    // RHS is evaluated once before the write; replacement does not read a value.
    return plan.concat([0,1,0,0, 8,0,2,2, 7,1,4,5]);
  }
  if (operation === 'and' || operation === 'or' || operation === 'coalesce') {
    // 4 RHS,5 write/result,6 bind RHS,7 selected continuation,8 bind previous.
    plan = plan.concat([0,1,0,0, 8,0,2,2, 7,1,4,5]);
    if (operation === 'and') plan = plan.concat([9,1,6,1]);
    else if (operation === 'or') plan = plan.concat([9,1,1,6]);
    else plan = plan.concat([6,1,6,0]);
    return plan.concat([7,0,3,7]);
  }
  const update = operator === '++' || operator === '--';
  let previous = 3;
  if (update) {
    // -(-value) preserves Number/BigInt and performs the same ToNumeric
    // before both the update and the postfix result, including nullish values.
    plan = plan.concat([10,3,0,0, 10,4,0,0]);
    previous = 5;
  }
  const right = plan.length / 4;
  plan = plan.concat([update ? 3 : 0,1,0,0]);
  const computed = right + 1, written = right + 2, bound = right + 3;
  const arithmetic = operation === 'plus' || operation === 'add' || operation === 'sub'
    || operation === 'mul' || operation === 'div' || operation === 'rem';
  return plan.concat([arithmetic ? 4 : 5,0,1,right, 8,0,2,postfix ? 1 : 2,
    7,1,computed,written, 7,0,previous,bound]);
}
