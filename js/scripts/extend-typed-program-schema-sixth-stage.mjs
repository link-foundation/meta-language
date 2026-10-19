// One immutable sixth-stage author. Source closure is separate from execution.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,readFileSync,writeFileSync,unlinkSync} from 'node:fs';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {checkProjectData,serializeProjectData} from './project-data.mjs';
import {readProgramTypeNeutralDefaults} from './read-program-type-neutral-defaults.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FILE,ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES} from '../../experiments/original-link-complete-table-projection/raw-schema-source-closure.mjs';
import {verifyOriginalLinkRawSchemaFourthExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-fourth-extension-closure.mjs';
import {verifyOriginalLinkRawSchemaThirdExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-third-extension-closure.mjs';
import {verifyOriginalLinkRawSchemaFifthExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-fifth-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSION_RECEIPT,ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES,
  ORIGINAL_LINK_RAW_SCHEMA_SIXTH_COMMAND,readOriginalLinkRawSchemaSixthContext,
  verifyOriginalLinkRawSchemaSixthExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-sixth-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_SIXTH_EXTENSIONS,ORIGINAL_LINK_RAW_SCHEMA_SIXTH_EXTENSION_SCOPE} from '../../experiments/original-link-complete-table-projection/raw-schema-sixth-extension-policy.mjs';
const root=new URL('../../',import.meta.url),schemaFile=new URL('../src/data/typed-program-links-schema.lino',import.meta.url);
const base=new URL('../../experiments/original-link-complete-table-projection/',import.meta.url);
const sixthFile=new URL('data/raw-schema-sixth-extension-closure.lino',base),oracleFile=new URL('raw-native-observations.mjs',base);
const sha256=bytes=>createHash('sha256').update(bytes).digest('hex');

export function extendTypedProgramSchemaSixthStage({check=false}={}){
  const input=readOriginalLinkRawSchemaSixthContext();
  const retainedFiles=['raw-schema-source-closure','raw-schema-extension-closure','raw-schema-second-extension-closure',
    'raw-schema-third-extension-closure','raw-schema-fourth-extension-closure','raw-schema-fifth-extension-closure',
    'raw-schema-second-extension-provenance','raw-schema-second-extension-provenance-0112b8e3'].map(name=>new URL('data/'+name+'.lino',base));
  const retained=retainedFiles.map(file=>readFileSync(file));
  const authoredSources=Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES.map(file=>[file,readFileSync(new URL(file,root))]));
  const actualBytes=readFileSync(schemaFile),oracle=readFileSync(oracleFile,'utf8');
  let sixth,sixthBytes,currentBytes=actualBytes;
  if(existsSync(sixthFile)){
    sixthBytes=readFileSync(sixthFile);sixth=readStructuredDataLino(sixthBytes.toString('utf8'));
    verifyOriginalLinkRawSchemaSixthExtensionClosure(input,actualBytes,sixth,authoredSources);checkProjectData(sixthFile,sixth);
  }else{
    assert.equal(check,false,'the sixth finite successor has not been authored');
    assert.equal(sha256(input.fifthBytes),ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSION_RECEIPT);
    verifyOriginalLinkRawSchemaFifthExtensionClosure(input,actualBytes,input.fifth,input.fifthSources);
    const after=readStructuredDataLino(actualBytes.toString('utf8'));
    assert.equal(Object.hasOwn(after,'neutralTypeFieldDefaults'),false);
    const sourceGuard=readProgramTypeNeutralDefaults();assert.deepEqual(sourceGuard.defaults,ORIGINAL_LINK_RAW_SCHEMA_SIXTH_EXTENSIONS.neutralTypeFieldDefaults);
    after.neutralTypeFieldDefaults=structuredClone(sourceGuard.defaults);currentBytes=Buffer.from(serializeProjectData(after,schemaFile));
    sixth={version:1,predecessorReceiptSha256:ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSION_RECEIPT,
      previous:{...input.fifth.current,source:actualBytes.toString('utf8')},
      current:{file:ORIGINAL_LINK_RAW_SCHEMA_FILE,bytes:currentBytes.length,sha256:sha256(currentBytes)},
      consumedSchemaProperties:[...ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES],appendedSchemaProperties:ORIGINAL_LINK_RAW_SCHEMA_SIXTH_EXTENSIONS,
      sourceGuard,authoring:{command:ORIGINAL_LINK_RAW_SCHEMA_SIXTH_COMMAND,sourceFiles:ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES
        .map(file=>({file,bytes:authoredSources[file].length,sha256:sha256(authoredSources[file])}))},scope:ORIGINAL_LINK_RAW_SCHEMA_SIXTH_EXTENSION_SCOPE};
    verifyOriginalLinkRawSchemaSixthExtensionClosure(input,currentBytes,sixth,authoredSources);
    sixthBytes=Buffer.from(serializeProjectData(sixth,sixthFile));
  }
  const pattern=/^const schemaSixthExtensionReceiptSha256 = '(?:PENDING|[0-9a-f]{64})';$/mu;
  assert.equal([...oracle.matchAll(new RegExp(pattern.source,'gmu'))].length,1,'one separate sixth receipt pin is required');
  const previousPin=oracle.match(pattern)[0],pin="const schemaSixthExtensionReceiptSha256 = '"+sha256(sixthBytes)+"';";
  if(check||existsSync(sixthFile))assert.equal(previousPin,pin,'the complete immutable sixth receipt pin stays exact');
  else{
    const updated=oracle.replace(pattern,pin);assert.equal(updated.replace(pin,previousPin),oracle);
    let created=false;
    try{writeFileSync(sixthFile,sixthBytes,{flag:'wx'});created=true;writeFileSync(schemaFile,currentBytes);writeFileSync(oracleFile,updated);}
    catch(error){if(created){writeFileSync(schemaFile,actualBytes);writeFileSync(oracleFile,oracle);unlinkSync(sixthFile);}throw error;}
  }
  retainedFiles.forEach((file,index)=>assert.deepEqual(readFileSync(file),retained[index]));
  return {schemaSha256:sixth.current.sha256,predecessorSchemaSha256:sixth.previous.sha256,
    predecessorReceiptSha256:sixth.predecessorReceiptSha256,appendedRootProperties:1,appendedTypeFields:5,
    preservedProducerBindings:Object.keys(input.historicalSources).length,sourceClosureOnly:true,check};
}

export function checkTypedProgramSchemaFifthArchive(){
  extendTypedProgramSchemaSixthStage({check:true});const input=readOriginalLinkRawSchemaSixthContext();
  const sixth=readStructuredDataLino(readFileSync(sixthFile,'utf8'));
  verifyOriginalLinkRawSchemaFifthExtensionClosure(input,Buffer.from(sixth.previous.source),input.fifth,input.fifthSources);
  return {schemaSha256:input.fifth.current.sha256,predecessorSchemaSha256:input.fifth.previous.sha256,
    predecessorReceiptSha256:input.fifth.predecessorReceiptSha256,appendedProjectionKeys:2,
    preservedProducerBindings:Object.keys(input.historicalSources).length,sourceClosureOnly:true,check:true};
}

/** Check the exact archived fourth profile after authenticating its live successor. */
export function checkTypedProgramSchemaFourthArchive() {
  extendTypedProgramSchemaSixthStage({check: true});
  const input = readOriginalLinkRawSchemaSixthContext();
  const fifth = input.fifth;
  verifyOriginalLinkRawSchemaFourthExtensionClosure(input, Buffer.from(fifth.previous.source, 'utf8'), input.fourth, input.fourthSources);
  return {schemaSha256: input.fourth.current.sha256, predecessorSchemaSha256: input.fourth.previous.sha256,
    predecessorReceiptSha256: input.fourth.predecessorReceiptSha256, appendedProjectionKeys: 1,
    preservedProducerBindings: Object.keys(input.historicalSources).length, sourceClosureOnly: true, check: true};
}

/** Check the exact archived third profile after authenticating all successors. */
export function checkTypedProgramSchemaThirdArchive() {
  extendTypedProgramSchemaSixthStage({check: true});
  const input = readOriginalLinkRawSchemaSixthContext();
  verifyOriginalLinkRawSchemaThirdExtensionClosure(input.receipt, input.first, Buffer.from(input.fourth.previous.source, 'utf8'),
    input.historicalSources, input.second, input.third, input.thirdSources);
  return {schemaSha256: input.third.current.sha256, predecessorSchemaSha256: input.third.previous.sha256,
    predecessorReceiptSha256: input.third.predecessorReceiptSha256,
    appendedProjectionKeys: Object.keys(input.third.appendedSchemaEntries.expressionProjections).length,
    appendedAlternativeKeys: Object.keys(input.third.appendedSchemaEntries.expressionProjectionAlternatives).length,
    preservedProducerBindings: Object.keys(input.historicalSources).length, sourceClosureOnly: true, check: true};
}


if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);assert.ok(args.length===0||args.length===1&&args[0]==='--check');
  console.log(JSON.stringify(extendTypedProgramSchemaSixthStage({check:args[0]==='--check'}),null,2));
}
