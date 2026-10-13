import { open } from 'node:fs/promises';

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
