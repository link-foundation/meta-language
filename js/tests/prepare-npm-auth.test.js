import assert from 'node:assert/strict';
import { test } from 'node:test';

import {
  SETUP_NODE_PLACEHOLDER_TOKEN,
  buildMissingCredentialGuidance,
  isUsableToken,
  prepareNpmAuth,
  resolveAuthMode,
  sanitizeNpmrc,
} from '../scripts/prepare-npm-auth.mjs';

// The exact npmrc actions/setup-node writes when `registry-url` is set.
const SETUP_NODE_NPMRC = [
  '//registry.npmjs.org/:_authToken=${NODE_AUTH_TOKEN}',
  'registry=https://registry.npmjs.org/',
  'always-auth=false',
  '',
].join('\n');

function silentLogger() {
  const lines = [];
  return {
    lines,
    log: (message) => lines.push(String(message)),
    warn: (message) => lines.push(String(message)),
  };
}

test('the setup-node placeholder is never treated as a usable credential', () => {
  assert.equal(isUsableToken(SETUP_NODE_PLACEHOLDER_TOKEN), false);
  assert.equal(isUsableToken('${NODE_AUTH_TOKEN}'), false);
  assert.equal(isUsableToken(''), false);
  assert.equal(isUsableToken('   '), false);
  assert.equal(isUsableToken(undefined), false);
  assert.equal(isUsableToken('npm_realtokenvalue'), true);
});

test('auth mode prefers a real token, then OIDC, then reports none', () => {
  assert.equal(resolveAuthMode({ NPM_TOKEN: 'npm_real' }), 'token');
  assert.equal(resolveAuthMode({ NODE_AUTH_TOKEN: 'npm_real' }), 'token');
  assert.equal(
    resolveAuthMode({
      NODE_AUTH_TOKEN: SETUP_NODE_PLACEHOLDER_TOKEN,
      ACTIONS_ID_TOKEN_REQUEST_URL: 'https://example.invalid/token',
      ACTIONS_ID_TOKEN_REQUEST_TOKEN: 'request-token',
    }),
    'oidc',
  );
  assert.equal(resolveAuthMode({ NODE_AUTH_TOKEN: SETUP_NODE_PLACEHOLDER_TOKEN }), 'none');
  assert.equal(resolveAuthMode({}), 'none');
});

test('sanitizing removes the auth token and deprecated always-auth entries', () => {
  const result = sanitizeNpmrc(SETUP_NODE_NPMRC);

  assert.equal(result.removedAuthToken, true);
  assert.equal(result.removedAlwaysAuth, true);
  assert.equal(result.content, 'registry=https://registry.npmjs.org/\n');
});

test('sanitizing keeps the auth token when a real credential is in play', () => {
  const result = sanitizeNpmrc(SETUP_NODE_NPMRC, { keepAuthToken: true });

  assert.equal(result.removedAuthToken, false);
  assert.equal(result.removedAlwaysAuth, true);
  assert.ok(result.content.includes('_authToken'));
});

// Reproduces issue #191: with only the setup-node placeholder present, npm sees
// configured credentials, skips the OIDC exchange, and the registry answers the
// anonymous PUT with E404. Sanitizing the npmrc is what unblocks OIDC.
test('OIDC runs unblock the npmrc that setup-node wrote', () => {
  const files = { '/tmp/.npmrc': SETUP_NODE_NPMRC };
  const logger = silentLogger();

  const result = prepareNpmAuth({
    env: {
      NPM_CONFIG_USERCONFIG: '/tmp/.npmrc',
      NODE_AUTH_TOKEN: SETUP_NODE_PLACEHOLDER_TOKEN,
      ACTIONS_ID_TOKEN_REQUEST_URL: 'https://example.invalid/token',
      ACTIONS_ID_TOKEN_REQUEST_TOKEN: 'request-token',
    },
    logger,
    fileExists: (target) => target in files,
    readFile: (target) => files[target],
    writeFile: (target, content) => {
      files[target] = content;
    },
  });

  assert.equal(result.mode, 'oidc');
  assert.equal(result.changed, true);
  assert.equal(files['/tmp/.npmrc'].includes('_authToken'), false);
  assert.ok(files['/tmp/.npmrc'].includes('registry=https://registry.npmjs.org/'));
});

test('token runs keep the credential line that npm needs', () => {
  const files = { '/tmp/.npmrc': SETUP_NODE_NPMRC };

  const result = prepareNpmAuth({
    env: {
      NPM_CONFIG_USERCONFIG: '/tmp/.npmrc',
      NODE_AUTH_TOKEN: 'npm_real',
    },
    logger: silentLogger(),
    fileExists: (target) => target in files,
    readFile: (target) => files[target],
    writeFile: (target, content) => {
      files[target] = content;
    },
  });

  assert.equal(result.mode, 'token');
  assert.ok(files['/tmp/.npmrc'].includes('_authToken'));
  assert.equal(files['/tmp/.npmrc'].includes('always-auth'), false);
});

test('a missing npm user config is reported instead of crashing', () => {
  const logger = silentLogger();
  const result = prepareNpmAuth({
    env: { NPM_CONFIG_USERCONFIG: '/tmp/absent.npmrc' },
    logger,
    fileExists: () => false,
    readFile: () => {
      throw new Error('should not read');
    },
    writeFile: () => {
      throw new Error('should not write');
    },
  });

  assert.equal(result.skipped, true);
  assert.equal(result.mode, 'none');
});

test('verbose mode logs config keys but never credential values', () => {
  const files = { '/tmp/.npmrc': '//registry.npmjs.org/:_authToken=npm_supersecret\n' };
  const logger = silentLogger();

  prepareNpmAuth({
    env: {
      NPM_CONFIG_USERCONFIG: '/tmp/.npmrc',
      ACTIONS_ID_TOKEN_REQUEST_URL: 'https://example.invalid/token',
      ACTIONS_ID_TOKEN_REQUEST_TOKEN: 'request-token',
    },
    logger,
    verbose: true,
    fileExists: () => true,
    readFile: (target) => files[target],
    writeFile: (target, content) => {
      files[target] = content;
    },
  });

  const output = logger.lines.join('\n');
  assert.ok(output.includes('_authToken'));
  assert.equal(output.includes('npm_supersecret'), false);
});

test('missing-credential guidance names both remediation paths', () => {
  const guidance = buildMissingCredentialGuidance(
    'meta-language',
    'link-foundation/meta-language',
    '.github/workflows/js.yml',
  );

  assert.ok(guidance.includes('trusted publisher'));
  assert.ok(guidance.includes('NPM_TOKEN'));
  assert.ok(guidance.includes('.github/workflows/js.yml'));
});
