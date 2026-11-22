/** @typedef {{$:'waiting',value:number,child?:number,marker?:number}|{$:'done',value:number,marker?:number}} State */
/** @param {State} source @returns {State} */
function clear(source) {
 const {child,...rest}=source;
 return {...rest,value:source.value+1};
}
function compute() {
 const waiting=clear({$:'waiting',value:2,child:4,marker:6});
 const done=clear({$:'done',value:10});
 return waiting.$==='waiting' && waiting.value===3 &&
  !Object.hasOwn(waiting,'child') && Object.hasOwn(waiting,'marker') && waiting.marker===6 &&
  done.$==='done' && done.value===11 && !Object.hasOwn(done,'marker');
}
