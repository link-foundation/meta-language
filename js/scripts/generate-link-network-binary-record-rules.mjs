// Extend the authenticated generator through checked Links decorators. The
// frozen producer and every historical compiler/style assertion remain intact.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {DecoratorSet} from '../src/decorators.js';
import {readProjectData,writeProjectData} from './project-data.mjs';

assert.ok(process.argv.slice(2).every(argument=>argument==='--check'),
  'record generation accepts only --check and cannot rewrite historical compiler receipts');
const producerFile=new URL('./generate-link-network-binary-rules.mjs',import.meta.url);
const producer=readFileSync(producerFile,'utf8');
const digest=value=>createHash('sha256').update(value).digest('hex');
assert.equal(digest(producer),'b69fe844f08f933f7cca47b3fc34139e606b7bd72cefb3fd3c6380d4301745c8');
const primitiveFile=new URL('../src/link-network-binary-rules.js',import.meta.url);
const primitives=readFileSync(primitiveFile,'utf8');
assert.equal(digest(primitives),'4f0debafedcc6e1985ccad8ed0dd673261e511e705d4d7535db008d446cf11a4');
const recordFile=new URL('../src/link-network-binary-record-rules.js',import.meta.url);
const recordSource=readFileSync(recordFile,'utf8');
const imports=/^import \{ linkNetworkBinaryUnit, acceptLinkNetworkBinaryInteger, readLinkNetworkBinaryInteger,\n  linkNetworkBinaryIntegerByte, renderLinkNetworkBinaryField \} from '\.\/link-network-binary-rules\.js';\n/mu;
assert.equal(recordSource.match(imports)?.length,1,'only the authenticated primitive module supplies the imported operations');
const boundary=primitives.indexOf('/** Validate and transcode');
assert.ok(boundary>0);
const completeSource=primitives.slice(0,boundary)+recordSource.replace(imports,'');
const stylesFile=new URL('../../parity/self-translation/link-network-binary-record-rules-decorators.lino',import.meta.url);
const receiptFile=new URL('../../parity/self-translation/link-network-binary-record-generation.lino',import.meta.url);
const adapterFile=new URL('../../parity/self-translation/link-network-binary-record-generator-adapter.lino',import.meta.url);

const deriveStyles=`
const historical = DecoratorSet.fromLino(readFileSync(originalStyles, 'utf8'));
const priorSource = readFileSync(new URL('../src/link-network-binary-rules.js', import.meta.url), 'utf8');
assert.equal(createHash('sha256').update(priorSource).digest('hex'), '4f0debafedcc6e1985ccad8ed0dd673261e511e705d4d7535db008d446cf11a4');
assert.equal(createHash('sha256').update(readFileSync(originalStyles)).digest('hex'), 'bebfe31e9c01424bb894fa80935df173a7814515bd51d7d8a6eea9f454d371cb');
const priorEmission = emitRust(checkProgram(parseJavaScript(priorSource, {referenceSemantics: false})));
const previousItems = [...priorEmission.preludes, ...priorEmission.definitions];
const previousStyled = styleBinaryItems(previousItems, historical);
const adapter = readBinaryLibraryAdapter({raw, names, items}, true);
const base = styleBinaryItems(items, new DecoratorSet([adapter]));
const {readBinaryMetadataCandidateStyles} = await import('./binary-metadata-candidate-styles.mjs');
const derived = readBinaryMetadataCandidateStyles({items, base, previousItems, previousStyled, adapter, format});
const expectedStyles = new DecoratorSet(derived.decorators).toLino();
if (process.argv.includes('--check')) assert.equal(readFileSync(styles, 'utf8'), expectedStyles, 'record-local binary styles are fresh');
else writeFileSync(styles, expectedStyles);
const decorators = DecoratorSet.fromLino(expectedStyles);
const styled = styleBinaryItems(items, decorators);
globalThis[Symbol.for('meta-language binary record generation')] = {
  ...derived.receipt,
  checkedDeclarations: emitted.definitions.length,
  historicalReplacementContexts: historical.decorators.filter(row => row.when.some(([field, value]) => field === 'scope' && value === 'formatted-item'))
    .reduce((count, row) => count + row.actions.filter(action => action.op === 'replace').length, 0),
  outputSources: styled,
};`;

const replacements=[
  ["const candidateMode = process.argv.includes('--metadata-buffer-candidate');",'const candidateMode = false;'],
  ["const styles = candidateMode ? new URL('../../experiments/binary-metadata-buffer/native-styles.lino', import.meta.url) : originalStyles;","const styles = new URL('../../parity/self-translation/link-network-binary-record-rules-decorators.lino', import.meta.url);"],
  ["const destination = new URL(candidateMode ? '../../experiments/binary-metadata-buffer/native/' : '../../rust/src/link_network_binary_rules/', import.meta.url);",`const destination = new URL('../../rust/src/link_network_binary_record_rules/', import.meta.url);`],
  ["parseJavaScript(readFileSync(source, 'utf8'), candidatePolicy?.parser ?? { referenceSemantics: false })",`parseJavaScript(${JSON.stringify(completeSource)}, candidatePolicy?.parser ?? { referenceSemantics: false })`],
  ["const decorators = DecoratorSet.fromLino(readFileSync(styles, 'utf8'));\nconst styled = styleBinaryItems(items, decorators);",deriveStyles],
  ['// Generated from link-network-binary-rules.js by the checked translator.','// Generated from link-network-binary-record-rules.js and authenticated shared primitives by the checked translator.'],
];
const profile=new DecoratorSet([{id:'binary-record-generator-adapter',level:'emitter',order:0,when:[['format','JavaScript']],
  actions:replacements.map(([from,to])=>({op:'replace',field:'source',from,to}))}]);
for(const[from]of replacements)assert.equal(producer.split(from).length,2,'each producer adaptation owns one complete exact context');
const profileBytes=profile.toLino();
if(process.argv.includes('--check'))assert.equal(readFileSync(adapterFile,'utf8'),profileBytes,'the linked producer adaptation is fresh');
else writeFileSync(adapterFile,profileBytes);
let adapted=DecoratorSet.fromLino(profileBytes).decorate('emitter',{format:'JavaScript',source:producer}).source;
// Resolve module addresses without copying a checkout, dependencies or compiler
// outputs. All compilation below is JavaScript translation and rustfmt only.
adapted=adapted.replace(/import\.meta\.url/gu,JSON.stringify(producerFile.href))
  .replace(/(from |import\()(['"])(\.[^'"\n]+)\2/gu,(_,prefix,quote,path)=>prefix+quote+new URL(path,producerFile).href+quote);
try {await import('data:text/javascript;base64,'+Buffer.from(adapted).toString('base64'));}
catch(error){throw new Error(String(error.message),{cause:undefined});}
const key=Symbol.for('meta-language binary record generation');
const generated=globalThis[key];delete globalThis[key];
assert.ok(generated?.outputSources?.length);
const {outputSources,...observations}=generated;
const receipt={schemaVersion:1,producerSha256:digest(producer),primitiveSourceSha256:digest(primitives),
  recordGeneratorSha256:digest(readFileSync(new URL(import.meta.url))),
  recordSourceSha256:digest(recordSource),completeSourceSha256:digest(completeSource),
  adapterSha256:digest(profileBytes),stylesSha256:digest(readFileSync(stylesFile)),
  styleBuilderSha256:digest(readFileSync(new URL('./binary-metadata-candidate-styles.mjs',import.meta.url))),
  ...observations,declarations:observations.declarations.map((row,index)=>({...row,styledSha256:digest(outputSources[index])}))};
if(process.argv.includes('--check'))assert.deepEqual(readProjectData(receiptFile),receipt,'the complete native generation receipt is fresh');
else writeProjectData(receiptFile,receipt);
