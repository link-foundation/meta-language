// Prints which candidate malformed OCaml interfaces tree-sitter-ocaml recovers
// with an ERROR or MISSING node, to pick the inventory's recoverySource.
import { LinkNetwork, LinkType } from '../js/src/index.js';

const candidates = process.argv.slice(2).length ? process.argv.slice(2) : [
  'val f : int ->\n', 'val f : int -> \n', 'val : int\n', 'val f int\n', 'type t = {\n', 'val f : (int\n', 'module M : sig\n', 'val f :\n', 'type = int\n',
];
for (const source of candidates) {
  const network = LinkNetwork.parse(source, 'OCaml Interface');
  const recovered = network.links().some((link) => {
    const { linkType, flags } = link.metadata();
    return linkType === LinkType.Syntax && (flags.isError || flags.isMissing);
  });
  console.log(JSON.stringify(source), recovered ? 'recovers with error' : 'clean');
}
