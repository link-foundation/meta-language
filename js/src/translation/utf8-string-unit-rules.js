// Shared exact UTF-8 transport: invalid input is the column [-1]; all
// actual UTF-16 units are nonnegative, so empty text remains unambiguous.
/** UTF-8 transport for targets whose string value is a byte sequence.
 * @param {number[]} bytes @returns {number[]} */
export function readJavaScriptUtf8StringUnits(bytes) {
  for(let index=0;index<bytes.length;index+=1){
    const unit=bytes[index]?? -1;
    if(!Number.isSafeInteger(unit)||unit<0||unit>255)return [-1];
  }
  const units=[];
  let index=0;
  while(index<bytes.length){
    let scalar=bytes[index]?? -1;
    index+=1;
    let remaining=0,minimum=0;
    if(scalar>=194&&scalar<=223){scalar-=192;remaining=1;minimum=128;}
    else if(scalar>=224&&scalar<=239){scalar-=224;remaining=2;minimum=2048;}
    else if(scalar>=240&&scalar<=244){scalar-=240;remaining=3;minimum=65536;}
    else if(scalar>=128)return [-1];
    while(remaining>0){
      const next=bytes[index]?? -1;
      if(next<128||next>191)return [-1];
      scalar=scalar*64+next-128;index+=1;remaining-=1;
    }
    if(scalar<minimum||scalar>1114111||scalar>=55296&&scalar<=57343)return [-1];
    if(scalar<=65535)units.push(scalar);
    else {units.push(55296+Math.floor((scalar-65536)/1024));units.push(56320+(scalar-65536)%1024);}
  }
  return units;
}
