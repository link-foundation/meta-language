// Shape validation is distinct from whole-program binder/region validation.
import {typeError} from './diagnostics.js';
import {checkOwnedProgramContainers} from './owned-program-containers.js';
import {acceptRustDataTypeFieldSignatureContainer} from './rust-data-type-field-signature-rules.js';
import {checkRustSourceSignature} from './rust-source-signatures.js';
import {readRustBorrowedExpressionPositionModelError,readRustBorrowedVectorPositionBudgetError} from './rust-borrowed-vector-rules.js';
import {measureRustSourcePositionMetadata} from './rust-standard-sum-source-metadata.js';
export function checkRustSourceExpressionPosition(facet,actualType,language,resolveType=value=>value,resolveData=()=>null){
 const fail=message=>{throw typeError(message);};
 checkOwnedProgramContainers({sourcePosition:facet},{selectedFields:['sourcePosition'],acceptContainer:acceptRustDataTypeFieldSignatureContainer,fail});
 if(!facet||typeof facet!=='object'||Array.isArray(facet)||Object.keys(facet).length!==2||!Object.hasOwn(facet,'model')||!Object.hasOwn(facet,'position'))fail('a borrowed expression source position has missing or unrepresented fields');
 const model=readRustBorrowedExpressionPositionModelError(facet.model,language);if(model)fail(model);
 try{measureRustSourcePositionMetadata(facet.position);}catch(error){if(!(error instanceof RangeError))throw error;fail(error.message);}
 const bindings=new Set(),pending=[[facet.position,0]];let visits=0;
 while(pending.length){const[position,depth]=pending.pop(),budget=readRustBorrowedVectorPositionBudgetError(++visits,depth);if(budget)fail(budget);if(!Array.isArray(position?.lifetimes))fail('a borrowed expression source position needs its ordered lifetime column');for(const lifetime of position.lifetimes)if(lifetime!==null&&lifetime!=='static'&&lifetime!=='_')bindings.add(lifetime);if(position.arrayElementPosition)pending.push([position.arrayElementPosition,depth+1]);}
 // This temporary binder list permits structural position checking only.
 // The whole-program proof independently checks every name against the actual
 // enclosing function binders and follows actual lexical binder graph links.
 const signature=checkRustSourceSignature({model:'Rust source function signature 3',language,parameters:[],result:facet.position,lifetimeBindings:[...bindings]},[],actualType,language,resolveType,resolveData);
 return {model:facet.model,position:signature.result};
}
