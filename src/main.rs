mod commands;

fn main() {
    commands::run(std::env::args().collect());
}