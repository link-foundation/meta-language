import { open } from 'node:fs/promises';
import { StringDecoder } from 'node:string_decoder';

/** Redact credentials and URL query parameters before printing child logs. */
export function redactEvidenceCommandLog(text, environment = process.env) {
  let redacted = text;
  const secrets = Object.entries(environment).filter(([name, value]) => value &&
    /(?:TOKEN|SECRET|PASSWORD|CREDENTIAL|PRIVATE_KEY|ACCESS_KEY)/iu.test(name)).map(([, value]) => String(value));
  for (const secret of secrets.sort((a, b) => b.length - a.length)) redacted = redacted.replaceAll(secret, '[redacted]');
  return redacted
    .replace(/https?:\/\/[^\s'"<>]+/gu, value => {
      try {
        const url = new URL(value);
        if (url.username || url.password) { url.username = '[redacted]'; url.password = ''; }
        if (url.search) url.search = '?[redacted]';
        if (url.hash) url.hash = '#[redacted]';
        return url.toString();
      } catch { return '[redacted URL]'; }
    })
    .replace(/-----BEGIN [^-]*PRIVATE KEY-----[\s\S]*?-----END [^-]*PRIVATE KEY-----/gu, '[redacted private key]')
    .replace(/\b(?:github_pat_|gh[pousr]_|npm_)[A-Za-z0-9_]+/gu, '[redacted]')
    .replace(/\b(?:Bearer|Basic)\s+[^\s'"<>]+/giu, '[redacted authorization]')
    .replace(/\bauthorization\s*[:=]\s*[^\r\n]+/giu, '[redacted authorization]')
    .replace(/\b(?:token|secret|password|credential|api[_-]?key|access[_-]?key|signature)\s*[:=]\s*[^\s,;]+/giu, '[redacted credential]');
}

/** Forward complete, redacted UTF-8 lines without retaining an unbounded log. */
export function createEvidenceCommandLogForwarder(write, environment = process.env) {
  const redactionEnvironment = { ...environment };
  for (const [name, value] of Object.entries(environment)) {
    if (value && /(?:TOKEN|SECRET|PASSWORD|CREDENTIAL|PRIVATE_KEY|ACCESS_KEY)/iu.test(name)) {
      String(value).split(/[\r\n]+/u).filter(Boolean).forEach((part, index) => {
        redactionEnvironment[`${name}_STREAM_PART_${index}`] = part;
      });
    }
  }
  const decoder = new StringDecoder('utf8'), maximum = 32 * 1024;
  let line = '', bytes = 0, oversized = false, encodedKey = false, sensitiveLine = false;
  let markerTail = '', markerOffset = 0, markerEnd = -1, ended = false;
  const append = part => {
    const markers = markerTail + part, base = markerOffset - markerTail.length;
    // Begin markers are recognized before discarding an oversized line. All
    // encoded key blocks are conservatively masked, including split headers.
    for (const match of markers.matchAll(/-----BEGIN |-----END [^-\r\n]*-----/gu)) {
      const end = base + match.index + match[0].length;
      if (end <= markerEnd) continue;
      markerEnd = end;sensitiveLine = true;encodedKey = match[0].startsWith('-----BEGIN ');
    }
    markerOffset += part.length;markerTail = markers.slice(-maximum);
    bytes += Buffer.byteLength(part);
    if (bytes > maximum) { oversized = true;line = ''; }
    else if (!oversized) line += part;
  };
  const flush = ending => {
    if (sensitiveLine) write('[redacted encoded key block]' + ending);
    else if (oversized) write('[evidence log line omitted: exceeds 32768 bytes; complete raw log retained]' + ending);
    else write(redactEvidenceCommandLog(line + ending, redactionEnvironment));
    line = '';bytes = 0;oversized = false;sensitiveLine = encodedKey;markerTail = '';
  };
  const consume = text => {
    let start = 0, end;
    while ((end = text.indexOf('\n', start)) !== -1) {
      append(text.slice(start, end));flush('\n');markerOffset++;start = end + 1;
    }
    if (start < text.length) append(text.slice(start));
  };
  return {
    write(chunk) {
      if (ended) throw new TypeError('the evidence command log forwarder already ended');
      consume(typeof chunk === 'string' ? chunk : decoder.write(chunk));
    },
    end() {
      if (ended) return;
      ended = true;consume(decoder.end());
      if (bytes) flush('');
    },
  };
}

/** Read a bounded actual log suffix; no full command log is loaded in memory. */
export async function readEvidenceCommandFailureTail(filename, environment = process.env) {
  const handle = await open(filename, 'r');
  try {
    const { size } = await handle.stat();
    const start = Math.max(0, size - 32 * 1024);
    const buffer = Buffer.alloc(Math.min(size, 32 * 1024));
    const { bytesRead } = await handle.read(buffer, 0, buffer.length, start);
    let text = buffer.subarray(0, bytesRead).toString('utf8');
    // Drop a partial first line before redaction so a sliced credential is not printed.
    if (start > 0) text = text.includes('\n') ? text.slice(text.indexOf('\n') + 1) : '';
    const redacted = redactEvidenceCommandLog(text, environment).split(/\r?\n/u).slice(-40).join('\n');
    return redacted.length <= 8000 ? redacted : redacted.slice(-8000).replace(/^[^\n]*\n/u, '');
  } finally { await handle.close(); }
}
