// Future independent module parser only; never invoke during drafting.
import { spawnSync } from 'node:child_process';

export function readJavaScriptModuleSyntaxError(source) {
  const result = spawnSync(process.execPath, ['--input-type=module', '--check'], {
    input: source, encoding: 'utf8', timeout: 5000, maxBuffer: 1048576,
  });
  if (result.error) throw result.error;
  if (!Number.isInteger(result.status) || result.status === 0 || result.signal !== null || !result.stderr.includes('SyntaxError'))
    throw new Error('the independent Node module parser did not produce the declared syntax error');
  return { source, exitCode: result.status, signal: result.signal, stdout: result.stdout, stderr: result.stderr };
}
