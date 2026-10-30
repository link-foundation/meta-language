// Source typing/emission and actual host-I/O construction only.
import {readFileSync,existsSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {draftNativeSourceInterfaces} from './draft-native-source-interfaces.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),path='js/src/translation/rust-record-rules.js',source=readFileSync(new URL(path,root),'utf8');parseAuthoredJavaScriptModule(source,path);
const program=checkProgram(parseJavaScript(source)),emitted=emitRust(program,{arrayConsumption:true}),file=new URL('parity/translation/rust-source-render-interface.lino',root);
const native=readProjectData(new URL('parity/self-translation/rust-record-rules-native.lino',root));
if(native.sourceClosure.source!==source||native.emitted.text!==emitted.text)throw new TypeError('source interface construction requires the complete current generated native library');
const functions=['planRustRecordFields','readRustRecordFieldPosition','planRustRecordPatternFields','planRustRecordUpdateFields',
 'acceptRustRecordUpdateOwner','acceptRustIrrefutablePatternNode','planTuplePatternFields','readRustLetElseInitializerError',
 'readRustRecordBindingNames','readRustRecordConstructorName','acceptRustRecordSourceName','acceptRustKeywordName',
 'acceptRustRawIdentifierName','acceptRustSourceKeywordToken','readRustRawIdentifierError','renderRustConstructorFieldName',
 'acceptRustNamedConstructorFields','renderRustConstructorFields','renderRustFunctionCall','renderRustDataFieldEncoding'];
if(!existsSync(file))writeProjectData(file,{schemaVersion:1,model:'native Rust source Unicode scalar interface 1',module:'rust_record_rules',functions:functions.map(source=>({source}))});
const definition=readProjectData(file);
if(definition.functions.map(row=>row.source).join('\n')!==functions.join('\n'))throw new TypeError('all actual source record operations require their explicitly registered interfaces');
draftNativeSourceInterfaces({program,emitted,definition,module:'rust_record_rules',output:'rust/src/translation/rust_source_render_interfaces.rs',receipt:'parity/self-translation/rust-source-render-interfaces.lino'});
