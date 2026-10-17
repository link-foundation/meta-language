import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';
const scalarCases=[
 ['zero',String.raw`\u{0}`,0],
 ['zero-trailing',String.raw`\u{0____}`,0],
 ['one-digit',String.raw`\u{a___}`,10],
 ['separated-ascii',String.raw`\u{4_1_}`,65],
 ['mixed-case',String.raw`\u{Ab_Cd_}`,43981],
 ['six-zero-digits',String.raw`\u{0_0__0___0____0_____0______}`,0],
 ['six-padded-digits',String.raw`\u{0_0_0_0_0_1____}`,1],
 ['before-surrogate',String.raw`\u{D_7_F_F_}`,55295],
 ['after-surrogate',String.raw`\u{E_0_0_0_}`,57344],
 ['noncharacter-first',String.raw`\u{F_D_D_0_}`,64976],
 ['noncharacter-before-bmp-end',String.raw`\u{F_F_F_E_}`,65534],
 ['noncharacter-bmp-end',String.raw`\u{F_F_F_F_}`,65535],
 ['first-astral',String.raw`\u{1_0_0_0_0_}`,65536],
 ['astral-face',String.raw`\u{1_f_6_0_0_}`,128512],
 ['plane-noncharacter',String.raw`\u{1_F_F_F_E_}`,131070],
 ['maximum-scalar',String.raw`\u{1_0_F_F_F_F_}`,1114111],
 ['long-trailing',String.raw`\u{41`+'_'.repeat(64)+'}',65],
];
const refusalCases=[
 ['empty',String.raw`\u{}`,'malformed'],['only-underscore',String.raw`\u{_}`,'malformed'],['leading-underscore',String.raw`\u{_41}`,'malformed'],['six-plus-one',String.raw`\u{0_0_0_0_0_0_0_}`,'malformed'],['seven-zero-digits',String.raw`\u{0000000}`,'malformed'],['non-hex',String.raw`\u{4_g}`,'malformed'],['whitespace',String.raw`\u{4 1}`,'malformed'],['unclosed',String.raw`\u{41___`,'malformed'],['missing-open',String.raw`\u41}`,'malformed'],['fixed-width',String.raw`\u0041`,'malformed'],['uppercase-prefix',String.raw`\U{41}`,'malformed'],['ascii-prefix',String.raw`\x41`,'malformed','A'],['high-surrogate',String.raw`\u{D_8_0_0_}`,'unsupported'],['low-surrogate',String.raw`\u{D_F_F_F_}`,'unsupported'],['above-maximum',String.raw`\u{1_1_0_0_0_0_}`,'unsupported'],['six-hex-maximum',String.raw`\u{F_F_F_F_F_F_}`,'unsupported'],
];
export function rustUnicodeEscapeFixture(){return {schemaVersion:1,model:'Rust Unicode scalar escape 1',reference:'https://doc.rust-lang.org/reference/tokens.html#character-escapes',scalarCases:scalarCases.map(([id,escape,code])=>({id,escape,code,value:String.fromCodePoint(code)})),refusalCases:refusalCases.map(([id,escape,kind,sourceValue])=>({id,escape,result:kind==='unsupported'?{$:kind,end:escape.length,escapeLength:escape.length}:{$:kind},...(sourceValue!==undefined?{sourceValue}:{})}))};}
if(process.argv[1]===fileURLToPath(import.meta.url)){const file=new URL('../../parity/fixtures/rust-unicode-escapes.lino',import.meta.url),fixture=rustUnicodeEscapeFixture();if(process.argv.includes('--check'))checkProjectData(file,fixture);else writeProjectData(file,fixture);console.log('Rust Unicode escapes: '+fixture.scalarCases.length+' scalar cases and '+fixture.refusalCases.length+' refusal cases');}
