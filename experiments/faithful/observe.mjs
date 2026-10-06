// Runs a source program with its own toolchain and its translation into each
// of JavaScript, Rust, Lean and Rocq, and prints what each run prints and the
// message it aborts with. Usage: node experiments/faithful/observe.mjs <file> <Language>
import { execFile } from 'node:child_process';
import { mkdtemp, readFile, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { translateProgram } from '../../js/src/index.js';

const EXT = { JavaScript: '.mjs', Rust: '.rs', Lean: '.lean', Rocq: '.v' };
const run = (command, args, cwd) => new Promise((resolve) => {
  execFile(command, args, { cwd, maxBuffer: 1 << 26, encoding: 'utf8' }, (error, stdout, stderr) => {
    resolve({ code: error ? error.code ?? 1 : 0, stdout, stderr });
  });
});
const quoted = (text) => [...text.matchAll(/"((?:[^"]|"")*)"/gu)].map((match) => match[1].replaceAll('""', '"'));

async function observe(language, code, directory, name) {
  const file = path.join(directory, `${name}${EXT[language]}`);
  await writeFile(file, code);
  if (language === 'JavaScript') {
    const result = await run('node', [file]);
    const abort = result.code === 0 ? null : (result.stderr.match(/^\w*Error(?:: (.*))?$/mu)?.[1] ?? '');
    return { lines: result.stdout.split('\n').slice(0, -1), abort, code: result.code };
  }
  if (language === 'Rust') {
    const binary = path.join(directory, name);
    const built = await run('rustc', ['--edition', '2024', '-O', '-C', 'overflow-checks=on', '-o', binary, file]);
    if (built.code !== 0) return { rejected: built.stderr };
    const result = await run(binary, []);
    const abort = result.code === 0 ? null : (result.stderr.match(/panicked at [^\n]*\n([^\n]*)/u)?.[1] ?? '');
    return { lines: result.stdout.split('\n').slice(0, -1), abort, code: result.code };
  }
  if (language === 'Lean') {
    const result = await run('lean', ['--run', file]);
    const abort = result.code === 0 ? null : (result.stderr.match(/^uncaught exception: (.*)$/mu)?.[1] ?? result.stderr);
    return { lines: result.stdout.split('\n').slice(0, -1), abort, code: result.code };
  }
  const result = await run('rocq', ['compile', '-q', path.basename(file)], directory);
  if (result.code !== 0) return { rejected: result.stdout + result.stderr };
  const tail = result.stdout.match(/,\s*(None|Some\s+"((?:[^"]|"")*)"(?:%string)?)\s*\)\s*:\s*list string \* option string\s*$/u);
  if (!tail) return { lines: quoted(result.stdout), abort: null, code: 0 };
  return {
    lines: quoted(result.stdout.slice(0, tail.index)),
    abort: tail[1] === 'None' ? null : tail[2].replaceAll('""', '"'),
    code: 0,
  };
}

const [file, language] = process.argv.slice(2);
const source = await readFile(file, 'utf8');
const directory = await mkdtemp(path.join(tmpdir(), 'faithful-'));
console.log('source', JSON.stringify(await observe(language, source, directory, 'source')));
for (const target of Object.keys(EXT).filter((name) => name !== language)) {
  const translation = translateProgram(source, language, target);
  if (translation.contract.support !== 'semantic-translation') {
    console.log(target, translation.contract.support, JSON.stringify(translation.diagnostic));
    continue;
  }
  console.log(target, JSON.stringify(await observe(target, translation.code, directory, `to_${target.toLowerCase()}`)),
    JSON.stringify(translation.semantics.assumptions?.map((assumption) => assumption.id)));
}
console.log(directory);
