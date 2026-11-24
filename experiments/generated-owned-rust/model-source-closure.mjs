import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';

// Exact inverse of the private proof seams and historical lexer support.
const activeSha256 = '54eece16dd41eb919da30f2b253a9d63eb58f0f9ba1c86855225a381ecb73856';
const inverse = [
  ["function checkCst(proof,selected,lexRust=source=>tokenize(source,'Rust').tokens){\n",
   "function checkCst(proof,selected){\n"],
  [" const lexical=lexRust(text);if(lexical.length!==proof.tokens.length)fail('the trivia or literal spelling changes active source tokens');\n",
   " const lexical=tokenize(text,'Rust').tokens;if(lexical.length!==proof.tokens.length)fail('the trivia or literal spelling changes active source tokens');\n"],
];
const sha256 = bytes => createHash('sha256').update(bytes).digest('hex');

export function verifyGeneratedOwnedRustModelSource(file, bytes, producerSha256) {
  const sha256Active = sha256(bytes);
  if (sha256Active === producerSha256) return {file, sha256: sha256Active};
  assert.equal(sha256Active, activeSha256, 'the frozen generated-owned Rust model changed: ' + file);
  let original = bytes.toString('utf8');
  for (const [active, previous] of inverse) {
    assert.equal(original.split(active).length, 2, 'each reviewed edit must have one source owner');
    original = original.replace(active, () => previous);
  }
  assert.equal(sha256(original), producerSha256, 'the reviewed model must recover every producer byte');
  return {file, sha256: sha256Active, producerSha256};
}
