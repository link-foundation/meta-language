"""Run the shared JSON Schema fixtures against pinned provider grammar engines."""

import argparse
import importlib.util
import json
import subprocess
import sys
from importlib.metadata import version
from pathlib import Path
from tempfile import TemporaryDirectory
from types import SimpleNamespace


class ByteTokenizer:
    eos_token_id = 256
    bos_token_id = None
    tokens = [bytes([byte]) for byte in range(256)] + [b"<end>"]
    special_token_ids = [eos_token_id]

    def __call__(self, source):
        return list(source)


def llguidance_tokenizer():
    from llguidance import LLTokenizer, TokenizerWrapper

    return LLTokenizer(TokenizerWrapper(ByteTokenizer()), slices=[])


def llguidance_decoder(grammar):
    from llguidance import LLMatcher

    tokenizer = llguidance_tokenizer()

    def accepts(sample):
        matcher = LLMatcher(tokenizer, grammar["lark"])
        if error := matcher.get_error():
            raise ValueError(error)
        tokens = list(sample.encode())
        return matcher.try_consume_tokens(tokens) == len(tokens) and matcher.is_accepting()

    return accepts


def outlines_decoder(grammar):
    import numpy as np
    from outlines.backends.llguidance import LLGuidanceBackend
    from outlines.types import CFG

    class ByteBackend(LLGuidanceBackend):
        def _create_llg_tokenizer(self, model):
            return llguidance_tokenizer()

    backend = ByteBackend(SimpleNamespace(tensor_library_name="numpy"))
    cfg = CFG(grammar["lark"])

    def accepts(sample):
        processor = backend.get_cfg_logits_processor(cfg.definition)
        prefix = []
        for token in [*sample.encode(), ByteTokenizer.eos_token_id]:
            ids = np.array([prefix], dtype=np.int64)
            logits = np.zeros((1, len(ByteTokenizer.tokens)), dtype=np.float32)
            if not np.isfinite(processor.process_logits(ids, logits)[0, token]):
                return False
            prefix.append(token)
        return True

    return accepts


def xgrammar_engine(package):
    import tvm_ffi

    # Load the wheel's native engine and generated bindings without a model stack.
    library = next(package.glob("libxgrammar_bindings.so"))
    native_module = tvm_ffi.load_module(str(library))
    spec = importlib.util.spec_from_file_location(
        "xgrammar_provider_binding", package / "tvm_ffi_binding/_ffi_api.py"
    )
    bindings = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = bindings
    spec.loader.exec_module(bindings)
    tokenizer = bindings.TokenizerInfo(
        ByteTokenizer.tokens, 0, None, [ByteTokenizer.eos_token_id], False
    )
    compiler = bindings.GrammarCompiler(tokenizer, 1, False, -1, False)

    def decoder(grammar):
        compiled = compiler.compile_grammar_from_strings(grammar["gbnf"], "root")

        def accepts(sample):
            # The native module owns the functions called by the generated bindings.
            _ = native_module
            matcher = bindings.GrammarMatcher(compiled, None, False, -1, None)
            return matcher.accept_string(sample, False) and matcher.is_completed()

        return accepts

    return decoder


def llama_decoder(executable, directory, grammar):
    grammar_path = directory / "document.gbnf"
    grammar_path.write_text(grammar["gbnf"], encoding="utf-8")
    sample_path = directory / "sample.json"

    def accepts(sample):
        sample_path.write_text(sample, encoding="utf-8")
        result = subprocess.run(
            [executable, grammar_path, sample_path], capture_output=True, encoding="utf-8", errors="replace"
        )
        if result.returncode != 0:
            raise ValueError(f"{result.stdout}\n{result.stderr}")
        first_line = result.stdout.splitlines()[0]
        if first_line == "Input string is valid according to the grammar.":
            return True
        if first_line == "Input string is invalid according to the grammar.":
            return False
        raise ValueError(result.stdout)

    return accepts


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("provider", choices=["llguidance", "outlines", "xgrammar", "llama.cpp"])
    parser.add_argument("fixtures", type=Path)
    parser.add_argument("--xgrammar-package", type=Path)
    parser.add_argument("--llama-validator", type=Path)
    options = parser.parse_args()
    if options.provider in {"llguidance", "outlines", "xgrammar"}:
        required = {"llguidance": "1.9.1", "outlines": "1.3.3", "xgrammar": "0.2.9"}
        observed = version(options.provider)
        if observed != required[options.provider]:
            parser.error(f"expected {options.provider} {required[options.provider]}, found {observed}")
        print(f"{options.provider} {observed}", flush=True)
    if options.provider == "xgrammar" and options.xgrammar_package is None:
        parser.error("xgrammar requires --xgrammar-package")
    if options.provider == "llama.cpp" and options.llama_validator is None:
        parser.error("llama.cpp requires the b12633 upstream validator via --llama-validator")

    count = 0
    failures = 0
    grammars = json.loads(options.fixtures.read_text(encoding="utf-8"))
    with TemporaryDirectory() as scratch:
        decoders = {"llguidance": llguidance_decoder, "outlines": outlines_decoder}
        if options.provider == "xgrammar":
            decoders["xgrammar"] = xgrammar_engine(options.xgrammar_package.resolve())
        if options.provider == "llama.cpp":
            decoders["llama.cpp"] = lambda grammar: llama_decoder(
                options.llama_validator.resolve(), Path(scratch), grammar
            )
        for index, grammar in enumerate(grammars, 1):
            decoder = decoders[options.provider](grammar)
            for sample, expected in grammar["cases"]:
                observed = decoder(sample)
                count += 1
                if observed != expected:
                    print(f"{index} {sample!r} expected {expected}, got {observed}", flush=True)
                    failures += 1
    print(f"{count - failures}/{count} fixtures passed")
    return int(failures > 0)


if __name__ == "__main__":
    sys.exit(main())
