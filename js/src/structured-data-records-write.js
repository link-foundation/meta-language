// One scoped typed data frame, writable before third-party dependencies exist.
import { randomUUID } from 'node:crypto';
import { encodeLinkStatement } from './lino-serialization-write.js';
import { encodeStructuredDataLinks } from './structured-data-encode.js';
import { namespacePattern, failure } from './structured-data-record-model.js';
const encoder = new TextEncoder();

/** Encode one bounded, uniquely scoped record for atomic append. */
export function structuredDataRecordToLino(value, namespace = randomUUID(), limits = {}) {
  if (!namespacePattern.test(namespace)) failure('invalid namespace');
  const { network, root } = encodeStructuredDataLinks(value, limits);
  const statements = network.links().map(link => {
    const canonical = encodeLinkStatement(link.id(), link.references(), link.metadata());
    const metadata = canonical.slice(canonical.indexOf(' (meta:'));
    return `(${namespace}/${link.id()}:${link.references().map(reference => ` ${namespace}/${reference}`).join('')}${metadata}`;
  });
  const text = `(${namespace}/record: ${namespace}/${root} ${statements.join(' ')}`
    + ' (meta: (t: semantic) (n: 1) (term: structured%20data%20record) (def: 1) (lang: meta-language%20structured%20data)))\n';
  if (encoder.encode(text).length > (limits.byteLimit ?? 128 * 1024 * 1024)) failure('serialized record byte resource limit exceeded');
  return text;
}

