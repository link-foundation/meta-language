// Finite, read-only reproduction of setup-ocaml's forced-color environment.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';

const commands = [
  ['api', 'repos/link-foundation/meta-language'],
  ['pr', 'view', '196', '--repo', 'link-foundation/meta-language', '--json', 'headRefOid'],
];
for (const forcedTerminal of [false, true]) {
  for (const clearForcedSettings of [false, true]) {
    const environment = { ...process.env, CLICOLOR_FORCE: '1', NO_COLOR: '1' };
    delete environment.GH_FORCE_TTY;
    if (forcedTerminal) environment.GH_FORCE_TTY = '1';
    if (clearForcedSettings) {
      delete environment.CLICOLOR_FORCE;
      delete environment.GH_FORCE_TTY;
      environment.CLICOLOR = '0';
    }
    for (const args of commands) {
      const output = execFileSync('gh', args, { encoding: 'utf8', env: environment, maxBuffer: 1024 * 1024 });
      const colored = output.includes('\x1b[');
      if (clearForcedSettings) {
        assert.equal(colored, false);
        assert.equal(typeof JSON.parse(output), 'object');
      } else if (colored) {
        assert.throws(() => JSON.parse(output), SyntaxError);
      } else {
        assert.equal(typeof JSON.parse(output), 'object');
      }
      console.log(`${args[0]}: GH_FORCE_TTY=${forcedTerminal}; forced controls cleared=${clearForcedSettings}; ANSI=${colored}`);
    }
  }
}
