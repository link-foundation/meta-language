// Inspect actual generated data fields; historical two-field pairs stay valid.
import {readSourceExceptionIgnoredPatternFields,acceptSourceExceptionPayloadFields} from './source-exception-rules.js';
export function hasSourceExceptionPayload(entry){
 const fields=entry?.ctors?.[1]?.fields??[];
 return acceptSourceExceptionPayloadFields(entry?.output===true,fields.map(field=>field.name),fields.map(field=>field.type.kind));
}
export function hasSourceExceptionOutput(program){return program.sourceLanguage==='JavaScript'&&[...program.declarations.values()].some(hasSourceExceptionPayload)
 &&(program.main?.effects??[]).every(effect=>effect.k!=='unwrap'||hasSourceExceptionPayload(program.declarations.get(effect.data)));}
export function sourceExceptionPatternSuffix(program,name){return readSourceExceptionIgnoredPatternFields(program.declarations.get(name)?.ctors?.[1]?.fields?.length??0);}
