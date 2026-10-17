JavaScript array bindings now retain ordered flat rest bindings and
undefined-only primitive defaults through shared generated decisions. The
initializer is read once, earlier defaults initialize before later bindings,
and rest creates a fresh array suffix. Nullable or structured defaults and
nested new patterns retain explicit unsupported diagnostics. Source-free Links
archives use the existing checked array and optional operations.
