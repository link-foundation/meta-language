// Text output remains Links Notation. Binary output is the actual graph codec.
import { LinkNetwork } from './network.js';
import { createMetaLanguageArchive, readMetaLanguageArchiveText,
  readMetaLanguageArchiveBinary, metaLanguageArchiveToText,
  metaLanguageArchiveToBinary, extendMetaLanguageArchive } from './meta-language-archive.js';
import { structuredDataToLino } from './structured-data-links.js';

const usage = 'usage: meta-language archive pack [--binary] --entry <identifier> <kind> <root> <links-file> ...\n'
  + '       meta-language archive list|select|extend <archive-file> [--input-binary] [--binary]\n'
  + '         select: --identifier <identifier> ...\n'
  + '         extend: [--replace] [--remove <identifier>] [--entry <identifier> <kind> <root> <links-file>] ...\n';

export function runMetaLanguageArchiveCommand(args, { readFile, readBytes }) {
  try {
    const [command, ...rest] = args;
    if (command === 'help' || command === '--help') return { exitCode: 0, stdout: usage, stderr: '' };
    if (!['pack', 'list', 'select', 'extend'].includes(command)) throw new TypeError('unknown archive operation');
    const entries = [], identifiers = [], remove = [], networks = new Map();
    let file, binary = false, inputBinary = false, replace = false;
    for (let index = 0; index < rest.length; index += 1) {
      const option = rest[index];
      if (option === '--binary') binary = true;
      else if (option === '--input-binary') inputBinary = true;
      else if (option === '--replace' && command === 'extend') replace = true;
      else if (option === '--identifier' && command === 'select' && index + 1 < rest.length) identifiers.push(rest[++index]);
      else if (option === '--remove' && command === 'extend' && index + 1 < rest.length) remove.push(rest[++index]);
      else if (option === '--entry' && ['pack', 'extend'].includes(command) && index + 4 < rest.length) {
        const identifier = rest[++index], kind = rest[++index], root = rest[++index], path = rest[++index];
        if (!/^[1-9][0-9]*$/u.test(root)) throw new TypeError('an entry root needs a positive decimal identity');
        if (!networks.has(path)) networks.set(path, LinkNetwork.fromLino(readFile(path)));
        entries.push({ identifier, kind, root, network: networks.get(path) });
      } else if (!option.startsWith('--') && file === undefined && command !== 'pack') file = option;
      else throw new TypeError(`unexpected archive argument ${option}`);
    }
    let archive;
    if (command === 'pack') {
      if (inputBinary) throw new TypeError('--input-binary applies to an archive input');
      archive = createMetaLanguageArchive(entries);
    } else {
      if (!file) throw new TypeError('an archive input is required');
      archive = inputBinary ? readMetaLanguageArchiveBinary(readBytes(file)) : readMetaLanguageArchiveText(readFile(file));
      if (command === 'list') {
        if (binary) throw new TypeError('list produces structured Links text');
        return { exitCode: 0, stdout: structuredDataToLino(archive.entries.map(entry => ({
          identifier: entry.identifier, kind: entry.kind, root: entry.root.asU64(),
          nodes: entry.network.links().length,
        }))), stderr: '' };
      }
      if (command === 'select') {
        if (!identifiers.length || new Set(identifiers).size !== identifiers.length) throw new TypeError('select needs distinct identifiers');
        const selected = identifiers.map(identifier => {
          const entry = archive.entries.find(entry => entry.identifier === identifier);
          if (!entry) throw new TypeError(`unknown archive entry ${identifier}`);
          return entry;
        });
        archive = createMetaLanguageArchive(selected);
      } else archive = extendMetaLanguageArchive(archive, entries, { replace, remove });
    }
    return { exitCode: 0, stdout: binary ? metaLanguageArchiveToBinary(archive) : metaLanguageArchiveToText(archive), stderr: '' };
  } catch (error) {
    return { exitCode: 2, stdout: '', stderr: `error: ${error.message}\n${usage}` };
  }
}
