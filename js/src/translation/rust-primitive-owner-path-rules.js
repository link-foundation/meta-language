// Shared primitive path domain; method registration is a separate source input.
/** Resolve only actual primitive paths; lexical resolution stays in the host.
 * @param {string[]} segments @param {boolean} ordinary
 * @param {boolean} shadowed @param {boolean} absolute @returns {string} */
export function readRustPrimitiveOwnerPath(segments,ordinary,shadowed,absolute){
 if(!ordinary||shadowed)return '';
 let owner='';
 if(segments.length===2&&!absolute)owner=segments[0]??'';
 else if(segments.length===4&&(segments[0]==='std'||segments[0]==='core')&&segments[1]==='primitive')owner=segments[2]??'';
 return ['bool','i8','u8','i16','u16','i32','u32','i64','u64','i128','u128','isize','usize'].includes(owner)?owner:'';
}
