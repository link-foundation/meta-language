/** @typedef {{head:number,'\x35':number,'2'?:number,late?:number,'l,a:te}'?:number}} Shape */
/** @returns {Shape} */
function make() { return {head:3,'5':5}; }
/** @returns {boolean} */
function compute() {
  const record = make(), alias = record;
  const absent = !Object.hasOwn(alias,'late') && !Object.hasOwn(alias,'2') &&
    !Object.hasOwn(alias,'l,a:te}');
  record.late = undefined;
  record['2'] = 2;
  alias.head = 4;
  alias.late = 7;
  record.late = undefined;
  const keys = Object.keys(alias), values = Object.values(record);
  return absent && Object.hasOwn(record,'late') && Object.hasOwn(alias,'2') &&
    keys.length===4 && keys[0]==='2' && keys[1]==='5' && keys[2]==='head' &&
    keys[3]==='late' && values[0]===2 && values[1]===5 && values[2]===4 &&
    values[3]===undefined;
}
