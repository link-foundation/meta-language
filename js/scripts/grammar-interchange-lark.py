"""Independent Lark compiler/parser, used only by the CI interchange oracle."""
import json
import sys

import lark
from lark import Lark, Tree, UnexpectedInput


def byte_offset(source, offset):
    return len(source[:offset].encode("utf-8"))


def observe(grammar, start, samples):
    parser = Lark(grammar, start=start, parser="earley", lexer="dynamic_complete",
                  keep_all_tokens=True, propagate_positions=True, maybe_placeholders=False)

    def project(tree, source):
        return {"kind": str(tree.data), "start": byte_offset(source, tree.meta.start_pos),
                "end": byte_offset(source, tree.meta.end_pos),
                "text": source[tree.meta.start_pos:tree.meta.end_pos],
                "children": [project(child, source) for child in tree.children if isinstance(child, Tree)]}

    output = []
    for source in samples:
        try:
            output.append({"accepted": True, "tree": project(parser.parse(source), source)})
        except UnexpectedInput as error:
            position = getattr(error, "pos_in_stream", len(source))
            if position < 0:
                position = len(source)
            output.append({"accepted": False, "offset": byte_offset(source, position),
                           "reason": type(error).__name__, "syntaxRejected": True})
    return output


payload = json.load(sys.stdin)
try:
    result = {"version": lark.__version__,
              "observations": observe(payload["source"], payload["start"], payload["samples"]),
              "mutation": observe(payload["changed"], payload["start"], [payload["witness"]])[0]}
except Exception as error:
    result = {"version": lark.__version__, "error": str(error)}
json.dump(result, sys.stdout, ensure_ascii=False)
sys.stdout.write("\n")
