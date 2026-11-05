// Future complete batch producer; source drafting must not execute this module.
import assert from 'node:assert/strict';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {Grammar} from '../../js/src/grammar.js';
import {createDiscoveredConceptLinks} from '../../js/src/discovered-concept-links.js';
import {createGrammarCompositionBatchLinks,readGrammarCompositionBatchLinks} from '../../js/src/grammar-composition-batch.js';
import {globalConceptSpaceToLinks,createGlobalConceptSpaceParser} from '../../js/src/global-concept-space.js';
import {checkGrammarSelectionMappedTree} from '../../js/src/grammar-runtime.js';
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/grammar-composition-batch.lino',import.meta.url));
const sources=fixture.sources.map(source=>({...source,grammar:new Grammar(source.grammar.start,source.grammar.rules.map(rule=>[rule.name,rule]),source.grammar.sourceFormat,source.grammar.declarations)}));
const original=createDiscoveredConceptLinks(sources),archive=createGrammarCompositionBatchLinks(original.network,original.root,fixture.policies),loaded=readGrammarCompositionBatchLinks(archive.network,archive.root);
const parser=createGlobalConceptSpaceParser(globalConceptSpaceToLinks(loaded.space),fixture.sources[0].id),outcome=parser.parseTree(fixture.input);
assert.equal(outcome.ok,true,JSON.stringify(outcome.rejection));assert.equal(outcome.sourcePlanes.length,fixture.expectedPlanes);
const bundle={tree:outcome.tree,sourcePlanes:outcome.sourcePlanes};checkGrammarSelectionMappedTree(bundle);
process.stdout.write(structuredDataToLino({links:archive.network.toLino(),root:archive.root.asU64(),bundle,space:loaded.space}));
