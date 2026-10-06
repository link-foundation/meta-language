import { LinkNetwork } from '../js/src/index.js';
const src = 'papa: loves mama  # note\n  (son "quoted \\" x")\n';
const n = LinkNetwork.parse(src, 'LiNo');
console.log(JSON.stringify(n.reconstructText()) === JSON.stringify(src));
const tokens = n.links().filter((l) => l.metadata?.term === 'mama' || l.term === 'mama');
console.log(tokens.length, tokens.map(t=>JSON.stringify(t).slice(0,300)));
const t = tokens.find(Boolean);
if (t) { n.setTerm(t.id, 'papa2'); console.log(JSON.stringify(n.reconstructText())); }
const lino = n.toLino(); console.log(LinkNetwork.fromLino(lino).toLino() === lino);
