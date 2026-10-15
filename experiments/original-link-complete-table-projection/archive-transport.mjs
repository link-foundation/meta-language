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
