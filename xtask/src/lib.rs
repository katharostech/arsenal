use argh::FromArgs;

mod commands;

#[derive(FromArgs, PartialEq, Debug)]
/// Run an cargo xtask
struct Args {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommand {
    Hello(commands::hello::HelloSubcommand),
}

pub fn run() {
    let args: Args = argh::from_env();

    if let Err(e) = match args.subcommand {
        Subcommand::Hello(args) => commands::hello::run(args),
    } {
        eprintln!("{:#?}", e);
        std::process::exit(1);
    }
}
