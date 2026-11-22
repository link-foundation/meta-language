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
/** @typedef {{head:number,count?:number,power?:number,increment?:number,label?:string}} CompoundShape */
/** @returns {CompoundShape} */
function makeCompound() { return {head:0}; }
/** @param {CompoundShape} record @returns {number} */
function writeCount(record) { record.count = 20; return 5; }
/** @param {CompoundShape} record @returns {string} */
function writeLabel(record) { record.label = 'replaced'; return '!'; }
/** @returns {boolean} */
function compound() {
  const record = makeCompound(), alias = record;
  record.count += writeCount(alias);
  alias.power = undefined;
  record.power **= 0;
  alias.increment++;
  record.label = undefined;
  alias.label += writeLabel(record);
  const keys = Object.keys(record);
  return Number.isNaN(alias.count ?? 0) && (alias.power ?? 0)===1 &&
    Number.isNaN(record.increment ?? 0) && (record.label ?? '')==='undefined!' &&
    keys.length===5 && keys[0]==='head' && keys[1]==='count' && keys[2]==='power' &&
    keys[3]==='increment' && keys[4]==='label';
}
