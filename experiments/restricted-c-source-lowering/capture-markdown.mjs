#!/usr/bin/env node
// Source-only CI experiment: no C/Rust compilation or scanner execution.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {mkdirSync,readFileSync,writeFileSync} from 'node:fs';
import {dirname,resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {captureMarkdownNativeSources,findMarkdownNativeSourceArchive} from '../../js/scripts/capture-markdown-native-sources.mjs';
import {readMarkdownNativeSourceProfile} from '../../js/scripts/markdown-native-source-profile.mjs';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {lowerRestrictedCst,archiveRestrictedCLowering,readRestrictedCLowering} from './restricted-c-cst.mjs';

const digest=value=>createHash('sha256').update(value).digest('hex');

/** Lower actual pinned source without claiming checked executable semantics. */
export async function captureMarkdownScannerLowering({archive,receipt,output}) {
  if(process.env.CI!=='true'||process.env.GITHUB_ACTIONS!=='true')throw new Error('complete upstream scanner CST lowering is CI-only; run authored controls locally');
  const profile=readMarkdownNativeSourceProfile();
  const capture=readProjectData(receipt);
  assert.deepEqual(capture,captureMarkdownNativeSources(archive,profile),'rederive the complete original source receipt from its authenticated archive');
  const archived=readFileSync(archive);
  assert.equal(digest(archived),profile.archive.sha256);
  const block=capture.components.find(component=>component.id==='markdown');
  assert.ok(block);
  const bytes=execFileSync('tar',['-xOzf','-',profile.archive.memberPrefix+block.scanner.path],{input:archived,maxBuffer:16*1024*1024,timeout:30_000});
  assert.equal(bytes.length,block.scanner.bytes);
  assert.equal(digest(bytes),block.scanner.sha256);
  assert.equal(bytes.length,59243);
  assert.equal(digest(bytes),'02834bcbaf0cf51178e74450d487cc0231b9a52541cd374527ee35cfaf17fd4a');
  const source=new TextDecoder('utf-8',{fatal:true,ignoreBOM:true}).decode(bytes);
  // The current C CST frontend is an explicit source-only experiment input.
  // Its parse is not independent proof of native grammar or scanner support.
  const {parseProgrammingLanguage}=await import('../../js/src/programming-language-parser.js');
  const parsed=parseProgrammingLanguage(source,'C');
  const unit=lowerRestrictedCst(parsed,source,{
    abi:{intBits:32,sizeBits:64,charSigned:true},
    roots:['parse_fenced_code_block'],
    sourceDenominators:{archive:capture.archive,components:capture.components,
      receiptSha256:digest(readFileSync(receipt)),
      meaning:'authenticated complete source inventory; selected lowering is not complete scanner behavior'},
  });
  assert.deepEqual(unit.functions.map(item=>item.name).sort(),['advance','mark_end','parse_fenced_code_block','push_block'].sort());
  assert.ok(unit.enums.some(item=>item.entries.length===block.externalTokens.length));
  assert.equal(unit.checkedPortableProgram,null);
  assert.ok(unit.obligations.some(item=>item.category==='C execution boundary'));
  assert.ok(unit.obligations.some(item=>item.category==='C syntax derivation'));
  const packet=archiveRestrictedCLowering(unit);
  readRestrictedCLowering(packet.network,packet.root);
  mkdirSync(dirname(resolve(output)),{recursive:true});
  writeFileSync(resolve(output),packet.network.toLino());
  console.log('Source lowering only:',unit.functions.length,'original functions;',unit.obligations.length,'explicit obligations; no scanner execution performed');
  return unit;
}

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2),archiveAt=args.indexOf('--archive'),receiptAt=args.indexOf('--receipt'),outAt=args.indexOf('--out');
  if((archiveAt>=0)===args.includes('--cargo-cache')||archiveAt>=0&&!args[archiveAt+1]||receiptAt<0||!args[receiptAt+1]?.endsWith('.lino')||outAt<0||!args[outAt+1]?.endsWith('.lino'))throw new Error('usage: capture-markdown.mjs (--archive PINNED.crate | --cargo-cache) --receipt CAPTURE.lino --out LOWERED.lino');
  if(process.env.CI!=='true'||process.env.GITHUB_ACTIONS!=='true')throw new Error('complete upstream scanner CST lowering is CI-only; run authored controls locally');
  await captureMarkdownScannerLowering({archive:archiveAt>=0?args[archiveAt+1]:findMarkdownNativeSourceArchive(),receipt:args[receiptAt+1],output:args[outAt+1]});
}
