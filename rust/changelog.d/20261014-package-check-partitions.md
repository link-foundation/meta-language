Partition JavaScript package validation into five required parallel groups under
the existing ten-minute job deadline. Preserve every original check, artifact,
cleanup step and publication dependency, with frozen step-conservation tests.

CI run 38431880453 reached the package job's ten-minute execution limit;
GitHub's job annotation confirms the automatic timeout. The fixture check was
interrupted nine seconds after starting, so its cancellation supplies no fixture
failure diagnosis. All fifteen JavaScript test partitions and all shared report
assertions passed. Six additional generated-rule and fixture checks remain
mandatory alongside the preserved original checks.
