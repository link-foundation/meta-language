// Bounded public source-plane consumers. The original JSON consumer retains
// its input and evidence; the operation discriminator narrows each mapping.
export function publicGrammarSourcePlaneConsumers(moduleSpecifier) {
  const module = JSON.stringify(moduleSpecifier);
  return [
    {
      name: 'valid-mapped-grammar-source-planes', expectedDiagnostics: [],
      source: `import {parseGrammarLinks,percentEncodeLinksText,compileGrammar,
  checkGrammarSelectionMappedTree,type GrammarSourcePlane,type GrammarSourceMapping,
  type GrammarIncludedRangeSourceMapping,type GrammarMappedTreeBundle} from ${module};
const raw = JSON.stringify('π');
const child = parseGrammarLinks('(grammar (start decoded))\\n(rule decoded normal (literal '
  + percentEncodeLinksText('π') + '))\\n');
const host = parseGrammarLinks('(grammar (start document))\\n(rule document normal (any-grammar (literal '
  + percentEncodeLinksText(raw) + ') (grammar-candidate Child json-string)))\\n');
const parser = compileGrammar(host,{resolveGrammar:name=>name==='Child'?child:undefined});
const outcome = parser.parseTree(raw);
if(!outcome.ok || !outcome.tree || !outcome.sourcePlanes) throw new Error('mapped result is absent');
const unknownBundle:unknown = {tree:outcome.tree,sourcePlanes:outcome.sourcePlanes};
const checked:GrammarMappedTreeBundle = checkGrammarSelectionMappedTree(unknownBundle);
const sources:GrammarSourcePlane[] = checked.sourcePlanes;
if(checked.tree.type!=='node' || checked.tree.children[0]?.type!=='embed') throw new Error('embed is absent');
const embedded = checked.tree.children[0];
const mapping:GrammarSourceMapping|GrammarIncludedRangeSourceMapping|undefined = embedded.selection?.sourceMapping;
if(!mapping || !embedded.sourceChildren) throw new Error('mapped source roles are absent');
if(mapping.operation!=='json-string') throw new Error('JSON mapping is absent');
const jsonMapping:GrammarSourceMapping = mapping;
export const contractEvidence = {sourceCount:sources.length,physicalChildren:embedded.sourceChildren.length,
  input:mapping.input,output:mapping.output,firstInterval:mapping.intervals[0],rootEncoding:sources[0].encoding};
`,
    },
    {
      name: 'valid-included-range-grammar-source-planes', expectedDiagnostics: [],
      source: `import {parseGrammarLinks,percentEncodeLinksText,compileGrammar,
  checkGrammarSelectionMappedTree,type GrammarSourcePlane,type GrammarSourceMapping,
  type GrammarIncludedRangeSourceMapping,type GrammarMappedTreeBundle} from ${module};
const raw = '*é\\n> β*';
const child = parseGrammarLinks('(grammar (start decoded))\\n(rule decoded normal (literal '
  + percentEncodeLinksText('*é\\nβ*') + '))\\n');
const host = parseGrammarLinks('(grammar (start document))\\n(rule document normal (any-grammar (literal '
  + percentEncodeLinksText(raw) + ') (grammar-range-candidate Child included-ranges (source-range 0 4) (source-range 6 9))))\\n');
const outcome = compileGrammar(host,{resolveGrammar:name=>name==='Child'?child:undefined}).parseTree(raw);
if(!outcome.ok || !outcome.tree || !outcome.sourcePlanes) throw new Error('included range result is absent');
const checked:GrammarMappedTreeBundle = checkGrammarSelectionMappedTree({tree:outcome.tree,sourcePlanes:outcome.sourcePlanes});
const sources:GrammarSourcePlane[] = checked.sourcePlanes;
if(checked.tree.type!=='node' || checked.tree.children[0]?.type!=='embed') throw new Error('included range embed is absent');
const embedded = checked.tree.children[0];
const mapping:GrammarSourceMapping|GrammarIncludedRangeSourceMapping|undefined = embedded.selection?.sourceMapping;
if(!mapping || mapping.operation!=='included-ranges' || !embedded.sourceChildren) throw new Error('included range source roles are absent');
const ranged:GrammarIncludedRangeSourceMapping = mapping;
const derivation = sources[1].derivation;
if(!derivation || derivation.operation!=='included-ranges') throw new Error('included range derivation is absent');
const ranges:Array<[number,number]> = ranged.ranges;
const originRanges:Array<[number,number]> = derivation.ranges;
export const contractEvidence = {sourceCount:sources.length,physicalChildren:embedded.sourceChildren.length,
  input:ranged.input,output:ranged.output,firstInterval:ranged.intervals[0],firstRange:ranges[0],
  originRange:originRanges[0],operation:ranged.operation,byteLength:sources[1].byteLength};
`,
    },
    {
      name: 'reject-included-range-mapping-as-json-only', expectedDiagnostics: ['TS2322'],
      source: `import type {GrammarSourceMapping,GrammarIncludedRangeSourceMapping} from ${module};
const ranged:GrammarIncludedRangeSourceMapping = {kind:'source-mapping',operation:'included-ranges',
  input:0,output:1,ranges:[[0,4],[6,9]],intervals:[[0,0,0,0],[0,1,0,1],[1,3,1,3],[3,4,3,4],
    [4,6,4,4],[6,8,4,6],[8,9,6,7],[9,9,7,7]]};
const jsonOnly:GrammarSourceMapping = ranged;
export const contractEvidence = {mapping:jsonOnly};
`,
    },
  ];
}
