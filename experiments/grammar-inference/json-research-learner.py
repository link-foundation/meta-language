"""Run the pinned model-free learners on the same JSON holdout batch."""
import base64
from contextlib import redirect_stdout
import hashlib
from importlib.metadata import version
import json
import multiprocessing
import os
from pathlib import Path
import random
import resource
import signal
import sys
import time
from types import ModuleType


def reject_constant(value):
    raise ValueError(value)


def accept_json(source):
    try:
        json.loads(source, parse_constant=reject_constant)
    except ValueError:
        return False
    return True


def fail_on_model_call():
    # The pinned source constructs model clients while importing. This
    # deterministic configuration supplies no credentials or model proposals.
    class NoModel:
        def __getattr__(self, name):
            print(f"model call in deterministic configuration: {name}", file=sys.stderr, flush=True)
            os._exit(76)

    model = ModuleType("openai")
    model.OpenAI = NoModel
    sys.modules["openai"] = model


def natgi_unused_pretty_print():
    # NatGI imports this debug printer by an unavailable module name. All
    # printer calls in the pinned source are commented out.
    def unexpected_debug_call(*args, **kwargs):
        print("unexpected debug printer call in pinned NatGI run", file=sys.stderr, flush=True)
        os._exit(76)

    debug = ModuleType("PrettyPrint")
    debug.PrettyPrintTree = unexpected_debug_call
    sys.modules["PrettyPrint"] = debug


def select_training(strategy, evaluate_query):
    if strategy == "crucio":
        from crucio.config import Config
        from crucio.oracle.string import CachedStringOracle
        from learn import run_crucio3

        Config.realInfer = True
        Config.log = False

        class CrucioOracle(CachedStringOracle):
            def __init__(self):
                super().__init__()
                self.parse_calls = 0

            def parse(self, source):
                self.parse_calls += 1
                return super().parse(source)

            def _parse(self, source):
                return evaluate_query(source)

        def train(oracle, positives):
            lexical, grammar, _ = run_crucio3(oracle, positives)
            tokenizer = lexical.getTokenizer()
            parser = grammar.getParser()

            def parse(source):
                try:
                    tokens = tokenizer.tokenize(source)
                except Exception as error:
                    # The pinned lexer signals a missing match, ambiguous
                    # token, or unrecognized token with a bare Exception.
                    lexical_refusal = not error.args or str(error).startswith(("Multiple tokens found:", "Unrecognized token "))
                    if type(error) is not Exception or not lexical_refusal:
                        raise
                    return {"learned": "reject", "learnerDiagnostic": str(error) or "no learned lexical match"}
                parser.parse(" ".join(token.type for token in tokens))
                return {"learned": "accept"}

            return grammar.asLark(), len(grammar.getRules()), parse

        return CrucioOracle(), train

    from oracle import ExternalOracle
    from search import approx_tokenize
    from start import build_start_grammar

    class MembershipOracle(ExternalOracle):
        def _parse_internal(self, source):
            accepted = evaluate_query(source)
            self.real_calls += 1
            return accepted

    def train(oracle, positives):
        observations = [approx_tokenize(oracle, source) for source in positives]
        grammars = build_start_grammar(oracle, observations, (3, 10))
        expected_grammars = 4 if strategy == "natgi" else 2
        # NatGI's bracket-guided branch still runs without model proposals.
        if len(grammars) != expected_grammars or grammars[1] is None:
            raise ValueError("the pinned learner did not return its model-free HDD grammar")
        grammar = grammars[1]
        parser = grammar.parser()

        def parse(source):
            parser.parse(source)
            return {"learned": "accept"}

        return str(grammar), len(grammar.rules), parse

    return MembershipOracle("independent CPython JSON parser"), train


def run(checkout, trace_path, request):
    input_limit = request["inputLimit"]
    query_limit = request["queryLimit"]
    memory = request["memoryBytes"]
    resource.setrlimit(resource.RLIMIT_AS, (memory, memory))
    fail_on_model_call()
    strategy = request["strategy"]
    if strategy == "natgi":
        natgi_unused_pretty_print()
    elif strategy not in ("xvada", "crucio"):
        raise ValueError("unknown pinned research learner")
    sys.path.insert(0, str(checkout))
    if strategy != "crucio":
        import config
        config.USE_LLM = False
        config.AI_LABEL = False
        if strategy == "natgi":
            config.TREEVADA = False
            config.HDD = True
    from lark.exceptions import UnexpectedInput
    random.seed(request["seed"])
    queries = 0

    def exhausted(reason):
        # Upstream contains broad exception handlers. Emit a separate
        # measurement outcome before exiting so they cannot turn exhaustion
        # into a negative answer to a membership query.
        for child in multiprocessing.active_children():
            child.kill()
        print(json.dumps({"trainingLimit": reason, "larkVersion": version("lark"),
                          "trainingOracleQueries": queries,
                          "trainingOracleRequests": oracle.parse_calls}),
              file=sys.__stdout__, flush=True)
        os._exit(75)

    def train_deadline(signum, frame):
        exhausted("training time")

    def evaluate_query(source):
        nonlocal queries
        if queries >= query_limit:
            exhausted("training queries")
        spelling = source.encode("utf-8")
        if len(spelling) > input_limit:
            exhausted("membership query bytes")
        try:
            accepted = accept_json(source)
        except RecursionError:
            exhausted("membership oracle nesting")
        queries += 1
        print(json.dumps({"query": queries, "sha256": hashlib.sha256(spelling).hexdigest(),
                          "bytes": len(spelling), "accepted": accepted, "source": source}), file=trace)
        return accepted

    with trace_path.open("x", encoding="utf-8", buffering=1) as trace:
        oracle, train = select_training(strategy, evaluate_query)
        signal.signal(signal.SIGALRM, train_deadline)
        signal.setitimer(signal.ITIMER_REAL, request["trainingSeconds"])
        started = time.monotonic()
        try:
            for source in request["positives"]:
                if not oracle.parse(source):
                    raise ValueError("the oracle refused a positive training source")
            grammar, grammar_rules, parse = train(oracle, request["positives"])
        except MemoryError:
            exhausted("training address space")
        finally:
            signal.setitimer(signal.ITIMER_REAL, 0)
        train_milliseconds = (time.monotonic() - started) * 1000

    class EvaluationDeadline(BaseException):
        pass

    def evaluation_deadline(signum, frame):
        raise EvaluationDeadline()

    signal.signal(signal.SIGALRM, evaluation_deadline)
    holdout = {}
    started = time.monotonic()
    deadline = started + request["evaluationSeconds"]
    for entry in request["holdout"]:
        data = base64.b64decode(entry["bytes"], validate=True)
        if len(data) > input_limit or hashlib.sha256(data).hexdigest() != entry["sha256"]:
            raise ValueError("holdout input differs from its byte budget or identity")
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            holdout[entry["path"]] = {"learned": "resource limit", "learnerDiagnostic": "holdout time budget", "attempted": False}
            continue
        signal.setitimer(signal.ITIMER_REAL, min(request["parseSeconds"], remaining))
        try:
            outcome = parse(data.decode("utf-8"))
        except (UnicodeError, UnexpectedInput) as error:
            outcome = {"learned": "reject", "learnerDiagnostic": str(error)[:4096]}
        except (EvaluationDeadline, MemoryError, RecursionError) as error:
            outcome = {"learned": "resource limit", "learnerDiagnostic": type(error).__name__}
        finally:
            signal.setitimer(signal.ITIMER_REAL, 0)
        holdout[entry["path"]] = outcome
    return {"grammar": grammar, "grammarRules": grammar_rules, "larkVersion": version("lark"),
            "trainMilliseconds": train_milliseconds, "trainingOracleQueries": queries,
            "trainingOracleRequests": oracle.parse_calls, "parseMilliseconds": (time.monotonic() - started) * 1000,
            "peakLearnerProcessRssKiB": resource.getrusage(resource.RUSAGE_SELF).ru_maxrss, "holdout": holdout}


if __name__ == "__main__":
    checkout, trace_path = map(Path, sys.argv[1:])
    request = json.load(sys.stdin)
    with redirect_stdout(sys.stderr):
        result = run(checkout.resolve(), trace_path, request)
    print(json.dumps(result))
