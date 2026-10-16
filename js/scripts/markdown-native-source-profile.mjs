#!/usr/bin/env node
// A declared source closure and scanner/composition design, not executable
// Markdown support. CI captures the actual archive members before native
// source registration; no unknown inner-file checksum is invented here.
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {resolve} from 'node:path';
import {readProjectData,writeProjectData,checkProjectData} from './project-data.mjs';

const root=new URL('../../',import.meta.url);
export const MARKDOWN_SOURCE_PROFILE='parity/grammars/markdown-source-profile.lino';

/** Preserve both actual pinned components and their unimplemented obligations. */
export function readMarkdownNativeSourceProfile(){
  const lock=readFileSync(new URL('rust/Cargo.lock',root),'utf8');
  const block=lock.split('[[package]]').find(value=>/^name = "tree-sitter-md-025"$/mu.test(value));
  const version=/^version = "([^"]+)"$/mu.exec(block??'')?.[1];
  const checksum=/^checksum = "([0-9a-f]{64})"$/mu.exec(block??'')?.[1];
  if(version!=='0.5.6'||checksum!=='b2d54d63bc7fdf0c1c38237611d966548fbccf8f0923a7de5d525b87c9957a96')throw new Error('Markdown source closure requires review after the Cargo pin changes');
  const grammars=readProjectData(new URL('js/src/vendor/grammars/grammar-lock.lino',root)).grammars;
  const components=['markdown','markdown_inline'].map((id,position)=>{
    const entry=grammars[id];
    if(entry?.crate!=='tree-sitter-md-025'||entry.version!==version||!/^[0-9a-f]{64}$/u.test(entry.parserSha256)||!/^[0-9a-f]{64}$/u.test(entry.wasmSha256))throw new Error(`Markdown component pin mismatch: ${id}`);
    return{id,position,role:position===0?'block-host':'inline-region',directory:entry.directory,
      grammarMember:`${entry.directory}/src/grammar.json`,scannerMember:`${entry.directory}/src/scanner.c`,
      corpusDirectory:`${entry.directory}/test/corpus/`,parserSha256:entry.parserSha256,wasmSha256:entry.wasmSha256,
      licenseFile:`js/src/vendor/grammars/${entry.license}`};
  });
  return{
    schemaVersion:1,kind:'source-closure-and-composition-design',behavioralEvidence:false,
    archive:{crate:'tree-sitter-md-025',version,sha256:checksum,memberPrefix:`tree-sitter-md-025-${version}/`,
      url:`https://static.crates.io/crates/tree-sitter-md-025/tree-sitter-md-025-${version}.crate`,license:'MIT',repository:'https://github.com/subsriram/tree-sitter-markdown'},
    components,
    scannerFamilies:[
      {family:'container-layout',sourceTokens:['_indented_chunk_start','_block_close','block_continuation','_close_block','_no_indented_chunk','_blank_line_start','_block_quote_start','_line_ending','_soft_line_ending','_error','_trigger_error','_eof'],
        sourceAudit:['ordered container state','lazy continuation','tab-expanded indentation','queued zero-width closure','line-ending extents','valid-symbol and error-sentinel guards']},
      {family:'contextual-line-marker',sourceTokens:[...Array.from({length:6},(_,index)=>`atx_h${index+1}_marker`),'setext_h1_underline','setext_h2_underline','_thematic_break',...['plus','minus','star','dot','parenthesis'].flatMap(marker=>[`_list_marker_${marker}`,`_list_marker_${marker}_dont_interrupt`])],
        sourceAudit:['heading and thematic priority','marker widths and exact ends','container-relative indentation','ordered marker bounds','paragraph interruption']},
      {family:'split-line-delimiter',sourceTokens:['_fenced_code_block_start_backtick','_fenced_code_block_end_backtick','_fenced_code_block_start_tilde','_fenced_code_block_end_tilde'],
        content:'retained imported grammar production; no whole-block replacement token',sourceAudit:['remembered marker and width','opening info restrictions','closing relation','container indentation','end-of-input closure']},
      {family:'contextual-block-boundary',sourceTokens:['minus_metadata','plus_metadata','_html_block_1_start','_html_block_1_end',...Array.from({length:6},(_,index)=>`_html_block_${index+2}_start`)],
        sourceAudit:['metadata document-start restriction','HTML tag sets and case matching','asymmetric source token interface','blank-line and container termination']},
      {family:'table-boundary',sourceTokens:['_pipe_table_start','_pipe_table_line_ending'],sourceAudit:['header and delimiter lookahead','escaped pipes','cell counts','marked ends versus inspected lines']},
    ],
    scannerGeneration:{registry:'js/scripts/scanner-families.mjs',profileGenerator:'js/scripts/generate-native-scanner-profiles.mjs',
      mechanism:'existing shared Links operations; profile descriptors require audited source transitions',
      primitives:['integer and text variables','ordered stacks','valid requested token','expected source-qualified token','mark versus lookahead cursor','rollback on failed speculation'],
      columnPolicy:'existing column counts code points; tab stops require source-audited explicit counters'},
    composition:{host:'markdown',target:'markdown_inline',sourceQualifiedHostKinds:['inline','pipe_table_cell'],
      ranges:{base:'host-node',exclude:{children:'direct',firstRawChildIndex:1,named:true}},
      projection:{retainHostContainer:true,retainComponentRootsAsLinkedProvenance:true,inlineChildren:'root-children',
        excludedNodes:'deepest-containing-child',uncontainedNodes:'siblings',equalStarts:'host-first-stable-source-order',emptyNodeContainment:'strict-interior'},
      spanUnit:'absolute-utf8-bytes',resourceBudgets:'shared and unchanged',sourceOwnership:'each source byte rendered exactly once',
      unknownComponents:'diagnostic',nonconsumingCycles:'diagnostic',globalConceptSpace:'shared; source-qualified meanings remain distinct without checked correspondence',
      implementationGaps:['native discontiguous range view','excluded-node reinsertion','composition-aware native tree projection','source-free linked component and policy archive']},
    captureRequirements:['authenticate complete crate archive before reading members','record actual grammar and scanner bytes and hashes','record scanner include closure and corpus member metadata','store receipts as Links data only','keep upstream files temporary and ignored','independent block and inline oracle cases before default promotion'],
  };
}

/** Execute the declared included-range policy over source-qualified CST views. */
export function readMarkdownInlineRanges(policy,host){
  if(policy.ranges?.base!=='host-node'||policy.ranges.exclude?.children!=='direct'
    ||policy.ranges.exclude.named!==true||!Number.isSafeInteger(policy.ranges.exclude.firstRawChildIndex)
    ||policy.ranges.exclude.firstRawChildIndex<0||policy.spanUnit!=='absolute-utf8-bytes')throw new Error('Unsupported Markdown composition range policy');
  if(host.component!==policy.host)throw new Error('Unknown Markdown host component');
  if(!policy.sourceQualifiedHostKinds.includes(host.sourceKind))return null;
  if(!Number.isSafeInteger(host.start)||!Number.isSafeInteger(host.end)||host.start<0||host.end<host.start||!Array.isArray(host.children))throw new Error('Invalid Markdown host CST range');
  for(const child of host.children)if(!Number.isSafeInteger(child.start)||!Number.isSafeInteger(child.end)
    ||child.start<host.start||child.end>host.end||child.end<child.start||typeof child.named!=='boolean')throw new Error('Invalid Markdown child CST range');
  const excluded=host.children.slice(policy.ranges.exclude.firstRawChildIndex).filter(child=>child.named);
  const included=[];let start=host.start;
  for(const child of excluded){
    if(child.start<start)throw new Error('Overlapping Markdown excluded CST ranges');
    included.push({start,end:child.start});start=child.end;
  }
  included.push({start,end:host.end});
  return{hostComponent:host.component,targetComponent:policy.target,sourceKind:host.sourceKind,
    spanUnit:policy.spanUnit,start:host.start,end:host.end,includedRanges:included,excludedChildren:excluded};
}

/** Execute the existing stable reinsertion policy over inspectable CST views. */
export function projectMarkdownInlineChildren(policy,children,excluded){
  const projection=policy.projection;
  if(policy.spanUnit!=='absolute-utf8-bytes'||projection.excludedNodes!=='deepest-containing-child'||projection.uncontainedNodes!=='siblings'
    ||projection.equalStarts!=='host-first-stable-source-order'||projection.emptyNodeContainment!=='strict-interior'
    ||projection.inlineChildren!=='root-children'||projection.retainHostContainer!==true
    ||projection.retainComponentRootsAsLinkedProvenance!==true)throw new Error('Unsupported Markdown composition projection policy');
  const contains=(outer,inner)=>outer.start<=inner.start&&inner.end<=outer.end
    &&(inner.start<inner.end||(outer.start<inner.start&&inner.start<outer.end));
  const distribute=(nodes,injected,ancestors)=>{
    const entries=nodes.map(node=>({node,injected:[]})),top=[];
    for(const node of injected){
      const owner=entries.find(entry=>contains(entry.node,node));
      if(owner)owner.injected.push(node);else top.push({node,injected:[]});
    }
    const ordered=top.length?[...top,...entries].sort((left,right)=>left.node.start-right.node.start):entries;
    return ordered.map(({node,injected:nested})=>{
      if(ancestors.has(node))throw new Error('Cyclic Markdown CST ownership');
      if(nested.length>0&&['token','error','missing'].includes(node.type))throw new Error('Markdown inline leaf spans excluded source; a native range view is required');
      if(!Array.isArray(node.children)&&nested.length===0)return node;
      const next=new Set(ancestors);next.add(node);
      return{...node,children:distribute(node.children??[],nested,next)};
    });
  };
  return distribute(children,excluded,new Set());
}

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
  const target=new URL(MARKDOWN_SOURCE_PROFILE,root),profile=readMarkdownNativeSourceProfile();
  if(process.argv.includes('--check'))checkProjectData(target,profile);
  else writeProjectData(target,profile);
}
