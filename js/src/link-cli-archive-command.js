import { runMetaLanguageArchiveCommand } from './meta-language-archive-cli.js';
import { metaLanguageArchiveToBinary, metaLanguageArchiveToLinkCliStore,
  metaLanguageArchiveToText, readMetaLanguageArchiveBinary,
  readMetaLanguageArchiveLinkCliStore, readMetaLanguageArchiveText } from './meta-language-archive.js';

const usage = 'usage: meta-language archive pack [--binary|--link-cli-store] --entry <identifier> <kind> <root> <links-file> ...\n'
  + '       meta-language archive list|select|extend|convert <archive-file> [--input-binary|--input-link-cli-store] [--binary|--link-cli-store]\n'
  + '         select: --identifier <identifier> ...\n'
  + '         extend: [--replace] [--remove <identifier>] [--entry <identifier> <kind> <root> <links-file>] ...\n';

function readArchiveCommandEncodings(command, values) {
  const args = [];
  let store = false, inputStore = false, binary = false, inputBinary = false;
  for (let index = 0; index < values.length; index++) {
    const option = values[index];
    if (option === '--link-cli-store') store = true;
    else if (option === '--input-link-cli-store') inputStore = true;
    else if (option === '--binary') binary = true;
    else if (option === '--input-binary') inputBinary = true;
    else {
      args.push(option);
      const remaining = values.length - index - 1;
      const entry = option === '--entry' && (command === 'pack' || command === 'extend');
      const selection = (option === '--identifier' && command === 'select')
        || (option === '--remove' && command === 'extend');
      const operands = entry && remaining >= 4 ? 4 : selection && remaining ? 1 : 0;
      for (let count = 0; count < operands; count++) args.push(values[++index]);
    }
  }
  if (store && binary || inputStore && inputBinary) throw new TypeError('select one archive encoding per input or output');
  return { args, store, inputStore, binary, inputBinary };
}

export function runLinkCliArchiveCommand(args, { readFile, readBytes }) {
  try {
    const [command, ...rest] = args;
    if (command === 'help' || command === '--help') return { exitCode: 0, stdout: usage, stderr: '' };
    const format = readArchiveCommandEncodings(command, rest);
    if (command === 'convert') {
      if (format.args.length !== 1 || format.args[0].startsWith('--')) throw new TypeError('convert needs one archive input');
      const [file] = format.args;
      const archive = format.inputStore ? readMetaLanguageArchiveLinkCliStore(readBytes(file))
        : format.inputBinary ? readMetaLanguageArchiveBinary(readBytes(file))
        : readMetaLanguageArchiveText(readFile(file));
      const stdout = format.store ? metaLanguageArchiveToLinkCliStore(archive)
        : format.binary ? metaLanguageArchiveToBinary(archive) : metaLanguageArchiveToText(archive);
      return { exitCode: 0, stdout, stderr: '' };
    }
    if (command === 'pack' && format.inputStore) throw new TypeError('--input-link-cli-store applies to an archive input');
    const input = format.inputStore ? file => metaLanguageArchiveToBinary(readMetaLanguageArchiveLinkCliStore(readBytes(file))) : readBytes;
    const legacyArgs = [command, ...format.args];
    if (format.inputBinary || format.inputStore) legacyArgs.push('--input-binary');
    if (format.binary || format.store) legacyArgs.push('--binary');
    const result = runMetaLanguageArchiveCommand(legacyArgs, { readFile, readBytes: input });
    if (result.exitCode || !format.store) return result;
    return { ...result, stdout: metaLanguageArchiveToLinkCliStore(readMetaLanguageArchiveBinary(result.stdout)) };
  } catch (error) {
    return { exitCode: 2, stdout: '', stderr: `error: ${error.message}\n${usage}` };
  }
}
