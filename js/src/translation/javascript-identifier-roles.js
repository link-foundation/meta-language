import { readJavaScriptEscapedIdentifierError } from './javascript-identifier-rules.js';
import { TranslationError } from './diagnostics.js';

export function requireJavaScriptIdentifierRole(token, binding) {
  const message = readJavaScriptEscapedIdentifierError(token.value, token.raw ?? '', binding);
  if (message) throw new TranslationError('syntax', message, token);
}
