import assert from 'node:assert/strict';
import { test } from 'node:test';

import {
  SETUP_NODE_PLACEHOLDER_TOKEN,
  buildMissingCredentialGuidance,
  buildTrustedPublisherSettings,
  isUsableToken,
  prepareNpmAuth,
  resolveAuthMode,
  sanitizeNpmrc,
  verifyTrustedPublishing,
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
  assert.match(guidance, /workflow\s+filename js\.yml/);
});

// A GitHub OIDC token whose claims name this repository's publish workflow.
const ID_TOKEN = ['header', Buffer.from(JSON.stringify({
  repository: 'link-foundation/meta-language',
  workflow_ref: 'link-foundation/meta-language/.github/workflows/js.yml@refs/tags/v0.58.3',
  ref: 'refs/tags/v0.58.3',
  event_name: 'workflow_dispatch',
  sub: 'repo:link-foundation/meta-language:ref:refs/tags/v0.58.3',
})).toString('base64url'), 'signature'].join('.');
const OIDC_ENV = {
  ACTIONS_ID_TOKEN_REQUEST_URL: 'https://token.actions.example/request?api-version=2.0',
  ACTIONS_ID_TOKEN_REQUEST_TOKEN: 'request-token',
};

/** A fetch double that answers the GitHub token request and the npm exchange. */
function registryDouble(exchange) {
  const requests = [];
  const fetchImpl = async (url, init = {}) => {
    requests.push({ url: String(url), method: init.method ?? 'GET', authorization: init.headers?.Authorization });
    if (String(url).startsWith('https://token.actions.example/')) {
      return new Response(JSON.stringify({ value: ID_TOKEN }), { status: 200 });
    }
    return exchange();
  };
  return { requests, fetchImpl };
}

test('the trusted-publisher probe performs the exchange npm publish performs', async () => {
  const { requests, fetchImpl } = registryDouble(() => new Response(JSON.stringify({ token: 'npm_short_lived' }), { status: 201 }));
  const result = await verifyTrustedPublishing({ env: OIDC_ENV, packageName: 'meta-language', fetchImpl });

  assert.equal(result.ok, true);
  assert.deepEqual(requests.map(({ url, method }) => [method, url]), [
    ['GET', 'https://token.actions.example/request?api-version=2.0&audience=npm%3Aregistry.npmjs.org'],
    ['POST', 'https://registry.npmjs.org/-/npm/v1/oidc/token/exchange/package/meta-language'],
  ]);
  assert.equal(requests[0].authorization, 'Bearer request-token');
  assert.equal(requests[1].authorization, `Bearer ${ID_TOKEN}`);
  assert.equal(result.claims.workflow_ref, 'link-foundation/meta-language/.github/workflows/js.yml@refs/tags/v0.58.3');
  assert.equal(JSON.stringify(result).includes('npm_short_lived'), false);
});

test('a refused exchange reports the registry answer instead of a later ENEEDAUTH', async () => {
  const { fetchImpl } = registryDouble(() => new Response(
    JSON.stringify({ message: 'No trusted publisher configured for this package' }), { status: 404 },
  ));
  const result = await verifyTrustedPublishing({ env: OIDC_ENV, packageName: 'meta-language', fetchImpl });

  assert.equal(result.ok, false);
  assert.equal(result.stage, 'exchange');
  assert.equal(result.status, 404);
  assert.match(result.message, /No trusted publisher configured/);
  assert.equal(result.claims.repository, 'link-foundation/meta-language');
});

test('a run without id-token permission is reported before any request', async () => {
  const { requests, fetchImpl } = registryDouble(() => assert.fail('no exchange expected'));
  const result = await verifyTrustedPublishing({ env: {}, packageName: 'meta-language', fetchImpl });

  assert.equal(result.ok, false);
  assert.equal(result.stage, 'id-token');
  assert.match(result.message, /id-token: write/);
  assert.deepEqual(requests, []);
});

test('the documented trusted-publisher settings match the publish workflow', async () => {
  const { readFile } = await import('node:fs/promises');
  const settings = buildTrustedPublisherSettings('meta-language', 'link-foundation/meta-language', '.github/workflows/js.yml');
  const documentation = await readFile(new URL('../../docs/ci-cd/npm-trusted-publishing.md', import.meta.url), 'utf8');
  const workflow = await readFile(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8');
  const publishJob = workflow.slice(workflow.indexOf('\n  publish:\n'));

  for (const [field, value] of [['Organization or user', 'link-foundation'], ['Repository', 'meta-language'], ['Workflow filename', 'js.yml']]) {
    assert.ok(settings.includes(`${field}: ${value}`), field);
    assert.ok(documentation.includes(`| ${field} | \`${value}\` |`), field);
  }
  assert.match(publishJob, /id-token: write/);
  assert.doesNotMatch(publishJob, /\n {4}environment:/);
  assert.match(publishJob, /node scripts\/prepare-npm-auth\.mjs --verify-exchange/);
});
