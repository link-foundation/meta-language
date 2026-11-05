// Future CI producer; source authors must never execute this file.
import {createHash} from 'node:crypto';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {inferRegularGrammar} from '../../js/src/grammar-inference.js';
import {refineRegularGrammarInference} from '../../js/src/grammar-inference-refinement.js';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
const [major,minor]=process.versions.node.split('.').map(Number);if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/regular-grammar-refinement.lino',import.meta.url));
const argument=process.argv[2];if(process.argv.length!==3||!/^(0|[1-9][0-9]*)$/u.test(argument??''))throw new TypeError('provide an actual fixture index');
const row=fixture.cases[Number(argument)];if(!row)throw new RangeError('unknown fixture index');
let model=inferRegularGrammar(row.positives,row.negatives,row.options);for(const step of row.steps)model=refineRegularGrammarInference(model.network,model.root,step.positives,step.negatives);
process.stdout.write(structuredDataToLino({name:row.name,identifier:model.source.id,alphabet:model.alphabet,table:model.table,grammar:model.grammar.normalized(),root:model.root.asU64(),archiveSha256:createHash('sha256').update(model.network.toLino()).digest('hex')}));
