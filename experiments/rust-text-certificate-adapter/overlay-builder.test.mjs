import assert from 'node:assert/strict';
import test from 'node:test';
import {relocateCheckedModuleImports} from './overlay.mjs';

test('lexical module relocation preserves active imports and resource bases without changing inert expressions',()=>{
 const source='import value from "./provider.js";\nexport {value} from "./export.js";\nexport const data=new URL("./data.lino",import.meta.url);\nconst inert="import.meta.url";\nfrom\n"./inert.js";\nconst property=object.import.meta.url;\nconst lazy=import("./lazy.js");';
 const origin='file:///original/module.js',linked=relocateCheckedModuleImports(source,origin,url=>url==='file:///original/provider.js'?'file:///overlay/provider.js':url);
 assert.ok(linked.includes('from "file:///overlay/provider.js"'));assert.ok(linked.includes('new URL("./data.lino","file:///original/module.js")'));assert.ok(linked.includes('inert="import.meta.url"'));
 assert.ok(linked.includes('from "file:///original/export.js"'));assert.ok(linked.includes('from\n"./inert.js"'));assert.ok(linked.includes('object.import.meta.url'));assert.ok(linked.includes('import("file:///original/lazy.js")'));
});
