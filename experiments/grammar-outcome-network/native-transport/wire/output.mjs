import assert from 'node:assert/strict';
import {mkdirSync,realpathSync,existsSync,readdirSync} from 'node:fs';
import {resolve,relative,sep,dirname,basename} from 'node:path';
import {readProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
import {repository} from './wire-policy.mjs';

/** Refuse implicit output and production paths before preparing any proposal. */
export function explicitOutputDirectory(args) {
  assert.equal(args.length,2,'supply --output-dir and one task-owned scratch directory');
  assert.equal(args[0],'--output-dir');
  const directory=resolve(args[1]),root=realpathSync(repository);
  const permitted=path=>path==='..'||path.startsWith('..'+sep)||path.startsWith('.issue-195-work'+sep);
  assert.ok(permitted(relative(root,directory)),
    'output must be outside the checkout or inside its ignored .issue-195-work directory');
  let ancestor=directory;const tail=[];
  while(!existsSync(ancestor)){tail.unshift(basename(ancestor));ancestor=dirname(ancestor);}
  const actual=resolve(realpathSync(ancestor),...tail);
  assert.ok(permitted(relative(root,actual)),'resolved output must remain outside production sources');
  const marker=resolve(actual,'.meta-language-native-outcome-output.lino');
  const owner={schemaVersion:1,repository:root,purpose:'native grammar outcome source proposals'};
  if(existsSync(actual)&&readdirSync(actual).length>0){
    assert.ok(existsSync(marker),'existing output must be empty or have the exact task ownership marker');
    assert.deepEqual(readProjectData(marker),owner,'output belongs to this source proposal task');
  }
  mkdirSync(actual,{recursive:true});
  if(!existsSync(marker))writeProjectData(marker,owner);
  return actual;
}
