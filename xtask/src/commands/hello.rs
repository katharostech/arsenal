use argh::FromArgs;
use xshell::cmd;

#[derive(FromArgs, PartialEq, Debug)]
/// Just an example command
#[argh(subcommand, name = "hello")]
pub struct HelloSubcommand {
    /// whether to say goodbye instead of hello
    #[argh(switch, short = 'g')]
    goodbye: bool,
}

pub fn run(args: HelloSubcommand) -> eyre::Result<()> {
    let greeting = if !args.goodbye { "Hello!" } else { "Goodbye!" };

    cmd!("echo {greeting}").run()?;

    Ok(())
}
