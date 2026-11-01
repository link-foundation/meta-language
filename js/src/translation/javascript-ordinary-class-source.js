// Explicit successor, leaving the ordinary typed closed-class profile intact.
import {captureJavaScriptProjectDataSource,checkJavaScriptProjectDataSource} from './javascript-project-data-source.js';
export function captureJavaScriptOrdinaryClassModule(source,{identifier,entries}={}){return captureJavaScriptProjectDataSource(source,{identifier,entries,profile:'actual closed source classes 1'});}
export function checkJavaScriptOrdinaryClassModule(capture){if(capture?.model!=='actual closed source classes 1')throw new TypeError('ordinary source classes require their explicit actual CST profile');return checkJavaScriptProjectDataSource(capture);}
