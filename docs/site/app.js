// Drives the interactive "Links Notation playground" on the landing page.
//
// It loads the WebAssembly module built from the `web/` Rust crate (which wraps
// the `links-notation` crate) and renders the parsed structure live as the user
// types. Everything runs client-side; there are no network calls after load.

import init, { parse_links_notation } from "./demo/pkg/meta_language_web.js";

const EXAMPLES = {
  point: "(1: 1 1)",
  relation: "(3: 1 2)",
  identified: "(parent: child1 child2)",
  nested: "(tree: (left: 1 2) (right: 3 4))",
  multi: "(1: 1 1)\n(2: 2 2)\n(3: 1 2)",
};

const input = document.getElementById("demo-input");
const output = document.getElementById("demo-output");
const status = document.getElementById("demo-status");
const select = document.getElementById("example-select");

let ready = false;

function escapeHtml(text) {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

// Render a parsed LiNo tree (the JSON shape produced by the wasm module) as
// an indented, syntax-highlighted text view.
function renderTree(nodes, depth = 0) {
  const indent = "  ".repeat(depth);
  let out = "";
  for (const node of nodes) {
    if (node.kind === "ref") {
      out += `${indent}<span class="tree-ref">ref</span> ${escapeHtml(node.value)}\n`;
    } else {
      const id = node.id != null ? ` <span class="tree-id">#${escapeHtml(String(node.id))}</span>` : "";
      out += `${indent}<span class="tree-link">link</span>${id}\n`;
      if (node.values && node.values.length) {
        out += renderTree(node.values, depth + 1);
      }
    }
  }
  return out;
}

function run() {
  if (!ready) return;
  const text = input.value;
  let parsed;
  try {
    parsed = JSON.parse(parse_links_notation(text));
  } catch (err) {
    status.textContent = `Internal error: ${err}`;
    status.className = "demo-status error";
    return;
  }

  if (parsed.ok) {
    status.textContent = `Parsed ${parsed.count} statement${parsed.count === 1 ? "" : "s"}.`;
    status.className = "demo-status ok";
    const tree = renderTree(parsed.tree);
    output.innerHTML =
      tree +
      `\n<span class="tree-ref">— re-formatted —</span>\n` +
      escapeHtml(parsed.formatted);
  } else {
    status.textContent = parsed.error;
    status.className = "demo-status error";
    output.textContent = "";
  }
}

select.addEventListener("change", () => {
  input.value = EXAMPLES[select.value] ?? "";
  run();
});

input.addEventListener("input", run);

init()
  .then(() => {
    ready = true;
    status.textContent = "Ready.";
    status.className = "demo-status ok";
    run();
  })
  .catch((err) => {
    status.textContent =
      "Failed to load the WebAssembly demo. The static examples below still work.";
    status.className = "demo-status error";
    console.error(err);
  });
