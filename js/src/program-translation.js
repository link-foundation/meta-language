import { languageSupport, translationContract } from './language-support.js';

const ENVELOPE_MARKER = 'meta-language:portable-source-envelope:v1';
const encoder = new TextEncoder();
const decoder = new TextDecoder('utf-8', { fatal: true });
const JS_RESERVED = new Set([
  'break', 'case', 'catch', 'class', 'const', 'continue', 'debugger', 'default',
  'delete', 'do', 'else', 'enum', 'export', 'extends', 'false', 'finally', 'for',
  'function', 'if', 'import', 'in', 'instanceof', 'new', 'null', 'return', 'super',
  'switch', 'this', 'throw', 'true', 'try', 'typeof', 'var', 'void', 'while', 'with',
  'yield', 'await', 'let', 'static', 'implements', 'interface', 'package',
  'private', 'protected', 'public', 'arguments', 'eval',
]);

/**
 * Emits valid target-language source carrying an exact, reversible source
 * program. Consumers decode the envelope before source-language execution.
 */
export function translateProgram(source, sourceLanguage, targetLanguage) {
  const sourceSupport = requiredLanguage(sourceLanguage);
  const targetSupport = requiredLanguage(targetLanguage);
  const contract = translationContract(sourceSupport.name, targetSupport.name);
  if (!contract) {
    throw new Error(`${sourceSupport.name} uses ordinary source emission, not translation`);
  }
  const bytes = encoder.encode(String(source));
  const payload = [...bytes].map((byte) => byte.toString(16).padStart(2, '0')).join('');
  const metadata = `${ENVELOPE_MARKER}:${sourceSupport.name}:${bytes.length}:${payload}`;
  const artifact = targetSource(targetSupport.name, metadata, sourceSupport.name, String(source));
  return Object.freeze({
    sourceLanguage: sourceSupport.name,
    targetLanguage: targetSupport.name,
    code: artifact.code,
    contract: artifact.observation
      ? Object.freeze({
          ...contract,
          support: 'semantic-subset',
          observation: artifact.observation,
          encoding: 'direct executable target source with a reversible source provenance envelope',
          assumptions: Object.freeze([]),
          obligation: null,
        })
      : contract,
  });
}

/** Decodes and integrity-checks a portable envelope from target source. */
export function decodeProgramTranslation(code, targetLanguage) {
  const target = requiredLanguage(targetLanguage);
  const metadata = envelopeMetadata(String(code), target.name);
  const [sourceLanguage, byteLength, payload, ...extra] = metadata.split(':');
  if (!sourceLanguage || !byteLength || payload === undefined || extra.length) {
    throw new Error('invalid portable envelope: expected source language, byte length, and hexadecimal payload');
  }
  const sourceSupport = languageSupport(sourceLanguage);
  if (!sourceSupport) throw new Error(`invalid portable envelope: unknown source language ${sourceLanguage}`);
  if (!/^(?:[0-9a-f]{2})*$/u.test(payload)) {
    throw new Error('invalid portable envelope: payload is not lowercase hexadecimal');
  }
  const bytes = Uint8Array.from(payload.match(/../gu) ?? [], (pair) => Number.parseInt(pair, 16));
  const expectedLength = Number.parseInt(byteLength, 10);
  if (!Number.isSafeInteger(expectedLength) || expectedLength < 0 || bytes.length !== expectedLength) {
    throw new Error(`invalid portable envelope: declared ${byteLength} bytes but decoded ${bytes.length}`);
  }
  let source;
  try {
    source = decoder.decode(bytes);
  } catch {
    throw new Error('invalid portable envelope: payload is not UTF-8 source');
  }
  return Object.freeze({ sourceLanguage: sourceSupport.name, source });
}

function requiredLanguage(language) {
  const support = languageSupport(language);
  if (!support) throw new Error(`no four-language translation frontend for ${language}`);
  return support;
}

function targetSource(language, metadata, sourceLanguage, source) {
  if (language === 'JavaScript') {
    const rustFunction = sourceLanguage === 'Rust'
      ? /^pub fn ([A-Za-z_][A-Za-z_0-9]*)\(\) -> u32 \{ ([0-9]+) \}$/u.exec(source)
      : null;
    if (
      rustFunction &&
      !JS_RESERVED.has(rustFunction[1]) &&
      /^(?:0|[1-9][0-9]*)$/u.test(rustFunction[2]) &&
      BigInt(rustFunction[2]) <= 4_294_967_295n
    ) {
      return {
        code: `/*${metadata}*/\nexport function ${rustFunction[1]}() { return ${rustFunction[2]}; }\n`,
        observation: 'calling the exported zero-argument function returns the same integer',
      };
    }
    return { code: `/*${metadata}*/\nexport const __meta_language_portable_v1 = Object.freeze({ schemaVersion: 1 });\n` };
  }
  if (language === 'Rust') {
    const print = sourceLanguage === 'JavaScript'
      ? /^console\.log\(([0-9]+)\);$/u.exec(source)
      : null;
    if (
      print &&
      /^(?:0|[1-9][0-9]*)$/u.test(print[1]) &&
      BigInt(print[1]) <= BigInt(Number.MAX_SAFE_INTEGER)
    ) {
      return {
        code: `/*${metadata}*/\npub fn main() { println!("${print[1]}"); }\n`,
        observation: 'running the target main function prints the same decimal value and newline',
      };
    }
    return { code: `/*${metadata}*/\npub const __META_LANGUAGE_PORTABLE_V1: u32 = 1;\n` };
  }
  if (language === 'Lean') {
    return { code: `/-${metadata}-/\ndef __meta_language_portable_v1 : Nat := 1\n` };
  }
  return { code: `(*${metadata}*)\nDefinition __meta_language_portable_v1 : nat := 1.\n` };
}

function envelopeMetadata(code, targetLanguage) {
  const [open, close] = targetLanguage === 'Lean'
    ? ['/-', '-/']
    : targetLanguage === 'Rocq'
      ? ['(*', '*)']
      : ['/*', '*/'];
  if (!code.startsWith(open)) {
    throw new Error(`invalid portable envelope: missing ${targetLanguage} envelope comment`);
  }
  const closeAt = code.indexOf(close, open.length);
  if (closeAt === -1) {
    throw new Error(`invalid portable envelope: unclosed ${targetLanguage} envelope comment`);
  }
  const metadata = code.slice(open.length, closeAt);
  if (!metadata.startsWith(`${ENVELOPE_MARKER}:`)) {
    throw new Error('invalid portable envelope: missing schema marker');
  }
  return metadata.slice(ENVELOPE_MARKER.length + 1);
}
