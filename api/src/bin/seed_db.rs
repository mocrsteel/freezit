#[cfg(not(feature = "seed"))]
use colored::Colorize;

#[cfg(feature = "seed")]
use api::seed;

#[cfg(feature = "seed")]
fn main() {
    seed();
}

#[cfg(not(feature = "seed"))]
fn main() {
    println!(
        "{} This binary is only intended for development. It requires to be run with feature {}: {}.",
        "Error:".bold().red(),
        "seed".bold().on_bright_black(),
        "cargo run --bin seed_db --features seed"
            .bold()
            .on_bright_black()
    )
}
