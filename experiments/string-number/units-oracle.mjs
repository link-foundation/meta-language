// Future independent Node and JavaScript-library observations, never an author.
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {convertJavaScriptStringNumber} from '../../js/src/string-number.js';
const [major,minor]=process.versions.node.split('.').map(Number);
if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/string-number.lino',import.meta.url));
const observe=value=>({value:String(value),negativeZero:Object.is(value,-0)});
process.stdout.write(structuredDataToLino({cases:fixture.unitCases.map(row=>{
  const text=String.fromCharCode(...row.units);
  return {...row,original:observe(Number(text)),javascript:observe(convertJavaScriptStringNumber(text))};
})}));
