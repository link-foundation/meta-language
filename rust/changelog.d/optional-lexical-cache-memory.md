Optional fixed lexical matching caches now release their retained entries when
required parser work needs the memory. Both runtimes keep the existing memory
limit and mandatory memo accounting. This preserves the exact native Zig tree
for the pinned diagnostic-comment input without increasing its budget.
