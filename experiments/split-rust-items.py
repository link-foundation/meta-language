#!/usr/bin/env python3
"""Moves top-level items of a rustfmt-formatted Rust file into another file.

    python3 experiments/split-rust-items.py SRC DEST NAME [NAME ...]

Each NAME is a top-level `fn`, `const`, `static`, `struct`, `enum`, `type`
or the type of an `impl` block. An item takes the doc comments, comments and
attributes right above it and ends at the first later line that starts with a
closing `}`, `]` or `)` (or at its own line when that ends with `;`). A
private item becomes `pub(super)`. DEST is appended to; the module
declaration and the `use` lines are left to the caller.
"""
import re
import sys

ITEM = re.compile(
    r"^(?P<vis>pub(?:\([^)]*\))? )?(?P<kw>(?:const |async |unsafe )*fn|const|static|struct|enum|type|trait|impl(?:<[^>]*>)?(?: [\w:<>', ]+ for)?) (?P<name>\w+)"
)


def item_spans(lines, name):
    spans = []
    for index, line in enumerate(lines):
        match = ITEM.match(line)
        if not match or match.group("name") != name:
            continue
        start = index
        while start > 0 and re.match(r"^(///|//|#\[)", lines[start - 1]):
            start -= 1
        end = index
        if not (line.rstrip().endswith(";") and "{" not in line):
            end = index + 1
            while not re.match(r"^(}.*|\];|\);)$", lines[end]):
                end += 1
        spans.append((start, end, match))
    if not spans:
        raise SystemExit(f"no top-level item {name}")
    return spans


def main():
    source, dest, *names = sys.argv[1:]
    lines = open(source, encoding="utf8").read().split("\n")
    spans = sorted((span for name in names for span in item_spans(lines, name)), key=lambda span: span[0])
    moved = []
    for start, end, match in spans:
        chunk = lines[start : end + 1]
        if match.group("vis") is None and not match.group("kw").startswith("impl"):
            item_line = next(i for i, text in enumerate(chunk) if ITEM.match(text))
            chunk[item_line] = "pub(super) " + chunk[item_line]
        moved.append("\n".join(chunk))
    for start, end, _ in reversed(spans):
        del lines[start : end + 1]
        # Drop the blank line the item leaves behind.
        if start < len(lines) and start > 0 and lines[start] == "" and lines[start - 1] == "":
            del lines[start]
    open(source, "w", encoding="utf8").write("\n".join(lines))
    with open(dest, "a", encoding="utf8") as out:
        out.write("\n\n".join(moved) + "\n")


main()
