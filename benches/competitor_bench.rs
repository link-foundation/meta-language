use meta_language::benchmark::{render_competitor_report, run_competitor_suite};
use meta_language::SampleConfig;

fn main() {
    let config = SampleConfig {
        seed: 0xE3C0_0017,
        count: 64,
        max_depth: 8,
        repeat_cap: 3,
    };
    let report = run_competitor_suite(&config).expect("competitor benchmark suite loads");
    println!("{}", render_competitor_report(&report));
    assert!(
        report.failures.is_empty(),
        "competitor benchmark suite failed"
    );
}
