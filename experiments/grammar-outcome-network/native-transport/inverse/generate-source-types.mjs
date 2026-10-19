// Translate the complete shared source-type algorithm; native execution is CI-only.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {parseJavaScript} from '../../../../js/src/translation/javascript.js';
import {checkProgram} from '../../../../js/src/translation/check.js';
import {emitRust} from '../../../../js/src/translation/emit-rust.js';
import {writeRecordRuleLibrary} from '../../../../js/scripts/record-rule-library.mjs';
import {writeProjectData,checkProjectData} from '../../../../js/scripts/project-data.mjs';
import {encodeStructuredDataLinks} from '../../../../js/src/structured-data-links.js';
import {captureOrdinaryInversePlan,extendRecursiveInversePlan} from './plan.mjs';
import {reconstructOrdinaryOutcome} from './owned.mjs';
import {deriveOwnedOutcomeTypeSchema,readOwnedOutcomeTypeColumns} from './source-types.mjs';
import {generateOwnedOutcomeInverseFixtures} from './generate-fixtures.mjs';

export function generateOwnedOutcomeSourceTypes({check=false}={}) {
  // Finish the actual fresh parser oracle before retaining the source graph.
  const oracle=generateOwnedOutcomeInverseFixtures({check:true});
  const source=readFileSync(new URL('./source-type-rules.mjs',import.meta.url),'utf8');
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  const declarations=[...program.declarations.keys()];assert.equal(declarations.length,16);
  assert.equal(declarations[0],'checkOwnedOutcomeSourceTypes');
  const emitted=emitRust(program,{arrayConsumption:true});assert.equal(emitted.definitions.length,16);
  const files=writeRecordRuleLibrary(emitted,'owned_outcome_source_type_rules',{check});
  const ordinary=captureOrdinaryInversePlan(),recursive=extendRecursiveInversePlan(ordinary);
  const schema={modes:[0,0,0,1,2,3,4,5,-1],patterns:[0,1,2,3,4,5,6,7,8],
    patternOffsets:[0,0,0,2,3,4,6,6,6,6,7,8],patternTypes:[0,1,0,1,1,0,0,2],
    variantOwners:[-1,-1,-1,-1,-1,-1,-1,-1,-1,6,6]};
  const cases=[
    {name:'scalar',types:[0],selectors:[-1],offsets:[0,0],children:[],expected:0},
    {name:'fixed fields',types:[2,0,1],selectors:[-1,-1,-1],offsets:[0,2,2,2],children:[2,3],expected:0},
    {name:'absent option',types:[3],selectors:[0],offsets:[0,0],children:[],expected:0},
    {name:'present option',types:[3,0],selectors:[1,-1],offsets:[0,1,1],children:[2],expected:0},
    {name:'repeated values',types:[4,1,1],selectors:[-1,-1,-1],offsets:[0,2,2,2],children:[2,3],expected:0},
    {name:'ordered mapping',types:[5,1,0],selectors:[-1,-1,-1],offsets:[0,2,2,2],children:[2,3],expected:0},
    {name:'first variant',types:[6,0],selectors:[9,-1],offsets:[0,1,1],children:[2],expected:0},
    {name:'recursive variant',types:[6,2,0,1],selectors:[10,-1,-1,-1],offsets:[0,1,3,3,3],children:[2,3,4],expected:0},
    ...[0,1,2,3].map(selector=>({name:'JSON scalar '+selector,types:[7],selectors:[selector],offsets:[0,0],children:[],expected:0})),
    ...[4,5].map(selector=>({name:'JSON container '+selector,types:[7,7,7],selectors:[selector,1,2],offsets:[0,2,2,2],children:[2,3],expected:0})),
    {name:'fixed field type differs',types:[2,1,0],selectors:[-1,-1,-1],offsets:[0,2,2,2],children:[2,3],expected:-6},
    {name:'missing present option',types:[3],selectors:[1],offsets:[0,0],children:[],expected:-5},
    {name:'unknown option',types:[3],selectors:[-1],offsets:[0,0],children:[],expected:-4},
    {name:'option child type differs',types:[3,1],selectors:[1,-1],offsets:[0,1,1],children:[2],expected:-6},
    {name:'repeated child type differs',types:[4,0],selectors:[-1,-1],offsets:[0,1,1],children:[2],expected:-6},
    {name:'odd mapping arity',types:[5,1],selectors:[-1,-1],offsets:[0,1,1],children:[2],expected:-5},
    {name:'mapping pair types reversed',types:[5,0,1],selectors:[-1,-1,-1],offsets:[0,2,2,2],children:[2,3],expected:-6},
    {name:'variant belongs to another owner',types:[6,0],selectors:[2,-1],offsets:[0,1,1],children:[2],expected:-4},
    {name:'variant child type differs',types:[6,0],selectors:[10,-1],offsets:[0,1,1],children:[2],expected:-6},
    {name:'variant arity differs',types:[6,0,0],selectors:[9,-1,-1],offsets:[0,2,2,2],children:[2,3],expected:-5},
    {name:'JSON child type differs',types:[7,0],selectors:[4,-1],offsets:[0,1,1],children:[2],expected:-6},
    {name:'JSON scalar has children',types:[7,7],selectors:[3,0],offsets:[0,1,1],children:[2],expected:-5},
    {name:'unknown JSON selector',types:[7],selectors:[6],offsets:[0,0],children:[],expected:-4},
    {name:'unadmitted source owner',types:[8],selectors:[-1],offsets:[0,0],children:[],expected:-3},
    {name:'unknown source owner',types:[9],selectors:[-1],offsets:[0,0],children:[],expected:-3},
    {name:'missing selector',types:[0],selectors:[],offsets:[0,0],children:[],expected:-1},
    {name:'missing occurrence',types:[],selectors:[],offsets:[0],children:[],expected:-1},
    {name:'offset origin differs',types:[0],selectors:[-1],offsets:[1,1],children:[],expected:-2},
    {name:'child outside inventory',types:[4,1],selectors:[-1,-1],offsets:[0,1,1],children:[3],expected:-6},
  ];
  const sourceSchema=deriveOwnedOutcomeTypeSchema(recursive);
  const projections=oracle.cases.map(row=>{
    const packet=encodeStructuredDataLinks(row.expected);
    const model=reconstructOrdinaryOutcome(recursive,packet.network,packet.root,{usizeBits:64});
    const {schema:actual,...columns}=readOwnedOutcomeTypeColumns(recursive,model,{usizeBits:64});
    assert.deepEqual(actual,sourceSchema);return{name:row.id,...columns,expected:0};
  });
  assert.equal(projections.length,14);
  const fixture={schemaVersion:1,sourceSha256:createHash('sha256').update(source).digest('hex'),declarations,
    schema,cases,ordinarySourceSchema:deriveOwnedOutcomeTypeSchema(ordinary),sourceSchema,projections,
    boundary:'Complete source-type edge admission only, including every original projection. Native DTO construction, root/scalar checks, effective native JSON variants, static references, physical ownership and complete public default CST integration remain independent requirements.'};
  const output=new URL('../../../../parity/fixtures/grammar-outcome-source-type-rules.lino',import.meta.url);
  if(check)checkProjectData(output,fixture);else writeProjectData(output,fixture);
  return {files,fixture};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.length===3&&['--write','--check'].includes(process.argv[2]));
  const result=generateOwnedOutcomeSourceTypes({check:process.argv[2]==='--check'});
  console.log('Translated all sixteen source-type declarations and retained '+result.fixture.cases.length+' independent shapes plus all fourteen original projections.');
}
