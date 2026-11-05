import { unsupported as makeUnsupported, typeError as makeTypeError } from '../../js/src/translation/diagnostics.js';

export function diagnostic(construct, reason, span) {
  return makeUnsupported(construct, reason, span);
}

export function reject(construct, reason, span) {
  throw makeUnsupported(construct, reason, span);
}

export function typed(message, span) {
  return makeTypeError(message, span);
}
