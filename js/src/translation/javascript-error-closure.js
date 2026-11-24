// Host frontend entry for the explicit supplied Error module profile. Actual
// importer and dependency bodies share ONE portable semantic call program.
// There is no data-only Error record or original-body host callback fallback.
import {checkSourceLibraryErrorClosure,createSourceLibraryErrorDeclarationWitness} from './javascript-error-closure-source.js';
import {createJavaScriptErrorRealm} from './javascript-error-realm.js';
import {beginJavaScriptErrorModule,advanceJavaScriptErrorModule,resumeJavaScriptErrorModule} from './javascript-error-module.js';

export function prepareJavaScriptErrorSourceClosure(archive,authority){
 const actual=checkSourceLibraryErrorClosure(archive),declarationWitness=createSourceLibraryErrorDeclarationWitness(actual);
 const embedding=createJavaScriptErrorRealm(actual.capture,{...authority,declarationWitness});
 return {archive:actual,embedding};
}

/** Only the original entry module's actual public exports are callable here.
 * Dependency exports remain private to resolved source calls, even though
 * their full definitions are retained in the complete portable program. */
export function beginJavaScriptErrorSourceClosure(linked,exportName,arguments_,{construct=false}={}){
 const matches=linked.archive.entries.filter(row=>row.exported===exportName);if(matches.length!==1||matches[0].construct!==construct)throw new TypeError('an Error closure call/new needs one actual public entry owner');
 const entry=matches[0],embedding=linked.embedding;
 return beginJavaScriptErrorModule(embedding.module,embedding.heap(),entry.identity,arguments_,{construct});
}

export function advanceJavaScriptErrorSourceClosure(linked,transition,options){return advanceJavaScriptErrorModule(linked.embedding.module,transition,options);}
export function resumeJavaScriptErrorSourceClosure(linked,transition){return resumeJavaScriptErrorModule(linked.embedding.module,transition,linked.embedding.authority);}
