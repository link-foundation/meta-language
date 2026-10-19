Task-scoped cleanup and wrapped checks support `--project-owned-only`, excluding
foreign Cargo target directories before discovery and byte accounting. The
policy disables container cleanup and low-disk escalation and records its scope
in the retained Links Notation receipt. Existing explicit target support and
all source, evidence and active-task checks remain.
