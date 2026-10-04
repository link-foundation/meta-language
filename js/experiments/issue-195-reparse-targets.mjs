// Re-reads emitted target artifacts with the target frontend and compares theorem statements.
import { readFileSync } from 'node:fs';
import { parseLean } from '../src/translation/lean.js';
import { parseRocq } from '../src/translation/rocq.js';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
const frontends = { lean: parseLean, v: parseRocq, rs: parseRust, mjs: parseJavaScript };
for (const dir of process.argv.slice(2)) {
  for (const [file, ext] of [['Out.lean', 'lean'], ['Out.v', 'v'], ['out.rs', 'rs'], ['out.mjs', 'mjs']]) {
    try {
      const program = checkProgram(frontends[ext](readFileSync(`${dir}/${file}`, 'utf8')));
      const theorems = [...program.declarations.values()].filter((d) => d.k === 'theorem').map((d) => d.fullName);
      console.log(dir, file, 'ok', theorems.join(','));
    } catch (error) {
      console.log(dir, file, 'FAIL', error.message.slice(0, 200));
    }
  }
}
