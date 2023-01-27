use daily_ruster_man::{
    cli::app_args::*,
    core::{
        list_queries, open_actions,
        date_models::{open_by::OpenByMonthInYear, units_validated::ValidatedYear},
    },
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
    match args {
        CliArgs::List(list_queries) => {
            let filter = list_queries.to_date_filter()?;
            let all = list_queries::fetch_all_daily_names(&filter)?;
            let in_lines = all.join("\n");
            println!("{in_lines}");
            Ok(())
        }
        CliArgs::Edit(command_arg) => {
            let edit_query = command_arg.to_advance_now()?;
            open_actions::open_by_date(edit_query)
        }
        CliArgs::MonthEdit(args) => {
            let month_in_year: OpenByMonthInYear = args.to_valid_ym_pair()?;
            open_actions::open_by_month_year(month_in_year)
        }
        CliArgs::MonthList(args) => {
            let month_in_year = args.create_find_month_in_year()?;
            let monthly_names = list_queries::fetch_all_monthly_names(&month_in_year)?;
            let lines = monthly_names.join("\n");
            println!("{lines}");
            Ok(())
        }
        CliArgs::YearList => {
            let all_yearlies = list_queries::fetch_yearly_names()?;
            let lines = all_yearlies.join("\n");
            println!("{lines}");
            Ok(())
        }
        CliArgs::YearEdit { year } => {
            if let Some(year_given) = year {
                let year_given: ValidatedYear = (*year_given).try_into()?;

                open_actions::open_by_year(year_given)?;
            } else {
                open_actions::open_by_current_year()?;
            }
            Ok(())
        }
    }
}

fn exit_with_error(error: &AppError) {
    if cfg!(debug_assertions) {
        eprintln!("Error debug: {error:?}");
    } else {
        eprintln!("Error: {error}");
    }
    std::process::exit(1);
}
