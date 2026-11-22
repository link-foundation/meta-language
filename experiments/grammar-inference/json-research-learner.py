"""Run the pinned, model-free XVada algorithm on an existing holdout batch."""
import base64
from contextlib import redirect_stdout
import hashlib
from importlib.metadata import version
import json
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
    except (ValueError, RecursionError):
        return False
    return True


def fail_on_model_call():
    # The pinned source constructs model clients while importing. This
    # deterministic configuration supplies no credentials or model proposals.
    class NoModel:
        def __getattr__(self, name):
            print(f"model call in deterministic XVada configuration: {name}", file=sys.stderr, flush=True)
            os._exit(76)

    model = ModuleType("openai")
    model.OpenAI = NoModel
    sys.modules["openai"] = model


def run(checkout, trace_path, request):
    input_limit = request["inputLimit"]
    query_limit = request["queryLimit"]
    memory = request["memoryBytes"]
    resource.setrlimit(resource.RLIMIT_AS, (memory, memory))
    fail_on_model_call()
    sys.path.insert(0, str(checkout))
    import config
    config.USE_LLM = False
    config.AI_LABEL = False
    from lark.exceptions import UnexpectedInput
    from oracle import ExternalOracle
    from search import approx_tokenize
    from start import build_start_grammar
    random.seed(request["seed"])

    def exhausted(reason):
        # Upstream contains broad exception handlers. Emit a separate
        # measurement outcome before exiting so they cannot turn exhaustion
        # into a negative answer to a membership query.
        print(json.dumps({"trainingLimit": reason, "larkVersion": version("lark"),
                          "trainingOracleQueries": oracle.real_calls,
                          "trainingOracleRequests": oracle.parse_calls}),
              file=sys.__stdout__, flush=True)
        os._exit(75)

    def train_deadline(signum, frame):
        exhausted("training time")

    class MembershipOracle(ExternalOracle):
        def _parse_internal(self, source):
            if self.real_calls >= query_limit:
                exhausted("training queries")
            spelling = source.encode("utf-8")
            if len(spelling) > input_limit:
                exhausted("membership query bytes")
            accepted = accept_json(source)
            self.real_calls += 1
            print(json.dumps({"query": self.real_calls, "sha256": hashlib.sha256(spelling).hexdigest(),
                              "bytes": len(spelling), "accepted": accepted, "source": source}), file=trace)
            return accepted

    with trace_path.open("x", encoding="utf-8", buffering=1) as trace:
        oracle = MembershipOracle("independent CPython JSON parser")
        signal.signal(signal.SIGALRM, train_deadline)
        signal.setitimer(signal.ITIMER_REAL, request["trainingSeconds"])
        started = time.monotonic()
        try:
            observations = []
            for source in request["positives"]:
                oracle.parse(source)
                observations.append(approx_tokenize(oracle, source))
            _, grammar = build_start_grammar(oracle, observations, (3, 10))
            parser = grammar.parser()
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
    for entry in request["holdout"]:
        data = base64.b64decode(entry["bytes"], validate=True)
        if len(data) > input_limit or hashlib.sha256(data).hexdigest() != entry["sha256"]:
            raise ValueError("holdout input differs from its byte budget or identity")
        signal.setitimer(signal.ITIMER_REAL, request["parseSeconds"])
        try:
            parser.parse(data.decode("utf-8"))
            outcome = {"learned": "accept"}
        except (UnicodeError, UnexpectedInput) as error:
            outcome = {"learned": "reject", "learnerDiagnostic": str(error)[:4096]}
        except (EvaluationDeadline, MemoryError, RecursionError) as error:
            outcome = {"learned": "resource limit", "learnerDiagnostic": type(error).__name__}
        finally:
            signal.setitimer(signal.ITIMER_REAL, 0)
        holdout[entry["path"]] = outcome
    return {"grammar": str(grammar), "grammarRules": len(grammar.rules), "larkVersion": version("lark"),
            "trainMilliseconds": train_milliseconds, "trainingOracleQueries": oracle.real_calls,
            "trainingOracleRequests": oracle.parse_calls, "parseMilliseconds": (time.monotonic() - started) * 1000,
            "peakLearnerProcessRssKiB": resource.getrusage(resource.RUSAGE_SELF).ru_maxrss, "holdout": holdout}


if __name__ == "__main__":
    checkout, trace_path = map(Path, sys.argv[1:])
    request = json.load(sys.stdin)
    with redirect_stdout(sys.stderr):
        result = run(checkout.resolve(), trace_path, request)
    print(json.dumps(result))
