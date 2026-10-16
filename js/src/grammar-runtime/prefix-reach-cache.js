// Preserve every logical Map entry while storing canonical numeric positions
// in sparse pages. Unknown keys and unrepresentable values keep their original
// Map representation, including undefined, signed zero and object identity.
const PAGE_WIDTH=64;
const SPARSE_ENTRY_LIMIT=8;

export class PrefixReachCache {
  constructor(maximumPosition) {
    this.states=new Map();
    this.other=new Map();
    this.size=0;
    this.wide=maximumPosition>0xfffffffe;
    this.recentKey=null;
    this.recentLocation=null;
  }

  locate(key,create=false) {
    if(typeof key!=='string')return null;
    if(key===this.recentKey&&(this.recentLocation!==null||!create))return this.recentLocation;
    const split=key.indexOf('|');
    if(split<1)return null;
    const text=key.slice(0,split),position=Number(text);
    if(!Number.isSafeInteger(position)||position<0||String(position)!==text)return null;
    const state=key.slice(split+1),block=Math.floor(position/PAGE_WIDTH),offset=position%PAGE_WIDTH;
    let pages=this.states.get(state);
    if(!pages&&create)this.states.set(state,pages=new Map());
    let page=pages?.get(block);
    if(!page&&create)pages.set(block,page=new Map());
    const found=page?{pages,page,block,offset}:null;
    this.recentKey=key;this.recentLocation=found;
    return found;
  }

  has(key) {
    if(this.other.has(key))return true;
    const found=this.locate(key);
    return found!==null&&(found.page instanceof Map?found.page.has(found.offset):found.page[found.offset]!==0);
  }

  get(key) {
    if(this.other.has(key))return this.other.get(key);
    const found=this.locate(key);
    if(found===null)return undefined;
    if(found.page instanceof Map)return found.page.get(found.offset);
    const value=found.page[found.offset];
    return value===0?undefined:value-1;
  }

  set(key,value) {
    const existed=this.has(key);
    const representable=typeof value==='number'&&Number.isSafeInteger(value)&&value>=0
      &&!Object.is(value,-0)&&(this.wide||value<=0xfffffffe);
    const found=this.locate(key,representable);
    if(!representable||found===null){
      if(found){if(found.page instanceof Map)found.page.delete(found.offset);else found.page[found.offset]=0;}
      this.other.set(key,value);
    }else{
      this.other.delete(key);
      if(found.page instanceof Map){
        found.page.set(found.offset,value);
        if(found.page.size>SPARSE_ENTRY_LIMIT){
          const dense=this.wide?new Float64Array(PAGE_WIDTH):new Uint32Array(PAGE_WIDTH);
          for(const[offset,stored]of found.page)dense[offset]=stored+1;
          found.pages.set(found.block,dense);
          found.page=dense;
        }
      }else found.page[found.offset]=value+1;
    }
    if(!existed)this.size++;
    return this;
  }
}
