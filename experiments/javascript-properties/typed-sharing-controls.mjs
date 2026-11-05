// Deferred graph identity/source semantics controls, never imported by authors.
import assert from 'node:assert/strict';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../../js/src/typed-program-links.js';
export const sharingFixture=readProjectData(new URL('../../parity/fixtures/typed-program-value-sharing.lino',import.meta.url));
export function observeTypedSharing(row){
 const program=checkProgram(parseJavaScript(row.source)),legacy=encodeTypedProgramLinks(program),shared=encodeTypedProgramLinks(program,{valueSharing:sharingFixture.model});
 assert.ok(shared.network.links().length<legacy.network.links().length,'exact value sharing must reduce actual nodes');
 const counts=packet=>packet.network.links().filter(link=>link.metadata().term===TYPED_PROGRAM_LINKS_SCHEMA.scalars.string&&link.metadata().definition===row.parameter).length;
 assert.equal(counts(shared),1);assert.ok(counts(legacy)>1);
 const relations=shared.network.fieldRelations(),targets=new Set(relations.filter(row=>row.label===TYPED_PROGRAM_LINKS_SCHEMA.references.binding).map(row=>String(row.child.asU64())));
 const label=name=>TYPED_PROGRAM_LINKS_SCHEMA.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
 const parameter=name=>{const owner=relations.find(row=>row.label===label('fullName')&&shared.network.link(row.child).metadata().definition===name)?.parent;assert.ok(owner);const params=relations.find(row=>String(row.parent.asU64())===String(owner.asU64())&&row.label===label('params'))?.child;assert.ok(params);return String(shared.network.link(params).references()[0].asU64());};
 const firstParameter=parameter('first'),secondParameter=parameter('second');assert.notEqual(firstParameter,secondParameter);assert.ok(targets.has(firstParameter));assert.ok(targets.has(secondParameter));
 const first=decodeTypedProgramLinks(legacy.network,legacy.root),second=decodeTypedProgramLinks(shared.network,shared.root);assert.deepEqual(second,first);
 assert.notStrictEqual(second.declarations.get('first').params[0],second.declarations.get('second').params[0]);
 assert.equal(runInNewContext(row.source+'\ncompute();'),row.expected);
 const product=emitJavaScript(second),entry=product.mappings.find(row=>row.kind==='function'&&row.source==='compute').target;assert.equal(runInNewContext(product.text+'\n'+entry+'();'),row.expected);
 return {...row,legacy:legacy.network.toLino(),legacyRoot:String(legacy.root.asU64()),shared:shared.network.toLino(),sharedRoot:String(shared.root.asU64())};
}
