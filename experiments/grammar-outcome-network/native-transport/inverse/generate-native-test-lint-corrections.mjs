// Preserve complete independent native controls while correcting observed lint idioms.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../../../js/scripts/git-object-certificate.mjs';
import {checkProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';

const root=new URL('../../../../',import.meta.url),revision='d1232503fb38246f7e28f5d414c882a70944b38b';
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});

/** Apply only the recorded independent-control spelling and item-placement repairs. */
export function generateNativeTestLintCorrections({check=false}={}) {
  const sources=[],outputs=[];
  const changes=[
    ['rust/tests/unit/grammar_outcome_constructor_storage.rs',[
      {from:'assert!(!references.is_empty());',to:'assert_ne!(references, &Vec::<Value>::new());'},
      {from:'assert!(outcome.ambiguities.is_empty());',to:'assert_eq!(outcome.ambiguities, Vec::<meta_language::Ambiguity>::new());'},
    ]],
    ['rust/tests/unit/grammar_outcome_integer_decimal_rules.rs',[
      {from:'match text.parse::<i64>() {\n            Ok(value) => value.to_string() == text,\n            Err(_) => false,\n        }',to:'text.parse::<i64>().is_ok_and(|value| value.to_string() == text)'},
      {from:'match text.parse::<u64>() {\n            Ok(value) => value.to_string() == text && (width == 64 || value <= u64::from(u32::MAX)),\n            Err(_) => false,\n        }',to:'text.parse::<u64>().is_ok_and(|value| value.to_string() == text && (width == 64 || u32::try_from(value).is_ok()))'},
      {from:'if width == 32.0 {',to:'if width.to_bits() == 32.0_f64.to_bits() {'},
      {from:'assert_eq!(width, 64.0);',to:'assert_eq!(width.to_bits(), 64.0_f64.to_bits());'},
      {from:'assert!(\n                rules::read_owned_outcome_integer_decimal_limit(false, signed, width)\n                    .0\n                    .is_empty()\n            );',to:'assert_eq!(rules::read_owned_outcome_integer_decimal_limit(false, signed, width).0, Vec::<u16>::new());'},
    ]],
    ['rust/tests/unit/root_types.rs',[
      {from:'let roles = network\n        .links()\n        .filter(|link| link.metadata().term() == Some("declared type role"))\n        .collect::<Vec<_>>();',to:'let roles = network\n        .links()\n        .filter(|link| link.metadata().term() == Some("declared type role"))\n        .count();'},
      {from:'assert_eq!(roles.len(), usize::from(typed));',to:'assert_eq!(roles, usize::from(typed));'},
    ]],
    ['rust/tests/unit/grammar_outcome_static_borrows.rs',[
      {from:'    static LARGE: std::sync::OnceLock<String> = std::sync::OnceLock::new();\n',to:''},
      {from:'fn native_storage_returns_the_actual_existing_static_reference_without_allocating_a_new_lifetime() {',to:'fn native_storage_returns_the_actual_existing_static_reference_without_allocating_a_new_lifetime() {\n    static LARGE: std::sync::OnceLock<String> = std::sync::OnceLock::new();'},
    ]],
  ];
  for(const[file,actions]of changes){
    const original=captureGitArtifactCertificate(revision,file);verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:file});
    const sourceText=original.blob.toString('utf8');let raw=sourceText;
    for(const action of actions){assert.equal(raw.split(action.from).length,2,file+': exact original lint context');raw=raw.replace(action.from,action.to);}
    const current=execFileSync('rustfmt',['--edition','2024','--config','skip_children=true','--emit','stdout'],{input:raw,encoding:'utf8',timeout:15000,maxBuffer:2*1024*1024});
    let start=0,end=sourceText.length,finish=current.length;while(start<end&&start<finish&&sourceText[start]===current[start])start++;
    while(end>start&&finish>start&&sourceText[end-1]===current[finish-1]){end--;finish--;}
    const edit={offsetUnit:'UTF-16 code units',start,end,removed:sourceText.slice(start,end),inserted:current.slice(start,finish)};
    assert.equal(current.slice(0,start)+edit.removed+current.slice(start+edit.inserted.length),sourceText,'every original source input and assertion survives the complete byte inverse');
    outputs.push({file,current});
    sources.push({file,commit:revision,...pin(original.blob),sourceText,certificate:original.certificate,current:pin(current),actions,rawSource:raw,edit});
  }
  const history={schemaVersion:1,revision,sources,
    diagnostics:{run:39372455987,job:118471485751,logBytes:251690,logSha256:'5d224ce5bb87af3c47caefb8121f2aa3bd2f3cd7c69bc293136bb7caac1c1594',compiler:'1.99.0'},
    boundary:'Only standard parsing adapters, exact fixture widths, equivalent empty-container assertions, count-only collection removal and an unchanged scoped static item move. Every original fixture input and independent assertion meaning remains required; complete source history is not native execution or semantic concept deduplication evidence.'};
  const output=new URL('../../../../parity/self-translation/native-test-lint-corrections.lino',import.meta.url);
  for(const{file,current}of outputs){const actual=readFileSync(new URL(file,root),'utf8'),original=sources.find(row=>row.file===file).sourceText;
    if(check)assert.equal(actual,current);else assert.ok(actual===original||actual===current,'refuse to overwrite unrelated native work: '+file);}
  if(!check)for(const{file,current}of outputs)writeFileSync(new URL(file,root),current);
  if(check)checkProjectData(output,history);else writeProjectData(output,history);
  return history;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateNativeTestLintCorrections({check:process.argv.includes('--check')});
  console.log('Preserved every independent native control through the observed lint-source corrections.');
}
