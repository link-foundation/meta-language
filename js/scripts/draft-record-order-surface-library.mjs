// Source construction only: complete closure and actual parsed syntax as Links.
import {readSourceLibraryClosure} from './read-source-library-closure.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
const sourceClosure=readSourceLibraryClosure('js/src/translation/javascript-record-order-rules.js');
const value={version:1,model:'complete ordinary record order source library 1',sourceClosure,surface:parseJavaScript(sourceClosure.source)};
for(const directory of ['js','rust'])writeProjectData(new URL(directory+'/src/data/record-order-surface-library.lino',root),value);
