// Add one post-execution observation artifact without changing native commands.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../../../js/scripts/git-object-certificate.mjs';
import {parseNative} from '../../../../js/src/native-grammar-parser.js';
import {checkProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';

const root=new URL('../../../../',import.meta.url),file='js/scripts/rust-test-shards.mjs';
const revision='c38390b03edea87312d27a513de4aa660b8c27fb';
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});

/** Retain the complete original runner with exact additive host I/O changes. */
export function generateNativeNumberPublicationRunner({check=false}={}) {
  const original=captureGitArtifactCertificate(revision,file);
  verifyGitArtifactCertificate(original.certificate,original.blob,{revision:original.certificate.revision,path:file});
  const source=original.blob.toString('utf8');
  const additions=[
    {from:"import {appendFileSync, closeSync, mkdirSync, openSync, readFileSync, readdirSync, writeFileSync, writeSync} from 'node:fs';",
      to:"import {appendFileSync, closeSync, fstatSync, mkdirSync, openSync, readFileSync, readSync, readdirSync, writeFileSync, writeSync} from 'node:fs';"},
    {from:"import {readStructuredDataLino, structuredDataToLino} from '../src/structured-data-links.js';",
      to:"import {readStructuredDataLino, structuredDataToLino} from '../src/structured-data-links.js';\nimport {readNativeNumberControlPublication} from '../../experiments/grammar-outcome-network/native-transport/inverse/number-publication.mjs';"},
    {from:'async function runHarness(binary, arguments_) {',to:`function readNativePublicationText(filename) {
  const descriptor = openSync(filename, 'r');
  try {
    const info = fstatSync(descriptor);
    requireValue(info.isFile() && info.size <= 16 * 1024 * 1024, 'native publication text resource limit');
    const buffer = Buffer.alloc(info.size);
    let offset = 0;
    while (offset < buffer.length) {
      const count = readSync(descriptor, buffer, offset, buffer.length - offset, null);
      requireValue(count > 0, 'incomplete retained native publication text');offset += count;
    }
    requireValue(readSync(descriptor, Buffer.alloc(1), 0, 1, null) === 0, 'changed retained native publication text length');
    return new TextDecoder('utf-8', {fatal: true, ignoreBOM: true}).decode(buffer);
  } finally {closeSync(descriptor);}
}

export function retainNativeNumberPublication(directory, name, receipt) {
  const testName = 'grammar_outcome_original_numbers::original_borrowed_number_predicates_payloads_and_wire_positions_match_every_independent_control';
  if (receipt.identity.profile !== 'evidence' || !receipt.selected.some(row => row.name === testName)) return false;
  requireValue(typeof name === 'string' && /^[0-9a-f]{24}$/u.test(name), 'invalid native publication artifact identity');
  const prefix = path.join(directory, name);
  const receiptSource = readNativePublicationText(prefix + '.lino');
  const report = readNativeNumberControlPublication(readNativePublicationText(prefix + '.stderr.log'),
    readNativePublicationText(prefix + '.stdout.log'), receiptSource, encoded(receipt.identity));
  requireValue(JSON.stringify(readStructuredDataLino(receiptSource)) === JSON.stringify(receipt), 'the actual retained native execution receipt changed');
  writeFileSync(prefix + '.original-number-publication.lino.log', encoded(report), {flag: 'wx'});
  return true;
}

async function runHarness(binary, arguments_) {`},
    {from:'  verifyHarnessExecution(receipt);\n  return receipt.exitCode;\n}',
      to:'  verifyHarnessExecution(receipt);\n  retainNativeNumberPublication(current.directory, name, receipt);\n  return receipt.exitCode;\n}'},
  ];
  let current=source;
  for(const action of additions){assert.equal(current.split(action.from).length,2);current=current.replace(action.from,action.to);}
  let inverse=current;for(const action of [...additions].reverse()){assert.equal(inverse.split(action.to).length,2);inverse=inverse.replace(action.to,action.from);}
  assert.equal(inverse,source,'every original scheduling selection native command verification and sealing byte survives');
  const tree=parseNative('native-javascript',current);assert.equal(tree.hasError,false);assert.equal(tree.end,Buffer.byteLength(current));
  const receipt={schemaVersion:1,revision:original.certificate.revision,sources:[{file,commit:original.certificate.revision,...pin(original.blob),sourceText:source,certificate:original.certificate,current:pin(current),actions:additions}],
    boundary:'Add bounded retained-text reads and a post-verification artifact only for the selected current native number control in the evidence profile. The original command inventory output digest selection result and complete physical seal algorithms remain exact. Derived artifacts are reproducible from sealed raw inputs; they are not independent CI authentication or complete native reconstruction.'};
  const output=new URL('../../../../parity/self-translation/native-number-publication-runner.lino',import.meta.url);
  if(check){assert.equal(readFileSync(new URL(file,root),'utf8'),current);checkProjectData(output,receipt);}
  else{writeFileSync(new URL(file,root),current);writeProjectData(output,receipt);}
  return receipt;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateNativeNumberPublicationRunner({check:process.argv.includes('--check')});
  console.log('Generated additive native number artifact retention while conserving every complete original runner byte.');
}
