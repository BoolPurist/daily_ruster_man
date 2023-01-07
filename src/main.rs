use daily_ruster_man::cli::app_args::*;
use daily_ruster_man::core;
fn main() {
    let args = CliArgs::parse();
    handle_commands(&args);
}

fn handle_commands(args: &CliArgs) {
    match args {
        CliArgs::List(_) => {
            for daily in core::fetch_all_daily_names() {
                println!("{daily}");
            }
        }
        CliArgs::Edit(_) => {
            core::open_today();
        }
    }
}
