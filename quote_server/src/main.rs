mod cli;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    cli::Cli::execute();
}
