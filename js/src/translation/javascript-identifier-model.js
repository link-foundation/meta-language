// Target identifier spelling only. Existing EmitState/Scope retain ownership,
// declaration reservation and capture-free binder/reference correspondence.
import {readRuntimeProjectData} from '../project-data-loader.js';
import {unsupported} from './diagnostics.js';
import {renderJavaScriptScopedBindingIdentifier} from './javascript-scoped-binding-emission-rules.js';
let selected;
const refuse=detail=>{throw unsupported('JavaScript identifier model',detail);};

export function readJavaScriptIdentifierModelPolicy(){
  if(!selected){
    const policy=readRuntimeProjectData(new URL('./data/javascript-identifier-models.lino',import.meta.url));
    const source=readRuntimeProjectData(new URL('./data/javascript-scoped-binding-emission.lino',import.meta.url));
    if(policy.schemaVersion!==1||policy.reservedPrefix!=='ml_'||policy.renderer.operation!=='renderJavaScriptScopedBindingIdentifier'
      ||policy.renderer.preserve!==false||policy.renderer.sourcePolicy.file!=='js/src/translation/data/javascript-scoped-binding-emission.lino'
      ||policy.renderer.sourcePolicy.model!==source.model||policy.renderer.sourcePolicy.prefix!==source.prefix)
      refuse('the installed identifier policy must retain its exact shared renderer authority');
    selected=policy;
  }
  return structuredClone(selected);
}

/** No model means the original target spelling. The new selection delegates
 * every actual fresh-name choice and corresponding reference to Scope. */
export function readJavaScriptIdentifierModel(model,ident){
  if(model===undefined)return ident;
  const policy=readJavaScriptIdentifierModelPolicy();
  if(model!==policy.model)refuse('select '+policy.model);
  return name=>{
    const normalized=ident(name);
    return normalized.startsWith(policy.reservedPrefix)?renderJavaScriptScopedBindingIdentifier(normalized,false):normalized;
  };
}
