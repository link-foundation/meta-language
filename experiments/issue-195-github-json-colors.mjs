// Finite, read-only reproduction of setup-ocaml's forced-color environment.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';

const args = ['pr', 'view', '196', '--repo', 'link-foundation/meta-language', '--json', 'headRefOid'];
for (const disableColors of [false, true]) {
  const environment = { ...process.env, CLICOLOR_FORCE: '1', GH_FORCE_TTY: '1' };
  delete environment.NO_COLOR;
  if (disableColors) environment.NO_COLOR = '1';
  const output = execFileSync('gh', args, { encoding: 'utf8', env: environment, maxBuffer: 1024 * 1024 });
  if (disableColors) {
    const result = JSON.parse(output);
    assert.match(result.headRefOid, /^[a-f0-9]{40}$/);
    console.log('NO_COLOR=1: valid JSON; inherited forced-color controls do not affect parsing');
  } else {
    assert.match(output, /\x1b\[/);
    assert.throws(() => JSON.parse(output), SyntaxError);
    console.log('CLICOLOR_FORCE=1 and GH_FORCE_TTY=1: ANSI output reproduces the JSON parsing failure');
  }
}
