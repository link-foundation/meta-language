// Canonical three-column unsigned decimal source versions; no host parsing.
/** @param {string} text @returns {number[]} */
export function readSourceVersionColumns(text){
 if(text.length>32)return [];
 const parts=text.split('.');if(parts.length!==3)return [];
 const columns=parts.map(part=>{
  if(part.length===0||part.length>1&&part.charCodeAt(0)===48)return -1;
  let value=0;
  for(let index=0;index<part.length;index+=1){const unit=part.charCodeAt(index);if(unit<48||unit>57)return -1;value=value*10+unit-48;if(value>4294967295)return -1;}
  return value;
 });
 return columns.some(value=>value<0)?[]:columns;
}
