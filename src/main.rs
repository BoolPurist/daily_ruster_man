use daily_ruster_man::{
    cli::app_args::*,
    core::{list_queries, open_actions},
};
use daily_ruster_man::prelude::*;
use env_logger::Env;

fn main() {
    init_logger();

    let args = CliArgs::parse();
    if let Err(error) = handle_commands(&args) {
        exit_with_error(&error);
    }
}

fn init_logger() {
    let mut logger_level = "warning";

    if cfg!(debug_assertions) {
        logger_level = "debug";
    }

    let logger_env = Env::new().default_filter_or(logger_level);
    env_logger::Builder::from_env(logger_env).init()
}

fn handle_commands(args: &CliArgs) -> AppResult {
    return match args {
        CliArgs::List(_) => {
            let all = list_queries::fetch_all_daily_names().join("\n");
            println!("{all}");
            Ok(())
        }
        CliArgs::Edit(command_arg) => {
            let edit_query = command_arg.get_date_query()?;
            match edit_query {
                EditByDate::None => open_actions::open_today(),
                EditByDate::Range(past_or_future) => {
                    open_actions::open_by_future_past_range(&past_or_future)
                }
                EditByDate::DayOfYear(day_of_year) => {
                    open_actions::open_by_day_of_year(&day_of_year)
                }
                EditByDate::DayMonthYear(day_month_year) => {
                    open_actions::open_by_year_month_day(&day_month_year)
                }
            }
        }
    };
}

fn exit_with_error(error: &AppError) {
    if cfg!(debug_assertions) {
        eprintln!("Error debug: {error:?}");
    } else {
        eprintln!("Error: {error}");
    }
    std::process::exit(1);
}
