import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readOwnCodebaseHistoricalNamingPolicy} from '../scripts/own-codebase-baseline-naming-history.mjs';

test('registered root tooling definitions retain their complete preceding source and naming identity',()=>{
 const file='scripts/lib/structured-data-bootstrap.mjs',revision='3d8e4513e2167d0d15cca13ccaefcb4ae86669fd';
 const history=readOwnCodebaseHistoricalNamingPolicy().additionalSourceViews;
 const source=history.sources.find(row=>row.file===file&&row.predecessor.revision===revision);assert.ok(source,'root scripts participate alongside package and experimental sources');
 const view=history.views.find(row=>row.file===file&&row.revision===revision&&row.inventory==='B70 physical field source view names for '+file);assert.ok(view);
 assert.ok(view.names.some(row=>row.record==='repository.evaluate-lazy-concrete-syntax-tree-comparison-and-source-binding-module-thirty-three-source-inspect-bootstrap-structured-data-lino'));
 assert.ok(source.predecessor.source.includes('export function inspectBootstrapStructuredDataLino('));
 const actual=readFileSync(new URL('../../'+file,import.meta.url));
 assert.deepEqual(source.current,{bytes:actual.length,sha256:createHash('sha256').update(actual).digest('hex')});
 assert.notEqual(source.predecessor.sha256,source.current.sha256,'the changed current definition retains a separately qualified source');
});
