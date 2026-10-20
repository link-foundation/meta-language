// CI source freshness selects this loader only for two immutable check CLIs.
import assert from 'node:assert/strict';
import {registerHooks} from 'node:module';
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {satisfies} from './semver-range.mjs';
import {SCHEMA_COMMAND_SOURCE_PROFILE,renderSchemaCommandSourceView} from './schema-command-source-profile.mjs';

const root=new URL('../../',import.meta.url);
assert.ok(satisfies(process.versions.node,SCHEMA_COMMAND_SOURCE_PROFILE.nodeRange,'npm'),'uncovered schema command source-loader Node.js version: '+process.versions.node);
assert.deepEqual(process.argv.slice(2),['--check'],'schema command source views are check-only');
const file=SCHEMA_COMMAND_SOURCE_PROFILE.files.find(file=>resolve(process.argv[1]??'')===fileURLToPath(new URL(file,root)));
assert.ok(file,'an exact registered immutable schema command entry is required');
const url=new URL(file,root),result=renderSchemaCommandSourceView(file,readFileSync(url,'utf8'));
console.error('Schema source freshness: authenticated current string-order model with original command '+file+'; executed source sha256 '+result.receipt.executed.sha256);
registerHooks({load(selected,context,nextLoad){
  if(selected===url.href)return{format:'module',shortCircuit:true,source:result.source};
  return nextLoad(selected,context);
}});
