use std::path::Path;
use std::process::Command;

const NODE_SMOKE_SCRIPT: &str = r#"
const fs = require("fs");
const source = fs.readFileSync(process.env.GRAMMAR_JS, "utf8");

function node(type, ...args) {
  return { type, args };
}

function grammar(spec) {
  return spec;
}

function seq(...args) {
  return node("seq", ...args);
}

function choice(...args) {
  return node("choice", ...args);
}

function repeat(rule) {
  return node("repeat", rule);
}

function repeat1(rule) {
  return node("repeat1", rule);
}

function optional(rule) {
  return node("optional", rule);
}

function token(rule) {
  return node("token", rule);
}

token.immediate = function immediate(rule) {
  return node("token.immediate", rule);
};

function field(name, rule) {
  return node("field", name, rule);
}

function alias(rule, name) {
  return node("alias", rule, name);
}

function prec(value, rule) {
  return node("prec", value, rule);
}

prec.left = function left(value, rule) {
  return node("prec.left", value, rule);
};

prec.right = function right(value, rule) {
  return node("prec.right", value, rule);
};

prec.dynamic = function dynamic(value, rule) {
  return node("prec.dynamic", value, rule);
};

function blank() {
  return node("blank");
}

const module = { exports: {} };
const exports = module.exports;
const evaluate = new Function(
  "module",
  "exports",
  "grammar",
  "seq",
  "choice",
  "repeat",
  "repeat1",
  "optional",
  "token",
  "field",
  "alias",
  "prec",
  "blank",
  source,
);

evaluate(
  module,
  exports,
  grammar,
  seq,
  choice,
  repeat,
  repeat1,
  optional,
  token,
  field,
  alias,
  prec,
  blank,
);

const spec = module.exports;
if (!spec || typeof spec.name !== "string") {
  throw new Error("grammar.js did not export a named grammar object");
}
if (!spec.rules || typeof spec.rules !== "object") {
  throw new Error("grammar.js did not export a rules object");
}

const $ = new Proxy({}, {
  get(_target, property) {
    return node("symbol", String(property));
  },
});

if (spec.inline !== undefined && typeof spec.inline !== "function") {
  throw new Error("inline must be a function when present");
}
if (typeof spec.inline === "function") {
  spec.inline($);
}

for (const [name, rule] of Object.entries(spec.rules)) {
  if (typeof rule !== "function") {
    throw new Error(`rule ${name} is not a function`);
  }
  if (rule($) === undefined) {
    throw new Error(`rule ${name} returned undefined`);
  }
}
"#;

#[test]
fn committed_tree_sitter_grammar_js_fixtures_are_valid_javascript_modules() {
    if Command::new("node").arg("--version").output().is_err() {
        eprintln!("skipping tree-sitter grammar.js smoke test because node is not available");
        return;
    }

    smoke_fixture("sum.grammar.js");
    smoke_fixture("covering.grammar.js");
}

fn smoke_fixture(file_name: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("grammar")
        .join("tree-sitter")
        .join(file_name);
    let output = Command::new("node")
        .arg("-e")
        .arg(NODE_SMOKE_SCRIPT)
        .env("GRAMMAR_JS", &path)
        .output()
        .expect("node smoke test runs");

    assert!(
        output.status.success(),
        "node smoke test failed for {}\nstdout:\n{}\nstderr:\n{}",
        path.display(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
