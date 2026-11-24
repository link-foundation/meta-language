/** @typedef {{tail:number,maybe?:number}} Shape */
/** @param {boolean} own @returns {Shape} */
function make(own) { return own ? {maybe:undefined,tail:3} : {tail:3}; }
/** @param {boolean} own @returns {boolean} */
function compute(own) {
 const original=make(own);
 const result={...original,lead:4,'2':2};
 const {lead,...rest}=result;
 const keys=Object.keys(rest), values=Object.values(rest), entries=Object.entries(rest);
 return lead===4 && keys[0]==='2' && values[0]===2 && keys.length===entries.length &&
  (own ? keys.length===3 && keys[1]==='maybe' && keys[2]==='tail' &&
   values[1]===undefined && values[2]===3 && Object.hasOwn(rest,'maybe')
  : keys.length===2 && keys[1]==='tail' && values[1]===3 && !Object.hasOwn(rest,'maybe'));
}
