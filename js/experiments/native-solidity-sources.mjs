// Classifies hand-written Solidity sources for the fixture: for each, whether
// the tree-sitter-solidity oracle recovers, and whether the native grammar
// rejects it or builds the oracle rows (SAME) or not (DIFF).
//   node experiments/native-solidity-sources.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../../parity/grammars/native/solidity.lino', import.meta.url), 'utf8');
const parser = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token', 'hex_digit'], extras: ['comment'], oracleKinds: nativeOracleKinds(text) };
const sources = [
  '', 'pragma solidity ^0.8.0;\n', 'contract C {}\n', 'contract C { uint x; }\n', 'contract C { uint256 public x = 1; }\n',
  'contract C { function f() public {} }\n', 'contract C { function f(uint a) external pure returns (uint) { return a + 1; } }\n',
  'interface I { function f() external; }\n', 'library L { function f() internal {} }\n', 'abstract contract A is B, C {}\n',
  'contract C { event E(uint indexed a); }\n', 'contract C { error E(uint a); }\n', 'contract C { modifier m() { _; } }\n',
  'contract C { struct S { uint a; } }\n', 'contract C { enum E { A, B } }\n', 'contract C { mapping(address => uint) m; }\n',
  'contract C { constructor() {} }\n', 'contract C { receive() external payable {} fallback() external {} }\n',
  'import "a.sol";\n', 'import {A as B} from "a.sol";\n', 'import * as A from "a.sol";\n', 'using L for uint;\n',
  'type T is uint;\n', 'uint constant X = 1;\n', 'function f() pure returns (uint) { return 1; }\n',
  'contract C { function f() public { if (a) { b(); } else { c(); } } }\n', 'contract C { function f() public { for (uint i = 0; i < 1; i++) {} } }\n',
  'contract C { function f() public { while (a) { break; } do { continue; } while (b); } }\n',
  'contract C { function f() public { emit E(1); revert E(); require(a, "é"); } }\n',
  'contract C { function f() public { try g() returns (uint a) {} catch Error(string memory s) {} catch {} } }\n',
  'contract C { function f() public { assembly { let x := add(1, 2) } } }\n', 'contract C { function f() public { unchecked { a++; } } }\n',
  'contract C { function f() public { (uint a, , uint b) = g(); } }\n', 'contract C { function f() public { a = b ? c : d; } }\n',
  'contract C { function f() public { x = new D{value: 1}(2); } }\n', 'contract C { function f() public { x = a[1:2]; delete a; } }\n',
  'contract C { string s = unicode"é"; bytes b = hex"00ff"; }\n', 'contract C { uint x = 1 ether + 2 gwei; }\n', '// é\n/* é */\n',
  '/// @notice é\ncontract C {}\n', 'contract C { function f() public virtual override(A, B) {} }\n',
  // Invalid input.
  'contract C {', 'contract C { function f( }\n', 'contract C { uint x = ; }\n', 'pragma solidity\n', 'contract { }\n', 'import ;\n',
  'contract C { function f() public { if (a { } } }\n', 'contract C { function f() public { x = (1; } }\n', 'contract C { struct S { } \n',
  'contract C { function f() public { return } }\n',
];
for (const source of sources) {
  let kind;
  if (oracleRecovers(source, 'Solidity')) kind = parser.parseTree(source).ok ? 'RECOVERS-ACCEPTED' : 'RECOVERS-REJECTED';
  else {
    const outcome = parser.parseTree(source);
    if (!outcome.ok) kind = 'REJECT';
    else kind = JSON.stringify(nativeRows(outcome.tree, source, options)) === JSON.stringify(oracleRows(source, 'Solidity')) ? (outcome.ambiguities?.length ? 'SAME-AMBIGUOUS' : 'SAME') : 'DIFF';
  }
  console.log(kind, JSON.stringify(source));
}
