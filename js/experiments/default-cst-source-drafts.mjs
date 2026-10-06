// Draft inventory sources for the default-CST evidence: each positive source
// exercises comments, several constructs, non-ASCII text and multiple lines,
// and each recovery source must make the grammar report ERROR or MISSING.
export const DRAFTS = {
  JavaScript: {
    grammar: 'javascript',
    source: `// Greeting module — café
import { readFile } from 'node:fs';
/* block comment */
export const greet = (name = "wörld") => \`Hello, \${name}!\`;
class Counter { #count = 0; increment() { return ++this.#count; } }
`,
    recovery: 'const = 1;\nif (ready { go(); }\n',
  },
  Rust: {
    grammar: 'rust',
    source: `//! Crate docs — café
/// A point.
#[derive(Debug)]
struct Point { x: i32, y: i32 }
fn main() {
    let p = Point { x: 1, y: 2 }; /* note */
    println!("{} é", p.x + p.y);
}
`,
    recovery: 'fn main( {\n    let x = ;\n}\n',
  },
  Lean: {
    grammar: 'lean',
    source: `-- Identity — λ
/-- Returns its argument. -/
def ident (x : Nat) : Nat := x
theorem ident_eq (n : Nat) : ident n = n := rfl
#eval ident 1
`,
    recovery: 'def f (x : Nat : Nat := x\n',
  },
  Rocq: {
    grammar: 'rocq',
    source: `(* Identity — λ *)
Definition ident (x : nat) : nat := x.
Lemma ident_eq : forall n, ident n = n.
Proof. intros n. reflexivity. Qed.
`,
    recovery: 'Definition f (x : nat : nat := x.\n',
  },
  Python: {
    grammar: 'python',
    source: `# Greeting — café
def greet(name: str = "wörld") -> str:
    """Return a greeting."""
    return f"Hello, {name}!"


class Point:
    x: int = 0
`,
    recovery: 'def f(x:\n    return x +\n',
  },
  C: {
    grammar: 'c',
    source: `/* Point — é */
#include <stdio.h>
// entry
struct point { int x; int y; };
int main(void) { printf("%d é\\n", 1 + 2); return 0; }
`,
    recovery: 'int main(void) { return 0\n',
  },
  'C++': {
    grammar: 'cpp',
    source: `// café
#include <vector>
namespace demo { template <typename T> T id(T x) { return x; } }
int main() { std::vector<int> v{1, 2}; return demo::id(0); } /* end */
`,
    recovery: 'int main() { return 0\n',
  },
  'C#': {
    grammar: 'csharp',
    source: `// café
using System;
/// <summary>Demo</summary>
class C { static void Main() { var s = $"é {1 + 2}"; Console.WriteLine(s); } }
`,
    recovery: 'class C { void M( { } }\n',
  },
  Java: {
    grammar: 'java',
    source: `// café
import java.util.List;
/** Main class. */
class Main { public static void main(String[] args) { String s = "é"; System.out.println(s + 1); } }
`,
    recovery: 'class Main { void f( { } }\n',
  },
  TypeScript: {
    grammar: 'typescript',
    source: `// café
interface Point { x: number; y?: string }
export function id<T>(value: T): T { return value; } /* é */
type Pair = [number, string];
`,
    recovery: 'const value: = 1;\n',
  },
  TSX: {
    grammar: 'tsx',
    source: `// café
const view = <div className="é">{items.map((i: number) => <span key={i}>{i}</span>)}</div>;
`,
    recovery: 'const view = <div>;\n',
  },
  'Visual Basic': {
    grammar: 'vb',
    source: `' café
Module Program
    Sub Main()
        Dim s As String = "é"
        Console.WriteLine(s)
    End Sub
End Module
`,
    recovery: 'Module Program\n    Sub Main(\nEnd Module\n',
  },
  'Delphi/Object Pascal': {
    grammar: 'pascal',
    source: `{ café }
program Demo;
// comment
var x: Integer;
begin
  x := 1 + 2;
  WriteLn('é', x);
end.
`,
    recovery: 'program Demo; begin x := ; end.\n',
  },
  Go: {
    grammar: 'go',
    source: `// Package main — café
package main

import "fmt"

/* entry */
func main() { s := "é"; fmt.Println(s, 1+2) }
`,
    recovery: 'package main\nfunc main( {}\n',
  },
  R: {
    grammar: 'r',
    source: `# café
square <- function(x) x^2
values <- c(1, 2, 3)
print(sapply(values, square)) # é
`,
    recovery: 'value <- c(1, 2\n',
  },
  Ruby: {
    grammar: 'ruby',
    source: `# café
class Greeter
  def greet(name = "é")
    "Hello, #{name}!"
  end
end
`,
    recovery: 'def f(x\n  x\nend\n',
  },
  PHP: {
    grammar: 'php',
    source: `<p>café</p>
<?php
// comment
function f($x) { return "é{$x}"; }
echo f(1);
`,
    recovery: '<?php function f($x { return $x; }\n',
  },
  Swift: {
    grammar: 'swift',
    source: `// café
struct Point { let x: Int }
func f(_ x: Int) -> Int { x } /* é */
let s = "é \\(f(1))"
`,
    recovery: 'func f(_ x: Int -> Int { x }\n',
  },
  Kotlin: {
    grammar: 'kotlin',
    source: `// café
data class Point(val x: Int)
fun f(x: Int): Int = x /* é */
val s = "é \${f(1)}"
`,
    recovery: 'fun f(x: Int: Int = x\n',
  },
  Scala: {
    grammar: 'scala',
    source: `// café
case class Point(x: Int)
object Demo { def f(x: Int): Int = x /* é */ ; val s = s"é \${f(1)}" }
`,
    recovery: 'object Demo { def f(x: Int: Int = x }\n',
  },
  Lua: {
    grammar: 'lua',
    source: `-- café
local function f(x) return x .. "é" end
--[[ block ]]
local t = { a = 1, [2] = "b" }
`,
    recovery: 'local function f(x return x end\n',
  },
  Perl: {
    grammar: 'perl',
    source: `# café
sub f { my ($x) = @_; return "é$x"; }
my @list = (1, 2);
print f($list[0]), "\\n";
`,
    recovery: 'sub f { my ($x = @_; }\n',
  },
  'sql-ansi': {
    grammar: 'sql',
    source: `-- café
SELECT u.id, COUNT(*) AS total
FROM users AS u /* é */
WHERE u.name = 'é'
GROUP BY u.id;
`,
    recovery: 'SELECT id FROM users WHERE;\n',
  },
  'sql-postgres': {
    grammar: 'sql',
    source: `-- café
SELECT id::text, data->>'name' FROM users
WHERE name ILIKE 'é%' AND id = ANY(ARRAY[1, 2]);
`,
    recovery: 'SELECT (id::text FROM users;\n',
  },
  'sql-mysql': {
    grammar: 'sql',
    source: `-- café
CREATE TEMPORARY TABLE t (id INT AUTO_INCREMENT PRIMARY KEY) ENGINE=InnoDB;
SELECT \`id\`, IFNULL(name, 'é') FROM \`users\` LIMIT 10 OFFSET 1;
`,
    recovery: 'SELECT `id FROM users;\n',
  },
  'sql-sqlite': {
    grammar: 'sql',
    source: `-- café
CREATE TABLE t (id INTEGER PRIMARY KEY, name TEXT DEFAULT 'é');
SELECT IFNULL(name, 'x') FROM t LIMIT 5;
`,
    recovery: 'CREATE TABLE t (id INTEGER,;\n',
  },
  'sql-server': {
    grammar: 'sql',
    source: `-- café
SELECT id INTO #tmp FROM users WHERE name = N'é';
SELECT GETDATE() AS now, ISNULL(name, 'x') FROM users;
`,
    recovery: 'SELECT ISNULL(name FROM users;\n',
  },
  'sql-oracle': {
    grammar: 'sql',
    source: `-- café
SELECT sysdate, NVL(name, 'é') FROM dual WHERE ROWNUM <= 5;
`,
    recovery: 'SELECT NVL(name FROM dual;\n',
  },
  'sql-bigquery': {
    grammar: 'sql',
    source: `-- café
SELECT STRUCT(1 AS a) AS s, ARRAY_AGG(name) FROM \`users\` GROUP BY s;
`,
    recovery: 'SELECT STRUCT(1 AS FROM users;\n',
  },
  'sql-snowflake': {
    grammar: 'sql',
    source: `-- café
SELECT PARSE_JSON('{}') AS v, IFF(id > 1, 'a', 'é') FROM users;
`,
    recovery: "SELECT IFF(id > 1, 'a' FROM users;\n",
  },
  HTML: {
    grammar: 'html',
    source: `<!-- café -->
<p class="x" style="color: red">Héllo</p>
<script>const value = 1;</script>
<style>.x { color: blue; }</style>
`,
    recovery: '<p class="x>text</p>\n',
  },
  CSS: {
    grammar: 'css',
    source: `/* café */
@media (min-width: 10px) { .x > a:hover { color: #fff; content: "é"; } }
`,
    recovery: '.x { color: red\n',
  },
  JSON: {
    grammar: 'json',
    source: '{"name": "café", "values": [1, 2.5, true, null],\n "nested": {"é": false}}\n',
    recovery: '{"value": 1,\n',
  },
  YAML: {
    grammar: 'yaml',
    source: `# café
name: "é"
items:
  - 1
  - two
`,
    recovery: 'name: [1, 2\n',
  },
  TOML: {
    grammar: 'toml',
    source: `# café
title = "é"
[owner]
dob = 1979-05-27T07:32:00Z
list = [1, 2]
`,
    recovery: 'title = \n[owner\n',
  },
  XML: {
    grammar: 'xml',
    source: `<?xml version="1.0"?>
<!-- café -->
<note lang="fr"><to>é</to><![CDATA[x < y]]></note>
`,
    recovery: '<note><to>text</note>\n',
  },
  DTD: {
    grammar: 'dtd',
    source: `<!-- café -->
<!ELEMENT note (to, body)>
<!ATTLIST note lang CDATA #IMPLIED>
`,
    recovery: '<!ELEMENT note (to, body>\n',
  },
  INI: {
    grammar: 'ini',
    source: '; café\n[owner]\nname = é\n',
    recovery: '[owner\nname = value\n',
  },
  'Protocol Buffers': {
    grammar: 'proto',
    source: `// café
syntax = "proto3";
message Item { string name = 1; repeated int32 ids = 2; }
`,
    recovery: 'syntax = "proto3";\nmessage Item { string name = ; }\n',
  },
  GraphQL: {
    grammar: 'graphql',
    source: '# café\ntype Item { id: ID!\n  name(lang: String = "é"): String }\n',
    recovery: 'type Item { id: }\n',
  },
  CSV: {
    grammar: 'csv',
    source: 'name,value\n"café, é",1\n"quote ""x""",2\n',
    recovery: 'name,value\n"open,1\n',
  },
  JSON5: {
    grammar: 'json5',
    source: "// café\n{value: 1, 'é': [0x10, +Infinity,],}\n",
    recovery: '{value: }\n',
  },
  Markdown: {
    grammar: 'markdown',
    source: `# Title *x*

> Quote with \`code\`
> and [link](https://example.com) *em
> ph* é.

| a | b |
|---|---|
| *c* | d |

\`\`\`js
const value = 1;
\`\`\`

<div>café</div>

- item é
`,
    // The Markdown grammars accept every text; U+0000 is their only error.
    recovery: '# Title\n\nText \u0000 here.\n',
  },
  DOCX: {
    grammar: 'xml',
    source: '<w:document xmlns:w="urn:w"><w:body><w:p><w:r><w:t>café</w:t></w:r></w:p></w:body></w:document>\n',
    recovery: '<w:document><w:body></w:document>\n',
  },
};
