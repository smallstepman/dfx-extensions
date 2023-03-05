// mod commands;
// mod lib;

use clap::{Args, Parser};

#[derive(Parser)]
#[clap(name("nns"))]
pub struct CliOpts {
    /// Displays detailed information about operations. -vv will generate a very large number of messages and can affect performance.
    #[clap(long, short('v'), parse(from_occurrences), global(true))]
    verbose: u64,

    /// Suppresses informational messages. -qq limits to errors only; -qqqq disables them all.
    #[clap(long, short('q'), parse(from_occurrences), global(true))]
    quiet: u64,

    /// The logging mode to use. You can log to stderr, a file, or both.
    #[clap(long("log"), default_value("stderr"), possible_values(&["stderr", "tee", "file"]), global(true))]
    logmode: String,

    /// The file to log to, if logging to a file (see --logmode).
    #[clap(long, global(true))]
    logfile: Option<String>,

    /// The user identity to run this command as. It contains your principal as well as some things DFX associates with it like the wallet.
    #[clap(long, global(true))]
    identity: Option<String>,

    /// The effective canister id for provisional canister creation must be a canister id in the canister ranges of the subnet on which new canisters should be created.
    #[clap(long, global(true))]
    provisional_create_canister_effective_canister_id: Option<String>,
    // #[clap(subcommand)]
    // command: commands::SubCommand,
}

fn main() {
    println!("Hello, world!");
    let cli_opts = CliOpts::parse();
    // let command = cli_opts.command;
    // commands::exec(command);
    // let (verbose_level, log) = setup_logging(&cli_opts);
    // let identity = cli_opts.identity;
    // let effective_canister_id = cli_opts.provisional_create_canister_effective_canister_id;
    // let command = cli_opts.command;
    // let mut error_diagnosis: Diagnosis = NULL_DIAGNOSIS;
    // let result = match EnvironmentImpl::new() {
    //     Ok(env) => {
    //         maybe_redirect_dfx(env.get_version()).map_or((), |_| unreachable!());
    //         match EnvironmentImpl::new().map(|env| {
    //             env.with_logger(log)
    //                 .with_identity_override(identity)
    //                 .with_verbose_level(verbose_level)
    //                 .with_effective_canister_id(effective_canister_id)
    //         }) {
    //             Ok(env) => {
    //                 slog::trace!(
    //                     env.get_logger(),
    //                     "Trace mode enabled. Lots of logs coming up."
    //                 );
    //                 match commands::exec(&env, command) {
    //                     Err(e) => {
    //                         error_diagnosis = diagnose(&env, &e);
    //                         Err(e)
    //                     }
    //                     ok => ok,
    //                 }
    //             }
    //             Err(e) => Err(e),
    //         }
    //     }
    //     Err(e) => match command {
    //         commands::Command::Schema(_) => commands::exec_without_env(command),
    //         _ => Err(e),
    //     },
    // };
    // if let Err(err) = result {
    //     print_error_and_diagnosis(err, error_diagnosis);
    //     std::process::exit(255);
    // }
}
