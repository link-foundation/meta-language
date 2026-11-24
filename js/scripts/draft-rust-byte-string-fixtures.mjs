// Independent literal declarations, never the decoder or lexer under draft.
import {writeProjectData} from './project-data.mjs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
writeProjectData(new URL('../../parity/fixtures/rust-byte-string-literals.lino',import.meta.url),{
 schemaVersion:1,model:'Rust byte string lexical source 1',
 versions:{Rust:'>=1.90.0 <1.100.0; edition 2024',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},
 references:['https://doc.rust-lang.org/reference/tokens.html#byte-string-literals','https://doc.rust-lang.org/reference/tokens.html#raw-byte-string-literals','https://doc.rust-lang.org/reference/tokens.html#suffixes'],
 legacyRawPrefixExpectations:[
  {source:'r#name',kind:'identifier'},{source:'r##name',kind:'identifier'},
  {source:'br"value"',kind:'byteString'},{source:'br#"value"#',kind:'byteString'},
  {source:'cr"value"',kind:'identifier'},{source:'cr#"value"#',kind:'identifier'},
 ],
 positives:[
  {source:'b""',value:''},{source:'b"ASCII"',value:'4153434949'},
  {source:String.raw`b"\x00\x7f\x80\xFF"`,value:'007f80ff'},
  {source:String.raw`b"\n\r\t\0\\\"\'"`,value:'0a0d09005c2227'},
  {source:'b"left\\\n \t\r\nright"',value:'6c6566747269676874'},
  {source:'b"a\r\nb"',value:'610a62'},
  {source:'br"\\n"',value:'5c6e'},
  {source:'br##"a"#b"##',value:'61222362'},
  {source:'br#"a\r\nb"#',value:'610a62'},
  {source:'b"value"suffix',value:'76616c7565',suffix:'suffix'},
  {source:'br#"x"#λ',value:'78',suffix:'λ'},
 ],
 negatives:[
  {source:'b"é"',reason:'non-ASCII source'},
  {source:'br#"😀"#',reason:'non-ASCII source'},
  {source:String.raw`b"\u{41}"`,reason:'Unicode escape'},
  {source:String.raw`b"\xG0"`,reason:'malformed Rust byte string escape'},
  {source:String.raw`b"\q"`,reason:'malformed Rust byte string escape'},
  {source:'b"a\rb"',reason:'isolated carriage return'},
  {source:'br#"a\rb"#',reason:'isolated carriage return'},
  {source:'b"unterminated',reason:'unterminated Rust byte string'},
  {source:'br##"unclosed"#',reason:'unterminated Rust byte string'},
  {source:'br'+'#'.repeat(256)+'"x"'+'#'.repeat(256),reason:'255 delimiter hashes'},
 ],
 boundary:'Unexecuted future controls. Hex payloads denote bytes, not text. Source spans, arbitrary lexical suffixes, CRLF and continuations are retained; static byte-array ownership semantics remain a separate typed translation obligation.',
});
const revision=execFileSync('git',['rev-parse','6fe2658d'],{encoding:'utf8'}).trim();
const sources=['js/tests/translation-rust-raw-string-literals.test.js','rust/tests/unit/translation_rust_raw_string_literals.rs'].map(path=>{
 const source=execFileSync('git',['show',revision+':'+path],{encoding:'utf8',maxBuffer:2*1024*1024});
 return {path,source,bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')};
});
writeProjectData(new URL('../../parity/fixtures/rust-byte-string-assertion-history.lino',import.meta.url),{
 schemaVersion:1,model:'Rust byte string prefix assertion correction 1',revision,sources,
 inputInventory:['r#name','r##name','br"value"','br#"value"#','cr"value"','cr#"value"#'],
 previous:'Every prefix outside the text raw-string decoder was expected to start with an identifier token.',
 current:'The text raw-string decoder still returns notRaw for every input. The full lexer now classifies actual br byte-string literals as byteString, preserving their full raw token and byte payload; all other original inputs retain their original expectations and typed refusals.',
 evidence:['https://doc.rust-lang.org/reference/tokens.html#raw-byte-string-literals','parity/fixtures/rust-byte-string-literals.lino'],
 versions:{Rust:'>=1.90.0 <1.100.0; edition 2024',profile:'Rust byte string lexical source 1'},
 boundary:'Complete original assertion sources and inputs retained. This is source/specification evidence for correcting an obsolete lexer expectation, not a recorded execution, pass observation or semantic-equivalence certificate.',
});
