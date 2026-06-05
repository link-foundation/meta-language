use clap::{Parser, Subcommand};

use meta_language::{LinkNetwork, ParseConfiguration};

#[derive(Parser, Debug)]
#[command(
    name = "meta-language",
    about = "Build and verify self-describing links networks"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Print the built-in self-description roots.
    Describe,
    /// Parse text into a lossless token network and verify it is clean.
    Verify {
        /// Language label for the parsed region.
        #[arg(long)]
        language: String,
        /// Source text to parse.
        #[arg(long)]
        text: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Describe => describe(),
        Command::Verify { language, text } => verify(&language, &text),
    }
}

fn describe() {
    let network = LinkNetwork::self_describing();
    let roots = [
        "link",
        "reference",
        "relation link",
        "language",
        "grammar",
        "type",
        "concept",
        "point",
    ];
    println!("self-description roots: {}", roots.join(", "));
    println!("links: {}", network.len());
}

fn verify(language: &str, text: &str) {
    let network = LinkNetwork::parse_lossless_text(text, language, ParseConfiguration::default());
    let report = network.verify_full_match(None);

    if report.is_clean() {
        println!("clean");
    } else {
        for issue in report.issues() {
            eprintln!("{}: {:?}", issue.link_id(), issue.kind());
        }
        std::process::exit(1);
    }
}
