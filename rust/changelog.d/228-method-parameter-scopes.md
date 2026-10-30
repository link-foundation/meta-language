Draft class/object method parameter scopes in both CST analyzers. Preserve
computed-name evaluation outside parameter environments and reuse shared
binding discovery for constructors, accessors, static/private/generator and
quoted/numeric methods. Add deferred original/renamed Node and snapshot controls;
retain full class runtime and implicit-binding obligations.
