// Counts web-tree-sitter parser setups and tree deletions while parsing a
// Markdown document with many inline regions.
import { Parser, Tree } from 'web-tree-sitter';

let setups = 0;
let deleted = 0;
const setLanguage = Parser.prototype.setLanguage;
Parser.prototype.setLanguage = function (...args) { setups += 1; return setLanguage.apply(this, args); };
const remove = Tree.prototype.delete;
Tree.prototype.delete = function (...args) { deleted += 1; return remove.apply(this, args); };
const { LinkNetwork } = await import('../src/index.js');
const document = Array.from({ length: 200 }, (_, index) => `Paragraph *${index}* with \`code\`.\n`).join('\n');
LinkNetwork.parse(document, 'Markdown');
const first = { setups, deleted };
LinkNetwork.parse(document, 'Markdown');
console.log(JSON.stringify({ first, second: { setups, deleted } }));
