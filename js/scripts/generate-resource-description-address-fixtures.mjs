// Normative input declarations only; no address parser is imported or run.
import { writeProjectData } from './project-data.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft to author normative address fixtures');
writeProjectData(new URL('../../parity/fixtures/resource-description-addresses.lino', import.meta.url), {
  schemaVersion: 1, specification: 'RFC 3987 section 2.2; RFC 3986 section 3.2.2',
  versions: { JavaScript: 'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0', Rust: '>=1.90.0 <1.100.0; edition 2024' },
  valid: ['http://[::]/', 'http://[::1]/', 'https://[2001:db8::1]:443/🙂?x#y', 'http://[1:2:3:4:5:6:7:8]/',
    'http://[1:2:3:4:5:6:7::]/', 'http://[::ffff:192.0.2.1]/', 'http://[1:2:3:4:5:6:192.0.2.1]/',
    'http://user:password@[::1]:/p?x#f', 'http://[::1]:99999/', 'http://[v1.example:future]/', 'http://[VF.a!$&\'()*+,;=:~_]/'],
  invalid: ['http://[]/', 'http://[1:2:3:4:5:6:7]/', 'http://[1:2:3:4:5:6:7:8:9]/', 'http://[1:2:3:4:5:6:7:8::]/',
    'http://[1::2::3]/', 'http://[:::]/', 'http://[:1]/', 'http://[1:]/', 'http://[12345::]/', 'http://[gg::]/',
    'http://[::ffff:256.1.1.1]/', 'http://[::ffff:192.00.2.1]/', 'http://[::ffff:192.0.2.]/', 'http://[192.0.2.1]/',
    'http://[::ffff:192.0.2.1]:bad/', 'http://[::1]suffix/', 'http://[v.example]/', 'http://[vG.example]/', 'http://[v1.]/',
    'http://[v1.%20]/', 'http://[fe80::1%25eth0]/', 'http://[::1/path', 'urn:[::1]', 'http://a@b@[::1]/'],
});
