// Polaris — Cold War Grand Strategy Game
// Entry point. Game loop orchestration lives here.

mod simulation;
mod rendering;
mod ui;
mod audio;
mod network;

fn main() {
    println!("Polaris — Cold War Grand Strategy");
    println!("v{}", env!("CARGO_PKG_VERSION"));
}
