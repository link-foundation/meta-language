// Current JavaScript policy is the oracle, including carried statement/comment items.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {tokenize} from '../../../../js/src/translation/lexer.js';
import {selfTranslate} from '../../../../js/src/self-translation.js';
import {checkProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
import {selectVersionedAssertion} from '../../../../js/scripts/assertion-version-ranges.mjs';

const root=new URL('../../../../',import.meta.url);
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});

/** Retain the exact native input and every ordered current JavaScript decision. */
export function generateRelativeCharacterSelfTranslation({check=false}={}) {
  const file='rust/tests/unit/translation_primitive_methods.rs',source=readFileSync(new URL(file,root),'utf8');
  const tokens=tokenize(source,'Rust').tokens,name='relative_character_keeps_default_order_complete_archives_and_lossless_source';
  const owners=tokens.flatMap((token,index)=>token.value==='fn'&&tokens[index+1]?.value===name?[index]:[]);assert.equal(owners.length,1);
  const start=owners[0],body=tokens.findIndex((token,index)=>index>start&&token.value==='{');assert.ok(body>start);
  let depth=0,end=-1;
  for(let index=body;index<tokens.length;index++){const token=tokens[index];if(token.kind!=='punct')continue;if(token.value==='{')depth++;else if(token.value==='}'&&--depth===0){end=index;break;}}
  assert.ok(end>body);
  const declarations=[];
  for(let index=body+1;index<end;index++)if(tokens[index].value==='let'&&tokens[index+1]?.value==='source'&&tokens[index+2]?.value==='='&&tokens[index+3]?.kind==='string')declarations.push(tokens[index+3]);
  assert.equal(declarations.length,1);const input=declarations[0].value;
  const translated=selfTranslate(input,'JavaScript','Rust');
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,input,'every input byte remains in the existing exact source round trip');
  const statuses=translated.items.map(item=>item.status);
  assert.equal(statuses.filter(status=>status==='translated').length,2,'both complete function declarations translate');
  assert.equal(statuses.length,8,'all original declarations comments and effectful statements remain represented');
  const packageSource=readFileSync(new URL('js/package.json',root),'utf8'),version=JSON.parse(packageSource).version;
  assert.match(version,/^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$/u,'covered stable project version required');
  const coordinates=version.split('.').map(Number);assert.ok(coordinates.every(Number.isSafeInteger)&&coordinates[2]<Number.MAX_SAFE_INTEGER);
  const maximum=coordinates[0]+'.'+coordinates[1]+'.'+(coordinates[2]+1);
  const versionPolicy={id:'relative-character-current-self-translation-items',subject:'meta-language',comparison:{kind:'release',precision:3},
    expectations:[{id:'current-source-qualified-item-decisions',minimum:version,maximum,includeMinimum:true,includeMaximum:false,expected:statuses,
      evidence:'Current JavaScript source-qualified decisions and exact restoration for the complete retained native input; carried statements/comments remain explicit translation obligations.'}]};
  assert.deepEqual(selectVersionedAssertion(versionPolicy,version).expected,statuses);
  const fixture={schemaVersion:1,source:{file,owner:name,text:input,...pin(input)},
    implementation:{file:'js/src/self-translation.js',...pin(readFileSync(new URL('js/src/self-translation.js',root)))},
    package:{file:'js/package.json',version,...pin(packageSource)},versionPolicy,statuses,
    decisions:translated.items.map(item=>({status:item.status,reason:item.reason??null})),
    boundary:'Actual current JavaScript decisions on the complete unchanged native input, including carried top-level statements and comments. Native parity and lossless restoration remain CI requirements. This does not establish complete semantic translation of carried code or remove any full language translation obligation.'};
  const output=new URL('../../../../parity/fixtures/relative-character-self-translation.lino',import.meta.url);
  if(check)checkProjectData(output,fixture);else writeProjectData(output,fixture);
  return fixture;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateRelativeCharacterSelfTranslation({check:process.argv.includes('--check')});
  console.log('Captured every current JavaScript item decision and exact source restoration; carried code remains an explicit translation obligation.');
}
