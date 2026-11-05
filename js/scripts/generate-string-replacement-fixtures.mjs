// Source declarations only. Do not invoke replacements or behavioral oracles.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['first literal match','return "aba".replace("a","x");','xba'],
 ['all literal matches','return "aba".replaceAll("a","x");','xbx'],
 ['nonoverlapping replacement','return "aaaaa".replaceAll("aa","X");','XXa'],
 ['empty single search','return "ab".replace("","-");','-ab'],
 ['empty all search','return "ab".replaceAll("","-");','-a-b-'],
 ['empty all input boundary','return "".replaceAll("","-");','-'],
 ['empty search visits UTF16 boundaries','return "A😀B".replaceAll("","|").length;','9'],
 ['empty search preserves high half','return "A😀B".replaceAll("","|").charCodeAt(3);','55357'],
 ['doubled dollar is literal','return "ab".replace("a","$$");','$b'],
 ['matched text token','return "aba".replaceAll("a","$&$&");','aabaa'],
 ['original prefix token','return "aba".replaceAll("a","$`");','bab'],
 ['original suffix token',`return "aba".replaceAll("a","$'");`,'bab'],
 ['all context token roles', "return \"abc\".replace(\"b\",\"[$`][$&][$']\");",'a[a][b][c]c'],
 ['capture-looking text without captures','return "abc".replace("b","$1|$01|$99|$<name>|$0");','a$1|$01|$99|$<name>|$0c'],
 ['sequential dollar tokens','return "abc".replaceAll("b","$$$&");','a$bc'],
 ['escaped dollar does not rescan','return "abc".replaceAll("b","$$&");','a$&c'],
 ['unmatched trailing dollar','return "abc".replace("b","$");','a$c'],
 ['no match retains input','return "abc".replaceAll("z","$&$`");','abc'],
 ['replacement text does not recursively match','return "aaaa".replaceAll("aa","aaa");','aaaaaa'],
 ['isolated units survive literal replacement','return "\\ud800".replace("\\ud800","\\udfff").charCodeAt(0);','57343'],
 ['surrogate half search remains a unit','return "😀".replace("\\ud83d","").charCodeAt(0);','56832'],
 ['Number search and Boolean replacement','return "a1b".replace(1,true);','atrueb'],
 ['null search and BigInt replacement','return "null".replace(null,12n);','12'],
 ['omitted replacement converts undefined','return "a".replace("a");','undefined'],
 ['omitted search converts undefined','return "xundefinedy".replace();','xundefinedy'],
 ['all omitted arguments','return "undefined undefined".replaceAll();','undefined undefined'],
 ['optional absent replacement','const values=["x"];return "aba".replaceAll("a",values[1]);','undefinedbundefined'],
 ['ignored extra argument retains effects','let value=0;const text="aba".replaceAll("a","x",value=7);return text+value;','xbx7'],
 ['earlier argument capture precedes later write','let search="a";return "aba".replaceAll(search,"x",search="b");','xbx'],
 ['replacement capture precedes later write','let replacement="x";return "aba".replaceAll("a",replacement,replacement="y");','xbx'],
 ['own replace remains callable property','const item={replace(value){return value+1;}};return item.replace(8);','9'],
 ['runtime helper name remains source binding','const ml_utf16_text=7;return "aba".replaceAll("a","x")+ml_utf16_text;','xbx7'],
];
const cases=inputs.map(([name,body,value],index)=>({name,source:'export function compute(){'+body+'}',entry:'stringReplacementCase'+index,value,abort:null}));
for(const operation of ['replace','replaceAll']){
 const index=cases.length,helper='failStringReplacementExtra'+index,abort='replacement extra';
 cases.push({name:operation+' extra throw precedes operation',source:'/** @returns {number} */ function '+helper+'(){throw new SyntaxError("replacement extra");} export function compute(){return "abc".'+operation+'("z","x",'+helper+'());}',entry:'stringReplacementExtra'+index,value:null,abort,errorClass:'SyntaxError',messagePresent:true,messageUnits:Array.from({length:abort.length},(_,at)=>abort.charCodeAt(at))});
}
writeProjectData(new URL('../../parity/fixtures/string-replacement.lino',import.meta.url),{
 schemaVersion:1,model:'shared literal UTF16 source replacement and substitution 1',sourceLanguage:'JavaScript',cases,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/text-processing.html#sec-string.prototype.replace ; https://tc39.es/ecma262/2025/multipage/text-processing.html#sec-getsubstitution ; https://tc39.es/ecma262/2025/multipage/text-processing.html#sec-string.prototype.replaceall',
 boundary:'Future original source, values/errors, native products, complete UTF16 definition graph, both codecs and all emissions. No observations. Literal search/no-capture primitive template domain with successful allocation. RegExp/Symbol.replace, callable replacement, generic receivers, dynamic unions, full resource-limit/failure equivalence and all preceding obligations remain required.',
});
