// Source/schema construction only. No original or current code tests run.
import {readFileSync,writeFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {writeProjectData} from './project-data.mjs';
import {extendOrderedRecordSchema,extendOrderedRecordOwnedSource} from './ordered-record-schema-profile.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),revision='cdf083b0';
const capture=file=>{
 const bytes=execFileSync('git',['show',revision+':'+file],{maxBuffer:4*1024*1024});
 return{file,revision,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')};
};
const previousSchema=capture('js/src/data/typed-program-links-schema.lino'),previousNativeTypes=capture('rust/src/translation/types.rs');
const ownedSources=['js/scripts/read-program-type-neutral-defaults.mjs','experiments/original-link-complete-table-projection/raw-schema-sixth-extension-closure.mjs'].map(capture);
writeProjectData(new URL('parity/self-translation/ordered-record-schema-predecessor.lino',root),{
 version:1,model:'bounded ordered record schema and native type source successor 1',previousSchema,previousNativeTypes,ownedSources,
 scope:'Complete preserved version-one source/schema inputs and exact known source bridges; version two derives all six active native Type facets. No execution evidence or semantic equivalence is asserted.',
});
for(const entry of ownedSources)writeFileSync(new URL(entry.file,root),extendOrderedRecordOwnedSource(entry.file,entry.source));
const after=extendOrderedRecordSchema(readStructuredDataLino(previousSchema.source));
for(const file of ['js/src/data/typed-program-links-schema.lino','rust/src/data/typed-program-links-schema.lino'])writeProjectData(new URL(file,root),after);
const nativeBytes=readFileSync(new URL('rust/src/translation/types.rs',root));
const hash=createHash('sha256').update(nativeBytes).digest('hex');
const policy=new URL('./ordered-record-schema-profile.mjs',import.meta.url),source=readFileSync(policy,'utf8');
const pattern=/const currentNativeTypeSha256='(?:PENDING|[a-f0-9]{64})';/u;
if(source.match(new RegExp(pattern.source,'gu'))?.length!==1)throw new TypeError('one exact active native Type source version is required');
writeFileSync(policy,source.replace(pattern,"const currentNativeTypeSha256='"+hash+"';"));
