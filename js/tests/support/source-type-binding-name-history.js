import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../../scripts/project-data.mjs';
import {tokenize} from '../../src/translation/lexer.js';
import {formatProductRuleSource} from '../../scripts/product-rule-style-corrections.mjs';

export function restoreSourceTypeBindingHistory(source) {
  const file=new URL('../../../parity/naming/source-type-binding-name-source-history.lino',import.meta.url);
  assert.equal(createHash('sha256').update(readFileSync(file)).digest('hex'),'92a9c26815601e9746786465d3caad50f7c5e152391aeb39614365d8fa3653e7');
  const history=readProjectData(file),rule=history.transformation;
  if(!source.includes(rule.nativeNewName))return source;
  const tokens=tokenize(source,'Rust').tokens.filter(token=>token.kind==='identifier'&&token.value===rule.nativeNewName);
  assert.ok(tokens.length>1);
  let inverse=source;
  for(const token of tokens.toReversed())inverse=inverse.slice(0,token.start)+rule.nativeOldName+inverse.slice(token.end);
  inverse=formatProductRuleSource(inverse);
  assert.equal(inverse,history.sources.find(row=>row.file.endsWith('/part_1.rs')).sourceText,'the complete old native module and every unchanged source context reconstruct exactly');
  return inverse;
}
