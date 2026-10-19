Retry the observed Windows native linker lookup timeout at most three times,
retaining the original command and ten-second per-attempt bound. Both workflow
owners still validate the first resolved absolute linker path and fail on other
errors or exhausted retries. Retain diagnostics and complete preceding sources;
native Windows execution remains a CI check.
