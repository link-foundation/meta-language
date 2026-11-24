import { LinkNetwork } from '../js/src/index.js';

const source = '<h2>Условия</h2><p>Оплата от 200&nbsp;000 руб. &mdash; <b>ежемесячно</b>.</p><ul><li>Java&nbsp;17</li><li>Code&#32;review</li></ul>';
const projection = LinkNetwork.parse(source, 'HTML').visibleText();
console.log(projection.text);
const match = '200 000';
const start = projection.text.indexOf(match);
const range = projection.sourceRange(start, start + match.length);
const bytes = new TextEncoder().encode(source);
console.log('Original HTML:', new TextDecoder().decode(bytes.subarray(range.start, range.end)));
