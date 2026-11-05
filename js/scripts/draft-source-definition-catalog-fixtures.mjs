// Future control declarations only; no represented graph or function executes.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const valid={identifiers:['meta-language/source-program/array-classification-program'],assets:['array-classification-program.lino'],digests:['0'.repeat(64)],bytes:[1],roots:['1']};
const cases=[{name:'qualified one-entry metadata',...valid,result:1}];
for(const [name,field,value,result]of [
 ['parent path','assets',['../outside-program.lino'],-2],
 ['absolute path','assets',['/outside-program.lino'],-2],
 ['wrong suffix','assets',['array-classification.json'],-2],
 ['unqualified identity','identifiers',['array-classification-program'],-3],
 ['upper-case digest','digests',['A'.repeat(64)],-4],
 ['empty source','bytes',[0],-5],
 ['oversized source','bytes',[134217729],-5],
 ['zero root','roots',['0'],-6],
 ['noncanonical decimal root','roots',['01'],-6],
 ['beyond safe-number root','roots',['9007199254740992'],-6],
 ['unequal columns','roots',[],-1],
 ])cases.push({name,...valid,[field]:value,result});
cases.push({name:'duplicate qualified identity',identifiers:[...valid.identifiers,...valid.identifiers],assets:[...valid.assets,...valid.assets],digests:[...valid.digests,...valid.digests],bytes:[1,1],roots:['1','1'],result:-7});
const fixture={
 schemaVersion:1,model:'complete qualified typed source programs 1',cases,
 selectedIdentifier:valid.identifiers[0],selectedFunction:'readJavaScriptArrayClassification',
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 assertions:['everyActualProgramCataloguedWithExactPackageTwin','qualifiedArtifactIdentityNotMeaningProof','sharedMetadataValidation','freshMetadataAndGraphOwnership','unknownIdentityRefusal','selectionResourceBounds','authenticatedRootAndEntireArtifact','existingFormalModelAndOwnedFunctionSelection','completeProgramAndFormalArchiveTextBinaryReload','pairedDefinitionCommandVocabulary'],
 boundary:'Unexecuted future controls. Catalog rows identify complete source artifacts, not a new formal primitive model. Every original unsupported formal context remains refused. No semantic correspondence or requirement completion is established by listing a source program.',
};
for(const path of ['parity/fixtures/source-definition-catalog.lino','rust/src/data/source-definition-catalog-controls.lino'])writeProjectData(new URL('../../'+path,import.meta.url),fixture);
