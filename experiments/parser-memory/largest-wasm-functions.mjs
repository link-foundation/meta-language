// Lists the largest function bodies of vendored grammar modules, with export
// names where present, to find the functions whose TurboFan tier-up is costly.
// Usage: node largest-wasm-functions.mjs <grammar id>...
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';

function leb(bytes, state) {
  let result = 0, shift = 0, byte;
  do { byte = bytes[state.at++]; result += (byte & 0x7f) * 2 ** shift; shift += 7; } while (byte & 0x80);
  return result;
}

for (const id of process.argv.slice(2)) {
  const bytes = gunzipSync(readFileSync(new URL(`../../js/src/vendor/grammars/${id}.wasm.gz`, import.meta.url)));
  const module = new WebAssembly.Module(bytes);
  const imported = WebAssembly.Module.imports(module).filter(({ kind }) => kind === 'function').length;
  const state = { at: 8 };
  let bodies = [];
  const names = new Map();
  while (state.at < bytes.length) {
    const section = bytes[state.at++];
    const size = leb(bytes, state);
    const end = state.at + size;
    if (section === 7) {
      const count = leb(bytes, state);
      for (let index = 0; index < count; index += 1) {
        const length = leb(bytes, state);
        const name = new TextDecoder().decode(bytes.subarray(state.at, state.at + length));
        state.at += length;
        const kind = bytes[state.at++];
        const target = leb(bytes, state);
        if (kind === 0) names.set(target, name);
      }
    }
    if (section === 10) {
      const count = leb(bytes, state);
      for (let index = 0; index < count; index += 1) {
        const length = leb(bytes, state);
        bodies.push({ index: imported + index, length });
        state.at += length;
      }
    }
    state.at = end;
  }
  bodies.sort((a, b) => b.length - a.length);
  console.log(id, `${bytes.length} bytes,`, bodies.slice(0, 3).map(({ index, length }) => `${names.get(index) ?? `#${index}`}=${length}`).join(' '));
}
