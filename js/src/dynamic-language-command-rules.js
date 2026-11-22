// Command vocabulary is shared inspectable source, not a second parser engine.
/** @param {string} command @returns {boolean} */
export function acceptDynamicLanguageCommand(command) {
  return command === 'prepare' || command === 'extend' || command === 'parse'
    || command === 'list' || command === 'grammar' || command === 'discover'
    || command === 'refine' || command === 'install' || command === 'infer-attribute'
    || command === 'translate-attributes';
}
/** A negative arity refuses an option in that operation.
 * @param {string} command @param {string} option @returns {number} */
export function readDynamicLanguageArgumentArity(command, option) {
  if (!acceptDynamicLanguageCommand(command)) return -1;
  if (command === 'infer-attribute') return option === '--grammar' ? 1 : -1;
  if (command === 'translate-attributes') {
    if (option === '--input-binary' || option === '--input-link-cli-store') return 0;
    if (option === '--source' || option === '--input' || option === '--target'
      || option === '--selections' || option === '--start-rule'
      || readDynamicLanguageResourceField(option) !== '') return 1;
    return -1;
  }
  if (option === '--binary' || option === '--link-cli-store') return command === 'list' || command === 'grammar' ? -1 : 0;
  if (option === '--input-binary' || option === '--input-link-cli-store') return command === 'prepare' || command === 'discover' ? -1 : 0;
  if (command === 'discover') return option === '--identifier' || option === '--language' || option === '--edition' ? 1 : -1;
  if (command === 'refine') return option === '--examples' || option === '--identifier' ? 1 : -1;
  if (command === 'install') {
    return option === '--base' || option === '--base-encoding' || option === '--merge-with'
      || option === '--identifier' ? 1 : -1;
  }
  if (option === '--source') return command === 'prepare' || command === 'extend' ? 4 : 1;
  if (option === '--configuration') return command === 'prepare' || command === 'extend' ? 1 : -1;
  if (option === '--replace') return command === 'extend' ? 0 : -1;
  if (option === '--input' || option === '--identifier' || option === '--projection'
    || option === '--start-rule' || option === '--step-limit' || option === '--memory-limit'
    || option === '--max-depth' || option === '--memo-limit' || option === '--max-repairs') return command === 'parse' ? 1 : -1;
  if (option === '--error-recovery' || option === '--accept-recovery' || option === '--reject-ambiguity'
    || option === '--replace-snapshot') return command === 'parse' ? 0 : -1;
  return -1;
}
/** @param {string} projection @returns {string} */
export function readDynamicLanguageProjection(projection) {
  if (projection === 'cst' || projection === 'concrete syntax') return 'concrete syntax';
  if (projection === 'ast' || projection === 'abstract syntax') return 'abstract syntax';
  return '';
}
/** @param {string} field @returns {boolean} */
export function acceptDynamicLanguageConfigurationField(field) {
  return field === 'records' || field === 'formalDefinitions' || field === 'equivalences'
    || field === 'nodeLimit' || field === 'referenceLimit' || field === 'byteLimit' || field === 'depthLimit';
}
/** @param {string} option @returns {string} */
export function readDynamicLanguageResourceField(option) {
  if (option === '--step-limit') return 'stepLimit';
  if (option === '--memory-limit') return 'memoryLimit';
  if (option === '--max-depth') return 'maxDepth';
  if (option === '--memo-limit') return 'memoLimit';
  if (option === '--max-repairs') return 'maxRepairs';
  return '';
}
/** @param {string} encoding @returns {boolean} */
export function acceptDynamicLanguageBaseEncoding(encoding) {
  return encoding === 'text' || encoding === 'binary' || encoding === 'link-cli-store';
}
/** @returns {string} */
export function readDynamicLanguageCommandUsage() {
  return 'usage: meta-language language prepare --source <id> <language> <edition> <grammar.lino> ... [--configuration <links-file>] [--binary]\n'
    + '       meta-language language extend <archive> --source <id> <language> <edition> <grammar.lino> ... [--replace] [--configuration <links-file>] [--input-binary] [--binary]\n'
    + '       meta-language language parse <archive> --source <id> --input <file> [--projection cst|ast] [--identifier <name>] [--replace-snapshot] [--start-rule <rule>]\n'
    + '         [--step-limit <n>] [--memory-limit <n>] [--max-depth <n>] [--memo-limit <n>] [--max-repairs <n>]\n'
    + '         [--error-recovery] [--accept-recovery] [--reject-ambiguity] [--input-binary] [--binary]\n'
    + '       meta-language language list|grammar <archive> [--source <id>] [--input-binary]\n'
    + '       meta-language language discover <examples.lino> [--identifier <id>] [--language <name>] [--edition <edition>] [--binary]\n'
    + '       meta-language language refine <checkpoint> --examples <examples.lino> [--identifier <id>] [--input-binary] [--binary]\n'
    + '       meta-language language install <checkpoint> [--input-binary] [--binary]\n'
    + '         [--base <archive> --merge-with <source> --identifier <merged> [--base-encoding text|binary|link-cli-store]]\n'
    + '       meta-language language infer-attribute <observations.lino> --grammar <grammar.lino>\n'
    + '       meta-language language translate-attributes <archive> --source <id> --input <file> --target JavaScript|Rust --selections <selections.lino> [--input-binary]\n'
    + 'Use --link-cli-store or --input-link-cli-store in place of the corresponding binary option for native link-cli stores.\n'
    + 'Parse writes the complete extended archive; CST is the default. AST retains its original CST. Configuration is structured Links data.\n';
}
