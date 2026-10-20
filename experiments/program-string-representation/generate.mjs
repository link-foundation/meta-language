// Mechanical host-field plumbing; all boolean decisions are shared JS rules.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';
import {readSourceTextRustBlock} from '../../js/scripts/source-text-byte-library-style.mjs';
import {formatProductRuleSource} from '../../js/scripts/product-rule-style-corrections.mjs';
import {readProjectData,checkProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';

const root=new URL('../../',import.meta.url),revision='90636f9fb52b1b9095defbac041c8089608b8576';
const output=new URL('parity/self-translation/program-string-representation-hosts.lino',root);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

/** Derive exact host changes and retain every complete original Git source. */
export function generateProgramStringRepresentationHosts({check=false,apply=false}={}) {
  assert.equal(check&&apply,false);
  const primitiveFile='rust/src/translation/surface/array_item.rs',primitive=captureGitArtifactCertificate(revision,primitiveFile);
  const booleanReader=readSourceTextRustBlock(primitive.blob.toString(),'fn','read_present_spread');
  assert.ok(booleanReader.includes('bool::deserialize(deserializer).map(Some)'));
  const reader=booleanReader.replace('read_present_spread','read_present_program_string_representation').replace("D: Deserializer<'de>","D: serde::Deserializer<'de>");
  const flagImport="import {acceptProgramStringRepresentationFlag} from './translation/program-string-representation-rules.js';\n";
  const changes=[
    ['js/src/typed-program-links-validation.js',[
      ["// Validate the source-free checked IR boundary before any target emission.\n",flagImport+"// Validate the source-free checked IR boundary before any target emission.\n"],
      ["  const active = new Set();\n","  if (!acceptProgramStringRepresentationFlag(typeof program.utf16Strings, Object.hasOwn(program, 'utf16Strings'))) throw new TypeError('a program string representation flag must be an explicit boolean');\n  const active = new Set();\n"],
    ]],
    ['js/src/typed-program-checked-projection.js',[
      ["// Rebuild a linked product program through the existing Surface checker.\n",flagImport+"// Rebuild a linked product program through the existing Surface checker.\n"],
      ['  const checked = checkProgram(surface);\n',"  const checked = checkProgram(surface);\n  if (Object.hasOwn(program, 'utf16Strings')) {\n    if (!acceptProgramStringRepresentationFlag(typeof program.utf16Strings, true)) throw new TypeError('a program string representation flag must be an explicit boolean');\n    checked.utf16Strings = program.utf16Strings;\n  }\n"],
    ]],
    ['rust/src/translation/ir.rs',[
      ['/// A checked portable-core program.\n',reader+'\n\n/// A checked portable-core program.\n'],
      ['    pub reference_semantics: bool,\n','    pub reference_semantics: bool,\n    /// Explicit UTF-16 string encoding selection; absence and false stay distinct.\n    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "read_present_program_string_representation")]\n    pub utf16_strings: Option<bool>,\n'],
    ]],
    ['rust/src/translation/check.rs',[
      ['            reference_semantics: self.reference_semantics,\n','            reference_semantics: self.reference_semantics,\n            utf16_strings: None,\n'],
    ]],
    ['rust/src/typed_program_links/checked.rs',[
      ['    let checked = check_program(&surface).map_err(|failure| error(failure.to_string()))?;\n','    let mut checked = check_program(&surface).map_err(|failure| error(failure.to_string()))?;\n    checked.utf16_strings = program.utf16_strings;\n'],
    ]],
    ['rust/src/translation/emit_rust.rs',[
      ['    emitter.utf16_strings =\n        utf16_strings || primitive_methods::program_requires_method_units(program);',
        '    emitter.utf16_strings = super::program_string_representation_rules::select_program_string_representation(\n        program.utf16_strings.unwrap_or(false),\n        utf16_strings,\n        primitive_methods::program_requires_method_units(program),\n    );'],
    ]],
    ['rust/src/translation/mod.rs',[
      ['mod primitive_method_rules;\n','mod primitive_method_rules;\nmod program_string_representation_rules;\n'],
    ]],
    ['rust/tests/unit/mod.rs',[
      ['mod project_data;\n','mod program_string_representation;\nmod project_data;\n'],
    ]],
    ['rust/tests/unit/translation_array_relative_element.rs',[
      ['        let native = emit_rust(&program).unwrap().text;\n','        let native = emit_rust(&program).unwrap().text;\n        let consumed = meta_language::translation::emit_rust::emit_rust_with_array_consumption(\n            &program, meta_language::translation::emit_rust::RustEmissionOptions::default(),\n        ).unwrap().text;\n        assert_eq!(array_relative_native(&consumed, index), expected);\n'],
      ['                    .code,\n                native\n','                    .code,\n                consumed\n'],
    ]],
  ];
  const sources=changes.map(([file,actions])=>{
    const{blob,certificate}=captureGitArtifactCertificate(revision,file),sourceText=blob.toString('utf8');assert.deepEqual(Buffer.from(sourceText),blob);
    let source=sourceText;
    for(const[before,after]of actions){assert.equal(source.split(before).length,2,file+': one complete host edit interval');source=source.replace(before,after);}
    const beforeFormatting=source;
    if(file.endsWith('.rs'))source=formatProductRuleSource(source);
    // Formatting is itself an exact recorded interval; never edit generated files.
    const steps=actions.map(([before,after])=>({before,after}));
    if(source!==beforeFormatting)steps.push({before:beforeFormatting,after:source});
    let inverse=source;
    for(const action of steps.toReversed()){assert.equal(inverse.split(action.after).length,2);inverse=inverse.replace(action.after,action.before);}
    assert.equal(inverse,sourceText);verifyGitArtifactCertificate(certificate,Buffer.from(inverse),{revision,path:file});
    const actual=readFileSync(new URL(file,root),'utf8');assert.ok(actual===sourceText||actual===source,file+': unknown host edits refuse');
    return{file,commit:revision,sourceText,...pin(blob),certificate,actions:steps,proposed:{...pin(source),source}};
  });
  const rules=readProjectData(new URL('parity/self-translation/program-string-representation-rules.lino',root));
  const packet={schemaVersion:1,revision,field:rules.field,nativeField:rules.nativeField,presence:rules.presence,
    primitiveReader:{file:primitiveFile,sourceText:primitive.blob.toString(),...pin(primitive.blob),certificate:primitive.certificate,selectedSource:booleanReader},sources,
    boundary:'Complete mechanical host source proposals with exact original Git membership and byte inverses. Shared checked JavaScript decides boolean admission and encoding selection; actual existing emitters retain all string semantics. No native compilation or execution is inferred. The relative-array test retains and executes its historical emitter and additionally compares the same consuming profile used by the linked emitter.'};
  if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
  if(apply)for(const row of sources)writeFileSync(new URL(row.file,root),row.proposed.source);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>['--check','--apply'].includes(value)));
  const packet=generateProgramStringRepresentationHosts({check:process.argv.includes('--check'),apply:process.argv.includes('--apply')});
  console.log('Prepared '+packet.sources.length+' complete program string-representation host source adapters; native checks remain in CI.');
}
