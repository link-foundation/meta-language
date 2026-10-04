#!/usr/bin/env python3
"""Lists no-message `assert!`/`debug_assert!` emptiness checks that Clippy 1.99's
`assert_is_empty` lint can flag, so they can be rewritten as comparison assertions.

    python3 experiments/issue-195-assert-is-empty-sites.py rust
"""
import os
import re
import sys

MACRO = re.compile(r'\b(debug_assert|assert)!\(')


def split_top_level(text):
    depth, parts, start, quote = 0, [], 0, None
    i = 0
    while i < len(text):
        c = text[i]
        if quote:
            if c == '\\':
                i += 1
            elif c == quote:
                quote = None
        elif c == '"':
            quote = c
        elif c in '([{':
            depth += 1
        elif c in ')]}':
            depth -= 1
        elif c == ',' and depth == 0:
            parts.append(text[start:i])
            start = i + 1
        i += 1
    parts.append(text[start:])
    return [p.strip() for p in parts if p.strip()]


def sites(root):
    for base, _, files in os.walk(root):
        if '/target' in base:
            continue
        for name in files:
            if not name.endswith('.rs'):
                continue
            path = os.path.join(base, name)
            source = open(path, encoding='utf8').read()
            for match in MACRO.finditer(source):
                depth, i = 1, match.end()
                while depth and i < len(source):
                    depth += {'(': 1, ')': -1}.get(source[i], 0)
                    i += 1
                args = split_top_level(source[match.end():i - 1])
                if len(args) != 1:
                    continue
                condition = ' '.join(args[0].split())
                negated = condition.startswith('!')
                body = condition[1:].strip() if negated else condition
                if not body.endswith('.is_empty()'):
                    continue
                receiver = body[: -len('.is_empty()')]
                line = source.count('\n', 0, match.start()) + 1
                yield path, line, match.group(1), negated, receiver, match.start(), i


if __name__ == '__main__':
    for path, line, macro, negated, receiver, *_ in sites(sys.argv[1] if len(sys.argv) > 1 else '.'):
        print(f'{path}:{line}\t{macro}\t{"ne" if negated else "eq"}\t{receiver}')
