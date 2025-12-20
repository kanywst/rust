use glossary_modules::net;
use glossary_modules::cli;

fn main() {
    net::connect();
    cli::run();
}
