// CI-authoring transport for complete canonical archives. The standard parser
// keeps its existing per-record input/buffer bounds; no records are filtered.
import {createRequire} from 'node:module';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {LinkNetwork} from '../../js/src/network.js';
import {Executor} from '../../js/src/grammar-runtime/executor.js';
import {decodeCanonicalStatements, LinoSerializationError} from '../../js/src/lino-serialization.js';
const {StreamParser} = await import(pathToFileURL(createRequire(new URL('../../js/package.json', import.meta.url)).resolve('links-notation')).href);

/** Actual CI transport source provenance, separate from old producer bindings. */
export function readCanonicalArchiveTransportSource() {
  const bytes = readFileSync(new URL(import.meta.url));
  return Object.freeze({file: 'experiments/original-link-complete-table-projection/archive-transport.mjs',
    sha256: createHash('sha256').update(bytes).digest('hex'), bytes: bytes.length});
}

/** Valid generated sources need their CST, including a successful parse. */
export function requireSuccessfulSourceArchive(network) {
  for (const link of network.links()) {
    if (link.metadata().flags.hasRecoveryIssue()) throw new TypeError('complete generated source archive contains a recovered or failed parse');
  }
  return network;
}

/** Reload every canonical record while retaining only one parser record. */
export function reloadCanonicalArchive(source, {createParser = () => new StreamParser({comments: false, collect: false})} = {}) {
  if (typeof source !== 'string') throw new TypeError('the complete canonical archive must be text');
  const network = new LinkNetwork(), parser = createParser();
  parser.on('error', error => {throw new LinoSerializationError('parse', error.message);});
  parser.on('link', statement => {
    for (const {id, references, metadata, registered} of decodeCanonicalStatements([statement])) {
      const identity = network.insertLinkWithOptionalId(id, references, metadata);
      if (registered && metadata.term !== undefined) network._terms.set(metadata.term, identity);
    }
  });
  for (let offset = 0; offset < source.length; offset += 64 * 1024) parser.write(source.slice(offset, offset + 64 * 1024));
  parser.end();
  return network;
}

/** Only encoded archives escape this scope; the original CST is not retained. */
export function createCompleteRustSourceArchives(source, {parseSource, encodeBinary, onPhase = () => {}, onNativeRun, nativeRunCellInterval = 65536}) {
  onPhase('parse complete Rust source');
  const network = observeCompleteSourceParse(source, parseSource, onNativeRun, nativeRunCellInterval);
  const recordLimit = Math.max(1, network.len());
  onPhase('encode complete binary archive', recordLimit);
  // This authenticated, already materialized graph selects an exact record
  // bound through the codec's existing public option. Its byte cap is unchanged.
  const binary = encodeBinary(network, {recordLimit});
  onPhase('encode complete text archive', recordLimit);
  const text = network.toLino();
  return Object.freeze({text, binary, recordLimit});
}

// CI-only observation wraps the actual default executor without replacing its
// result or limits. Only scalar diagnostics leave the invocation.
function observeCompleteSourceParse(source, parseSource, onNativeRun, nativeRunCellInterval) {
  if (onNativeRun === undefined) return parseSource(source);
  if (!Number.isSafeInteger(nativeRunCellInterval) || nativeRunCellInterval < 1) throw new TypeError('native source diagnostics need a positive integer cell interval');
  const original = Executor.prototype.run;
  const originalRetain = Executor.prototype.retain;
  const contexts = [];
  let run = 0;
  Executor.prototype.retain = function(...args) {
    const result = originalRetain.apply(this,args);
    const current = contexts.at(-1);
    if (current?.executor === this && this.budget.memory.cells >= current.next) {
      current.next = this.budget.memory.cells + nativeRunCellInterval;
      onNativeRun(current.event('progress'));
    }
    return result;
  };
  Executor.prototype.run = function(...args) {
    const sequence = ++run;
    const event = phase => ({phase, sequence, inputBytes:this.end-this.begin,
      memoCells:this.budget.memory.cells, storedRules:this.memo.size,
      heap:process.memoryUsage().heapUsed});
    contexts.push({executor:this,next:this.budget.memory.cells+nativeRunCellInterval,event});
    try {
      onNativeRun(event('begin'));
      const result = original.apply(this,args);
      onNativeRun({...event('end'),accepted:result.ok});
      return result;
    } finally {contexts.pop();}
  };
  try {return parseSource(source);}
  finally {Executor.prototype.run = original;Executor.prototype.retain = originalRetain;}
}

/** Each reconstructed graph expires before the next complete replay begins. */
export function replayCompleteRustSourceArchives(archives, {decodeBinary, onPhase = () => {}}) {
  onPhase('reload complete binary archive', archives.recordLimit);
  const binaryReplay = (() => decodeBinary(archives.binary, {recordLimit: archives.recordLimit}).reconstructText())();
  onPhase('reload complete text archive', archives.recordLimit);
  const textReplay = (() => reloadCanonicalArchive(archives.text).reconstructText())();
  onPhase('complete source replay finished', archives.recordLimit);
  return {binaryReplay, textReplay};
}
