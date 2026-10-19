// An explicit source interpretation, not a general Rust const evaluator.
// The older predicate model deliberately closes over its original expression
// forms. This profile has a different boundary: Boolean-only interfaces and
// inspectable conditional trees, excluding even the older numeric comparisons.
import {fileURLToPath} from 'node:url';
import {readFileSync, writeFileSync} from 'node:fs';
import {writeProjectData, checkProjectData} from '../js/scripts/project-data.mjs';
import {writeRecordRuleLibrary} from '../js/scripts/record-rule-library.mjs';

export const rustConstBranchPolicy = {
  version:1,
  model:'closed Boolean Rust const decision trees 1',
  language:'Rust',
  primitiveType:'bool',
  expressionKinds:['lit','var','unary','binary','if'],
  unaryOperations:['not'],
  binaryOperations:['and','or'],
  fields:{expressions:{
    lit:['k','value','type','span'],
    var:['k','name','type','span'],
    unary:['k','op','arg','type','span'],
    binary:['k','op','left','right','type','domain','semantics','span'],
    if:['k','cond','then','else','type','span'],
  }},
  interpretation:{
    inputDomain:'primitive Boolean values',
    observations:['returned primitive Boolean value','owned const function qualification'],
    excludedObservations:['compile-time invocation and evaluation','compile-time diagnostics','public API visibility','external linkage and ABI'],
  },
};

export function rustConstBranchRules(policy = rustConstBranchPolicy) {
  const either = (value, choices) => choices.map(choice => value+' === '+JSON.stringify(choice)).join(' || ');
  return '// Generated from the linked Boolean const decision-tree policy. Do not edit.\n'
    +'/** @param {string} model @returns {boolean} */\nexport function acceptRustConstBranchModel(model){return model === '+JSON.stringify(policy.model)+';}\n'
    +'/** @param {string} kind @returns {boolean} */\nexport function acceptRustConstBranchType(kind){return kind === '+JSON.stringify(policy.primitiveType)+';}\n'
    +'/** @param {string} kind @returns {boolean} */\nexport function acceptRustConstBranchKind(kind){return '+either('kind',policy.expressionKinds)+';}\n'
    +'/** @param {string} kind @param {string} operation @returns {boolean} */\nexport function acceptRustConstBranchOperation(kind,operation){return kind === "unary" && ('+either('operation',policy.unaryOperations)+') || kind === "binary" && ('+either('operation',policy.binaryOperations)+');}\n'
    +'/** @param {string} result @param {string} condition @param {string} consequent @param {string} alternative @returns {boolean} */\nexport function acceptRustConstBranchConditional(result,condition,consequent,alternative){return result === "bool" && condition === "bool" && consequent === "bool" && alternative === "bool";}\n';
}

export async function generateRustConstBranchRules({check = false, native = false} = {}) {
  const data = new URL('../js/src/translation/data/rust-const-branches.lino',import.meta.url);
  const source = new URL('../js/src/translation/rust-const-branch-rules.js',import.meta.url);
  const text = rustConstBranchRules();
  if (check) {
    if (!checkProjectData(data,rustConstBranchPolicy) || readFileSync(source,'utf8') !== text) throw new Error('Boolean const branch policy or complete source decisions are stale');
  } else {
    writeProjectData(data,rustConstBranchPolicy);
    writeFileSync(source,text);
  }
  if (native) {
    const [{parseJavaScript},{checkProgram},{emitRust}] = await Promise.all([
      import('../js/src/translation/javascript.js'),import('../js/src/translation/check.js'),import('../js/src/translation/emit-rust.js'),
    ]);
    // The entire five-declaration module is checked and translated together.
    const program = checkProgram(parseJavaScript(text));
    if (program.items.length !== 5) throw new Error('the complete Boolean const decision module must retain all five declarations');
    writeRecordRuleLibrary(emitRust(program),'rust_const_branch_rules',{check});
    const copy = new URL('../rust/src/data/rust-const-branches.lino',import.meta.url);
    if (check) {
      if (!readFileSync(copy).equals(readFileSync(data))) throw new Error('native Boolean const branch policy is stale');
    } else writeFileSync(copy,readFileSync(data));
  }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateRustConstBranchRules({check:process.argv.includes('--check'),native:process.argv.includes('--native') && !process.argv.includes('--javascript-only')});
}
