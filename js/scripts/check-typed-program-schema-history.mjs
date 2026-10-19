// Archive checks authenticate the live successor before using original strict guards.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {generateStringOrderRules} from './string-order-generation.mjs';
import {readProjectData} from './project-data.mjs';
import {readOriginalLinkRawSchemaSixthContext,ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES,
  verifyOriginalLinkRawSchemaSixthExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-sixth-extension-closure.mjs';
import {verifyOriginalLinkRawSchemaFifthExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-fifth-extension-closure.mjs';
import {verifyOriginalLinkRawSchemaFourthExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-fourth-extension-closure.mjs';
import {verifyOriginalLinkRawSchemaThirdExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-third-extension-closure.mjs';
const root=new URL('../../',import.meta.url);

export function readStringOrderSchemaPredecessor(){
  // The existing generator validates the full actual model, both complete
  // schema inverses, every native declaration, the prelude and all fixtures.
  generateStringOrderRules({check:true});
  const history=readProjectData(new URL('parity/naming/unicode-string-order-source-history.lino',root));
  const source=history.sources.find(row=>row.file==='js/src/data/typed-program-links-schema.lino');
  assert.ok(source);assert.equal(source.commit,'2b9fd0df45757bf70a69deea6ab97d969cf6a25d');
  return Buffer.from(source.sourceText);
}

export function verifyStringOrderSchemaSuccessor(input,currentBytes,sixth,authoredSources){
  const original=readStringOrderSchemaPredecessor();
  assert.ok(Buffer.isBuffer(currentBytes));
  assert.ok(currentBytes.equals(readFileSync(new URL('js/src/data/typed-program-links-schema.lino',root))),
    'the supplied current schema is the actual complete checked string order successor');
  const previous=verifyOriginalLinkRawSchemaSixthExtensionClosure(input,original,sixth,authoredSources);
  const hash=value=>createHash('sha256').update(value).digest('hex');
  return {...previous,sha256:hash(currentBytes),stringOrderPredecessorSha256:hash(original),
    stringOrderSourceRevision:'2b9fd0df45757bf70a69deea6ab97d969cf6a25d'};
}

function readAuthenticatedSixthArchive(){
  const original=readStringOrderSchemaPredecessor();
  const input=readOriginalLinkRawSchemaSixthContext();
  const sixth=readStructuredDataLino(readFileSync(new URL('experiments/original-link-complete-table-projection/data/raw-schema-sixth-extension-closure.lino',root),'utf8'));
  const authoredSources=Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES.map(path=>[path,readFileSync(new URL(path,root))]));
  verifyOriginalLinkRawSchemaSixthExtensionClosure(input,original,sixth,authoredSources);
  return {input,sixth};
}

export function checkTypedProgramSchemaFifthArchive(){
  const {input,sixth}=readAuthenticatedSixthArchive();
  verifyOriginalLinkRawSchemaFifthExtensionClosure(input,Buffer.from(sixth.previous.source),input.fifth,input.fifthSources);
  return {schemaSha256:input.fifth.current.sha256,predecessorSchemaSha256:input.fifth.previous.sha256,
    predecessorReceiptSha256:input.fifth.predecessorReceiptSha256,appendedProjectionKeys:2,
    preservedProducerBindings:Object.keys(input.historicalSources).length,sourceClosureOnly:true,check:true};
}

export function checkTypedProgramSchemaFourthArchive() {
  const {input}=readAuthenticatedSixthArchive();
  const fifth = input.fifth;
  verifyOriginalLinkRawSchemaFourthExtensionClosure(input, Buffer.from(fifth.previous.source, 'utf8'), input.fourth, input.fourthSources);
  return {schemaSha256: input.fourth.current.sha256, predecessorSchemaSha256: input.fourth.previous.sha256,
    predecessorReceiptSha256: input.fourth.predecessorReceiptSha256, appendedProjectionKeys: 1,
    preservedProducerBindings: Object.keys(input.historicalSources).length, sourceClosureOnly: true, check: true};
}

export function checkTypedProgramSchemaThirdArchive() {
  const {input}=readAuthenticatedSixthArchive();
  verifyOriginalLinkRawSchemaThirdExtensionClosure(input.receipt, input.first, Buffer.from(input.fourth.previous.source, 'utf8'),
    input.historicalSources, input.second, input.third, input.thirdSources);
  return {schemaSha256: input.third.current.sha256, predecessorSchemaSha256: input.third.previous.sha256,
    predecessorReceiptSha256: input.third.predecessorReceiptSha256,
    appendedProjectionKeys: Object.keys(input.third.appendedSchemaEntries.expressionProjections).length,
    appendedAlternativeKeys: Object.keys(input.third.appendedSchemaEntries.expressionProjectionAlternatives).length,
    preservedProducerBindings: Object.keys(input.historicalSources).length, sourceClosureOnly: true, check: true};
}
