Reuse one lazily encoded UTF-16 source when tokenizing regular-expression
literals in both runtimes. Modules with many literals no longer copy each
remaining source suffix, and Unicode token offsets remain absolute.
