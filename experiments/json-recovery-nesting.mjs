// Run with a bounded stack, for example node --stack-size=984 --max-old-space-size=256.
import { recoverJson } from '../js/src/index.js';

for (const depth of [32, 48, 55, 60, 64, 68, 72, 76, 80, 96, 112, 127]) {
  for (const [opener, closer] of [['{"value":', '}'], ['[', ']']]) {
    const input = opener.repeat(depth) + '0';
    const result = recoverJson(input, { allow: ['closeOpenContainers'] });
    console.log(closer, depth, result.edits.length, result.issues);
  }
}
