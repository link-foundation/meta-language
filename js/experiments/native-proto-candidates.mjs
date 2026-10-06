// Classifies candidate Protocol Buffers sources for the native grammar
// fixture: MATCH (the oracle reads it and the native tree agrees), DIFF, REJECT
// (the oracle reads it, the native grammar does not), RECOVERS (the oracle
// recovers; the native grammar must reject and repair it) or ACCEPTS (the
// oracle recovers but the native grammar accepts).
//   node experiments/native-proto-candidates.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../../parity/grammars/native/proto.lino', import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['comment'], oracleKinds: nativeOracleKinds(text) };
const language = 'Protocol Buffers';
const candidates = [
  '', 'syntax = "proto3";', "syntax = 'proto2';", 'edition = "2023";', 'package a.b.c;', 'import "x.proto";', 'import public "x.proto";',
  'import weak "x.proto";', 'option java_package = "com.x";', 'option (my.opt).sub = 1;', 'option (a) = { b: 1 c: "s" };', 'option a = 1.5e3;',
  'option a = -inf;', 'option a = nan;', 'option a = true;', 'option a = "x" "y";', 'option a = "\\x41\\101\\n";', 'message M {}', 'message M { int32 a = 1; }',
  'message M { optional string a = 1 [deprecated = true]; }', 'message M { repeated M.N a = 1; }', 'message M { map<string, int32> m = 1; }',
  'message M { message N { bool b = 1; } }', 'message M { enum E { A = 0; B = 1 [(x) = 2]; } }', 'message M { oneof o { string a = 1; int32 b = 2; } }',
  'message M { reserved 1, 2 to 5, 9 to max; reserved "a", "b"; }', 'message M { extensions 100 to 199; }', 'message M { ; }', 'message M { .a.B f = 1; }',
  'enum E { A = 0; B = -1; option allow_alias = true; }', 'enum E { A = 0x1F; B = 017; }', 'service S { rpc F (Req) returns (Res); }',
  'service S { rpc F (stream Req) returns (stream Res) { option (x) = 1; } }', 'extend google.protobuf.MessageOptions { string my = 50000; }',
  '// c\nmessage M {}', '/* c */ message M {}', 'message M { int32 a = 1; // trailing\n }', 'syntax = "proto3";\npackage p;\nimport "a.proto";\nmessage M {\n  string é = 1;\n}\n',
  'message M { required group G = 1 { optional int32 a = 2; } }',
  // rejections
  'message', 'message M', 'message M {', 'message M { int32 a; }', 'message M { int32 a = ; }', 'syntax = ;', 'syntax "proto3";', 'import;', 'package;',
  'enum E { A }', 'service S { rpc F (A) (B); }', 'option = 1;', 'message M { map<string> m = 1; }', '}', 'message M { int32 a = 1 }', '"unterminated',
  'message M { int32 a = 1; } }', 'service S { rpc F (A) returns B; }', 'enum E { A = ; }', 'message M { reserved; }',
];
for (const source of candidates) {
  let kind;
  if (oracleRecovers(source, language)) kind = compiled.parseTree(source).ok ? 'ACCEPTS' : (compiled.parseTree(source, { errorRecovery: true }).rejection?.reason === 'recovered' ? 'RECOVERS' : 'NO-REPAIR');
  else {
    const outcome = compiled.parseTree(source);
    if (!outcome.ok) kind = 'REJECT';
    else kind = JSON.stringify(nativeRows(outcome.tree, source, options)) === JSON.stringify(oracleRows(source, language)) ? 'MATCH' : 'DIFF';
  }
  console.log(kind, JSON.stringify(source));
}
