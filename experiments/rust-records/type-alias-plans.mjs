// Future paired declaration-role products; never run during drafting.
import {readProjectData} from '../../js/src/project-data-files.js';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {tokenize} from '../../js/src/translation/lexer.js';
import {readRustTypeAliasDeclarationPlan} from '../../js/src/translation/rust-type-alias-rules.js';
const [major,minor]=process.versions.node.split('.').map(Number);
if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/rust-type-aliases.lino',import.meta.url));
const plans=fixture.plans.map(row=>{
 const tokens=tokenize(row.source,'Rust').tokens;
 const kinds=tokens.map(token=>token.kind==='identifier'&&['mod','type'].includes(token.value)&&token.raw===token.value?'keyword':['identifier','punct','eof'].includes(token.kind)?token.kind:'atom');
 const values=tokens.map(token=>token.value);
 return {...row,kinds,values,actual:readRustTypeAliasDeclarationPlan(kinds,values)};
});
process.stdout.write(structuredDataToLino({plans,versions:fixture.versions,boundary:'Actual future source products; no execution evidence is recorded.'}));
