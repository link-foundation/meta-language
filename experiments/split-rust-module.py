#!/usr/bin/env python3
"""Move line ranges of a Rust source file into child modules.

Usage: split-rust-module.py SPEC.json

SPEC: {"file": "rust/src/translation/check.rs",
       "parts": [{"name": "exprs", "doc": "Expression checking.",
                  "ranges": [[400, 900]]}]}

Each range is [first item line, next item line): 1-based line numbers of
the item keywords in the original file.  The end is exclusive and backs off
past the next item's doc comments; use the closing brace of an `impl` block
or the file length plus one to end a range there.
A range inside an `impl` block is wrapped in a copy of that block's
header.  Private items and methods that move gain `pub(super)` so the
parent and sibling parts can still reach them; each child starts with
`use super::*;`, which also brings in the parent's imports.
"""

import json
import os
import re
import sys

ITEM = re.compile(
    r"^(?P<indent>\s*)(?P<kw>(?:const |async |unsafe )*fn |struct |enum |const |static |type |trait )"
)


def enclosing_impl(lines, start):
    """Header line index of the impl block that contains line index start."""
    depth_header = None
    for i in range(start, -1, -1):
        line = lines[i]
        if line.startswith("}"):
            return None
        if re.match(r"^impl\b|^impl<", line):
            depth_header = i
            break
    return depth_header


def widen(lines, start):
    """Include the doc comments and attributes that precede an item."""
    indent = len(lines[start]) - len(lines[start].lstrip())
    i = start
    while i > 0:
        prev = lines[i - 1]
        stripped = prev.strip()
        prev_indent = len(prev) - len(prev.lstrip())
        if prev_indent == indent and (
            stripped.startswith("///") or stripped.startswith("#[") or stripped.startswith("//")
        ):
            i -= 1
            continue
        break
    return i


def publicise(line, indent):
    match = ITEM.match(line)
    if not match or len(match.group("indent")) != indent:
        return line
    return f"{match.group('indent')}pub(super) {line[len(match.group('indent')):]}"


def main():
    spec = json.load(open(sys.argv[1]))
    path = spec["file"]
    lines = open(path).read().split("\n")
    moved = set()
    stem = os.path.splitext(path)[0]
    os.makedirs(stem, exist_ok=True)
    declarations = []
    for part in spec["parts"]:
        body = []
        for first, last in part["ranges"]:
            start = widen(lines, first - 1)
            end = widen(lines, last - 1) - 1 if last <= len(lines) else len(lines) - 1
            header = enclosing_impl(lines, start)
            indent = 4 if header is not None else 0
            chunk = [publicise(l, indent) for l in lines[start : end + 1]]
            if header is not None:
                chunk = [lines[header]] + chunk + ["}"]
            if body:
                body.append("")
            body.extend(chunk)
            moved.update(range(start, end + 1))
        text = "//! " + part["doc"] + "\n\nuse super::*;\n\n" + "\n".join(body) + "\n"
        open(os.path.join(stem, part["name"] + ".rs"), "w").write(text)
        exports = part.get("exports", "")
        declarations.append(f"mod {part['name']};")
        if exports:
            declarations.append(f"{exports} self::{part['name']}::*;")
    kept = [l for i, l in enumerate(lines) if i not in moved]
    # Collapse blank runs left behind by removed items.
    out = []
    for line in kept:
        if line == "" and out and out[-1] == "":
            continue
        out.append(line)
    # Insert the module declarations after the last top-level `use`.
    last_use = 0
    i = 0
    while i < len(out):
        if out[i].startswith("use ") or out[i].startswith("pub use "):
            j = i
            while not out[j].rstrip().endswith(";"):
                j += 1
            last_use = j + 1
            i = j
        elif re.match(r"^(pub )?(fn|struct|enum|impl|const|static|mod|trait)\b", out[i]):
            break
        i += 1
    out[last_use:last_use] = [""] + declarations
    while out and out[-1] == "":
        out.pop()
    open(path, "w").write("\n".join(out) + "\n")


if __name__ == "__main__":
    main()
