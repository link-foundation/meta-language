import {draftSourceLibrary} from './draft-source-library.mjs';

if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');

draftSourceLibrary({
  sourcePath: 'js/src/link-cli-archive-topology-rules.js',
  moduleName: 'link_cli_archive_topology_rules',
  directory: '',
  model: 'reversible ordered graph incidence in link-cli stores 1',
  programAsset: 'link-cli-archive-topology-program',
});
