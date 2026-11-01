// Deferred extension of existing original/compiler/native-machine controls.
// Importing this module constructs only finite authored input records.
import {encodeJavaScriptCallSource} from './call-source-codecs.mjs';
import {encodeJavaScriptSemanticValue} from './call-machine-codecs.mjs';
import {observeCallSourceException,observeCallSourceExceptionRefusal} from './call-source-exception-controls.mjs';
import {javascriptSourceControlCases,javascriptSourceControlRefusals} from '../../js/tests/fixtures/javascript-source-controls.mjs';
export function observeCallSourceControl(row){
 return observeCallSourceException({...row,expression:encodeJavaScriptCallSource(row.expression),receiver:{$:'undefined'},expected:{kind:row.expected.kind,value:encodeJavaScriptSemanticValue(row.expected.value)}});
}
export function observeAllCallSourceControls(){return javascriptSourceControlCases.map(observeCallSourceControl);}

export function observeAllCallSourceControlRefusals(){return javascriptSourceControlRefusals.map(row=>observeCallSourceExceptionRefusal({...row,parameters:[],expression:encodeJavaScriptCallSource(row.expression)}));}
