#[test]
fn a_sibling_that_does_not_translate_leaves_its_callers_carried() {
    // Same [1.0.0, 1.0.1) package profile as the JavaScript caller assertion.
    let version = env!("CARGO_PKG_VERSION")
        .split('.')
        .map(|part| part.parse::<u64>().expect("stable package release"))
        .collect::<Vec<_>>();
    assert!(
        version.as_slice() >= [1, 0, 0].as_slice() && version.as_slice() < [1, 0, 1].as_slice(),
        "uncovered sibling external-call package version: {}",
        env!("CARGO_PKG_VERSION")
    );
    let source = "/** @param {bigint} n @returns {boolean} */\nfunction isEven(n) {\n  return n === 0n ? true : isOdd(n - 1n);\n}\n\n/** @param {bigint} n @returns {boolean} */\nfunction isOdd(n) {\n  return n === 0n ? false : isEven(n - 1n);\n}\n\n/** @param {number} x @returns {number} */\nfunction opaque(x) {\n  return [x].map((y) => y)[0];\n}\n\n/** @param {number} x @returns {number} */\nfunction caller(x) {\n  return opaque(x) + 1;\n}\n";
    let translation = self_translate(source, "JavaScript", "Rust").expect("translates");
    // The complete binding scope checks mutually recursive siblings together.
    for needle in [
        "function isEven",
        "function isOdd",
        "function opaque",
        "function caller",
    ] {
        assert_eq!(
            status_of(&translation, source, needle),
            "translated",
            "{needle}"
        );
    }
    let caller = translation
        .items
        .iter()
        .find(|item| source[item.start..].starts_with("function caller"))
        .expect("the caller is an item");
    assert_eq!(caller.reason, None);
    let rust = emitted_rust(&translation);
    for definition in [
        "pub fn opaque(x: f64) -> Option<f64>",
        "pub fn caller(x: f64) -> f64",
        "opaque(x)",
        "pub fn is_even(",
        "pub fn is_odd(",
    ] {
        assert!(rust.contains(definition), "{definition}: {rust}");
    }
    // Actual externals use the module-local target in both normal emitters.
    assert!(!rust.contains("crate::opaque(x)"));
    let restored = self_translate(&translation.code, "Rust", "JavaScript").expect("restores");
    assert_eq!(restored.code, source);

    let unsupported = "/** @param {number} x @returns {number} */\nfunction opaqueRegex(x) { return /./u.test(String(x)) ? x : 0; }\n/** @param {number} x @returns {number} */\nfunction regexCaller(x) { return opaqueRegex(x) + 1; }\n";
    let carried =
        self_translate(unsupported, "JavaScript", "Rust").expect("carries unsupported siblings");
    for needle in ["function opaqueRegex", "function regexCaller"] {
        assert_eq!(
            status_of(&carried, unsupported, needle),
            "carried",
            "{needle}"
        );
    }
    let caller = carried
        .items
        .iter()
        .find(|item| unsupported[item.start..].starts_with("function regexCaller"))
        .expect("the unsupported caller remains an item");
    assert_eq!(caller.reason.as_deref(), Some("type"));
}
