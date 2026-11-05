// Declarations only. No source initializer, classifier, translator or fixture runs.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
writeProjectData(new URL('../../parity/fixtures/intrinsic-module-initializers.lino',import.meta.url),{
 schemaVersion:1,model:'complete intrinsic allocation and deferred body module scope interpretation 1',
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 references:['https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-regular-expression-literals-runtime-semantics-evaluation','https://tc39.es/ecma262/2025/multipage/text-processing.html#sec-regexpcreate','https://tc39.es/ecma262/2025/multipage/text-processing.html#sec-regexpalloc'],
 positives:[
  {source:'const TABLE = {head: /^([a-z]+)/u, tail: /end$/u};',name:'TABLE',owner:0},
  {source:'export const TABLE = {normalize: value => value, test(value) { return value; }};',name:'TABLE',owner:0},
  {source:'const TABLE = {get value() { return sideEffect(); }, set value(next) { sideEffect(next); }};',name:'TABLE',owner:0},
  {source:'const TABLE = {later: (value = sideEffect()) => value};',name:'TABLE',owner:0},
  {source:'const TABLE = [function named(value) { return sideEffect(value); }, /pattern/];',name:'TABLE',owner:0},
  {source:'const TABLE = new Map([["pattern", /value/], ["function", value => value]]);',name:'TABLE',owner:1},
  {source:'const TABLE = new Set([/pattern/, value => value]);',name:'TABLE',owner:2},
  {source:'const TABLE = Object.freeze({pattern: /value/, later: value => value});',name:'TABLE',owner:3},
 ],
 negatives:[
  'const TABLE = {value: sideEffect()};','const TABLE = {[sideEffect()]: 1};',
  'const TABLE = { ...foreign };','const TABLE = [ ...foreign ];',
  'const TABLE = (() => sideEffect())();','const TABLE = new Foreign([/value/]);',
  'const TABLE = new Set([sideEffect()]);','const TABLE = Object.freeze({get [sideEffect()]() { return 1; }});',
  'const TABLE = new Map([["first", /x/]], sideEffect());','const TABLE = {value: foreign.property};',
  'const TABLE = class extends sideEffect() {};','const TABLE = `${sideEffect()}`;',
  'const TABLE = /first/, OTHER = /second/;',
 ],
 scope:[
  {forms:['named function declaration','intrinsic value binding'],owners:[-1,0],bindings:['keep','TABLE'],accepted:true},
  {forms:['named function declaration','intrinsic value binding'],owners:[-1,1],bindings:['keep','TABLE'],accepted:true},
  {forms:['intrinsic value binding'],owners:[1],bindings:['Map','TABLE'],accepted:false},
  {forms:['intrinsic value binding'],owners:[2],bindings:['Set','TABLE'],accepted:false},
  {forms:['intrinsic value binding'],owners:[3],bindings:['Object','TABLE'],accepted:false},
  {forms:['unknown module form','intrinsic value binding'],owners:[-1,0],bindings:['TABLE'],accepted:false},
 ],
 module:'const TABLE = {head: /^a/u};\nexport function keep(value) { return Math.floor(value); }\n',
 boundary:'Scope classification does not translate or execute an initializer, publish its signature, validate complete RegExp semantics or claim a whole module is executable. Literal RegExp creation uses the intrinsic constructor, not an arbitrary source callback; abrupt completion remains a separate semantic obligation. Every legacy classifier/refusal remains unchanged.',
});
