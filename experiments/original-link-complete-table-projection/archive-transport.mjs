// CI-authoring transport for complete canonical archives. The standard parser
// keeps its existing per-record input/buffer bounds; no records are filtered.
import {createRequire} from 'node:module';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {LinkNetwork} from '../../js/src/network.js';
import {decodeCanonicalStatements, LinoSerializationError} from '../../js/src/lino-serialization.js';
const {StreamParser} = await import(pathToFileURL(createRequire(new URL('../../js/package.json', import.meta.url)).resolve('links-notation')).href);

/** Actual CI transport source provenance, separate from old producer bindings. */
export function readCanonicalArchiveTransportSource() {
  const bytes = readFileSync(new URL(import.meta.url));
  return Object.freeze({file: 'experiments/original-link-complete-table-projection/archive-transport.mjs',
    sha256: createHash('sha256').update(bytes).digest('hex'), bytes: bytes.length});
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
export function createCompleteRustSourceArchives(source, {parseSource, encodeBinary, onPhase = () => {}}) {
  onPhase('parse complete Rust source');
  const network = parseSource(source);
  const recordLimit = Math.max(1, network.len());
  onPhase('encode complete binary archive', recordLimit);
  // This authenticated, already materialized graph selects an exact record
  // bound through the codec's existing public option. Its byte cap is unchanged.
  const binary = encodeBinary(network, {recordLimit});
  onPhase('encode complete text archive', recordLimit);
  const text = network.toLino();
  return Object.freeze({text, binary, recordLimit});
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
