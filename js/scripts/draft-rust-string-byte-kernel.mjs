// Author actual production source and a complete closed lambda; never run it.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {draftSourceLibrary} from './draft-source-library.mjs';
import {readCallableCapturedBindings} from '../src/translation/callable-source.js';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const sourcePath='js/src/translation/rust-string-byte-kernel.js',source=readFileSync(new URL('../../'+sourcePath,import.meta.url),'utf8'),surface=parseJavaScript(source),checked=checkProgram(surface),originalDeclaration=surface.items.find(item=>item.k==='fn'&&item.name==='encodeRustStringUtf8');
if(!originalDeclaration||surface.items.length!==1||checked.items.length!==1)throw new TypeError('the byte kernel must retain one complete source declaration');
if(readCallableCapturedBindings(checked.items[0].body,checked.items[0].params.map(parameter=>parameter.name)).size!==0)throw new TypeError('the byte kernel must retain its complete closed source body');
const template={k:'lambda',params:structuredClone(originalDeclaration.params),ret:structuredClone(originalDeclaration.ret),body:structuredClone(originalDeclaration.body),span:originalDeclaration.span};
const policy={schemaVersion:1,model:'portable Rust scalar string UTF8 byte lambda 1',sourcePath,source,sourceSha256:createHash('sha256').update(source).digest('hex'),originalDeclaration,template,versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},boundary:'A complete source-derived closed kernel, not an execution observation. Iterates Unicode scalar points and flat-maps their byte packets; no repeated append to a growing full output array. Rust source type excludes isolated UTF16 surrogates.'};
for(const path of ['js/src/translation/data/rust-string-byte-template.lino','rust/src/data/rust-string-byte-template.lino'])writeProjectData(new URL('../../'+path,import.meta.url),policy);
draftSourceLibrary({sourcePath,moduleName:'rust_string_byte_kernel',model:'Rust scalar string UTF8 byte kernel 1',programAsset:'rust-string-byte-kernel-program'});
