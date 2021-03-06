use commands;
use prelude::*;

pub fn main() -> CliResult {
    let args = cli().get_matches();
    execute_subcommand(&args)
}

fn execute_subcommand(args: &ArgMatches) -> CliResult {
    let (cmd, args) = match args.subcommand() {
        (cmd, Some(args)) => (cmd, args),
        _ => {
            cli().print_help()?;
            return Ok(());
        }
    };

    if let Some(exec) = commands::builtin_exec(cmd) {
        return exec(args);
    }

    Ok(())
}

pub fn cli() -> App {
    App::new("devbox")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::AllowExternalSubcommands)
        .about("Control your local infrastructure and services")
        .subcommands(commands::builtins())
}
