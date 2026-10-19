// Experiment only. Writes proposals solely into explicitly selected task scratch.
// Mechanical data transport, not JavaScript semantic self-translation.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {resolve,dirname} from 'node:path';
import {fileURLToPath} from 'node:url';
import {readRustSourceItems} from '../../../../js/scripts/generate-private-output-test-model.mjs';
import {writeProjectData} from '../../../../js/scripts/project-data.mjs';
import {buildOutcomePacket} from '../wire/generate.mjs';
import {repository,outcomeWirePolicy} from '../wire/wire-policy.mjs';
import {explicitOutputDirectory} from '../wire/output.mjs';

const digest=source=>createHash('sha256').update(source).digest('hex');
const parentPath='rust/src/structured_data_links.rs';
const moduleAddition='mod grammar_outcome;\npub use grammar_outcome::encode_grammar_parse_outcome_links;\n\n';
const moduleAnchor='const LANGUAGE: &str = "meta-language structured data";';
const functionAnchor='pub(crate) fn from_json(value: &Value, depth: usize) -> Result<StructuredDataValue, String> {';
const armPrefix='        Value::Number(number) => {\n';
const armSuffix='\n        }';
const callArm='        Value::Number(number) => StructuredDataValue::Number(exact_json_number(number)?),';
const unique=(source,needle)=>{
  const index=source.indexOf(needle);assert.ok(index>=0&&source.indexOf(needle,index+1)===-1,'one exact qualified source occurrence');return index;
};

/** Extract the actual original Number arm from the actual CST-owned from_json.
 * Its body, comments, integer comparison and existing scoped allow move intact.
 */
export function proposeParent(source) {
  assert.equal(digest(source),outcomeWirePolicy.pins[parentPath],'original parent remains authenticated');
  const parsed=readRustSourceItems(Buffer.from(source));
  assert.ok(!parsed.items.some(item=>item.name==='exact_json_number'||item.name==='grammar_outcome'),'no hidden new owner collision');
  const functions=parsed.items.filter(item=>item.kind==='function_item'&&item.name==='from_json');
  assert.equal(functions.length,1);
  const originalFunction=parsed.text(functions[0].node);
  const start=unique(originalFunction,armPrefix);
  const end=originalFunction.indexOf('\n        Value::String(text) =>',start);
  assert.ok(end>start);
  const originalArm=originalFunction.slice(start,end);
  assert.ok(originalArm.endsWith(armSuffix));
  const indented=originalArm.slice(armPrefix.length,-armSuffix.length).split('\n');
  assert.ok(indented.every(line=>line===''||line.startsWith('            ')),'only source indentation is removed');
  const body=indented.map(line=>line.slice(12)).join('\n');
  const returnLine='StructuredDataValue::Number(value)';
  assert.ok(body.endsWith(returnLine));unique(body,returnLine);
  const borrowedBody=body.slice(0,-returnLine.length)+'Ok(value)';
  const helper='fn exact_json_number(number: &Number) -> Result<f64, String> {\n'
    +borrowedBody.split('\n').map(line=>'    '+line).join('\n')+'\n}\n\n';
  const fromJsonReplacement=originalFunction.replace(originalArm,callArm);
  unique(source,originalFunction);unique(source,moduleAnchor);unique(source,functionAnchor);
  const proposed=source.replace(originalFunction,fromJsonReplacement)
    .replace(functionAnchor,helper+functionAnchor).replace(moduleAnchor,moduleAddition+moduleAnchor);
  const inverse=candidate=>{
    unique(candidate,moduleAddition);unique(candidate,helper);unique(candidate,callArm);
    return candidate.replace(moduleAddition,'').replace(helper,'').replace(callArm,originalArm);
  };
  assert.equal(inverse(proposed),source,'whole original parent is exactly reconstructible');
  return {source:proposed,inverse,helper,originalArm,callArm,
    evidence:{path:parentPath,originalBytes:Buffer.byteLength(source),originalSha256:digest(source),
      proposedBytes:Buffer.byteLength(proposed),proposedSha256:digest(proposed),
      originalNumberArmSha256:digest(originalArm),helperSha256:digest(helper),
      sourceFunction:{start:functions[0].node.start,end:functions[0].node.end,sha256:digest(originalFunction)},
      permittedChanges:['private child module and public use','exact Number arm extraction and scalar call']}};
}

export function buildIntegration(root=repository) {
  // This reauthenticates all B94 source pins, aliases, imports, root paths and
  // every one of the nine declarations before emitting any private output.
  const packet=buildOutcomePacket(root);
  const original=readFileSync(resolve(root,parentPath),'utf8');
  const parent=proposeParent(original);
  const template=readFileSync(new URL('./sinks.rs',import.meta.url),'utf8');
  const oldNotice='// PRIVATE GENERATED PROPOSAL; no concrete sink or native execution.\n'
    +'// One ordered borrowed visitor drives CountSink(Node=()) and DirectSink(Node=LinkId).\n'
    +'// Recursive control flow below is NOT an arbitrary-depth resource proof.\n';
  assert.ok(packet.source.startsWith(oldNotice),'exact unchanged generated visitor notice');
  const notice='// Generated exact borrowed declaration transport. Native execution remains unverified.\n'
    +'// Recursive visitor admission is explicitly bounded to depth limits at most 256.\n';
  const child=template+'\n'+notice+packet.source.slice(oldNotice.length);
  assert.ok(!child.includes('.clone(')&&!child.includes('to_value')&&!child.includes('from_json('));
  assert.ok(!child.includes('StructuredDataValue')&&!child.includes(' as f64'));
  assert.ok(child.split('\n').length<1000,'bounded source owner');
  return {original,parent,child,plan:packet.plan,receipt:{schemaVersion:1,
    boundary:'Private mechanically generated transport; no native compilation, execution, public API parity or default CST integration claim.',
    parent:parent.evidence,child:{path:'rust/src/structured_data_links/grammar_outcome.rs',bytes:Buffer.byteLength(child),sha256:digest(child)},
    predecessor:{policySha256:digest(readFileSync(new URL('../wire/wire-policy.mjs',import.meta.url))),
      originalParentPin:outcomeWirePolicy.pins[parentPath],pins:packet.receipt.inputs,sourceViews:packet.receipt.sourceViews},
    sourcePolicyChanges:[], // Parent successor is recorded separately above; no old pin is overwritten.
    generatorSha256:digest(readFileSync(fileURLToPath(import.meta.url))),templateSha256:digest(template),
    profile:{maximumDepthLimit:256,limits:'positive; expanded occurrences/scalar bytes; new API only',rootCountOrder:'last'},
    pending:['native compilation and execution','matching JavaScript API or experiment-only placement','CI all-nine-type parity and budget controls','arbitrary-depth explicit native frames']}};
}

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)) {
  const directory=explicitOutputDirectory(process.argv.slice(2));
  const result=buildIntegration();
  writeFileSync(resolve(directory,'structured_data_links.proposed.rs'),result.parent.source);
  writeFileSync(resolve(directory,'grammar_outcome.rs'),result.child);
  writeProjectData(resolve(directory,'integration-receipt.lino'),result.receipt);
  console.log('Prepared private exact-inverse parent and exhaustive direct-sink child; native execution remains pending.');
}
