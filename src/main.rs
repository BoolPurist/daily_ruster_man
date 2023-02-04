use daily_ruster_man::{
    cli::app_args::*,
    core::{
        list_queries, open_actions,
        app_options::AppOptions,
        date_models::{open_by::OpenByMonthInYear, units_validated::ValidatedYear},
        delete_actions::{self, DeletionResult},
    },
};
use daily_ruster_man::prelude::*;
use env_logger::Env;

fn main() {
    let cli_args = CliArgs::parse();

    init_logger(cli_args.args());
    set_up_env(cli_args.args());

    if let Err(error) = handle_commands(&cli_args) {
        exit_with_error(&error, cli_args.args());
    }
}

fn set_up_env(generell_args: &GenerellArgs) {
    if generell_args.debug() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
}

fn init_logger(generell_args: &GenerellArgs) {
    let mut logger_level = "warning";

    if cfg!(debug_assertions) || generell_args.debug() {
        logger_level = "debug";
    }

    let logger_env = Env::new().default_filter_or(logger_level);
    env_logger::Builder::from_env(logger_env).init()
}

fn handle_commands(args: &CliArgs) -> AppResult {
    let app_options = AppOptions::new(args);
    return match args.commands() {
        AppCommands::List(list_queries) => {
            let filter = list_queries.to_date_filter()?;
            let all = list_queries::fetch_all_daily_names(&filter, &app_options)?;
            let in_lines = all.join("\n");
            println!("{in_lines}");
            Ok(())
        }
        AppCommands::Edit(command_arg) => {
            let edit_query = command_arg.to_advance_now()?;
            open_actions::open_by_date(edit_query, &app_options)
        }
        AppCommands::MonthEdit(args) => {
            let month_in_year: OpenByMonthInYear = args.to_valid_ym_pair()?;
            open_actions::open_by_month_year(month_in_year, &app_options)
        }
        AppCommands::MonthList(args) => {
            let month_in_year = args.create_find_month_in_year()?;
            let monthly_names =
                list_queries::fetch_all_monthly_names(&month_in_year, &app_options)?;
            let lines = monthly_names.join("\n");
            println!("{lines}");
            Ok(())
        }
        AppCommands::YearList => {
            let all_yearlies = list_queries::fetch_yearly_names(&app_options)?;
            let lines = all_yearlies.join("\n");
            println!("{lines}");
            Ok(())
        }
        AppCommands::YearEdit { year } => {
            if let Some(year_given) = year {
                let year_given: ValidatedYear = (*year_given).try_into()?;

                open_actions::open_by_year(year_given, &app_options)?;
            } else {
                open_actions::open_by_current_year(&app_options)?;
            }
            Ok(())
        }
        AppCommands::Delete(to_delete) => {
            let validated = to_delete.date().to_advance_now()?;
            let has_delteted = delete_actions::delete_day_journal(validated, &app_options)?;

            report_deletion_result(has_delteted);
            Ok(())
        }
        AppCommands::DeleteMonth(to_delete) => {
            let validated = to_delete.month().to_valid_ym_pair()?;
            let has_delteted = delete_actions::delete_month_journal(validated, &app_options)?;

            report_deletion_result(has_delteted);
            Ok(())
        }
        AppCommands::DeleteYear(to_delete) => {
            let validated: ValidatedYear = (*to_delete.year()).try_into()?;
            let has_delteted = delete_actions::delete_year_journal(validated, &app_options)?;

            report_deletion_result(has_delteted);
            Ok(())
        }
    };

    fn report_deletion_result(has_delteted: DeletionResult) {
        match has_delteted {
            DeletionResult::NoJournalFound => {
                println!("There was no journal to be deleted.");
            }
            DeletionResult::NoConfirmation => {
                println!("Cancaled deletion of journal");
            }
            DeletionResult::Deleted => {
                println!("Journal was deleted");
            }
        }
    }
}

fn exit_with_error(error: &AppError, generell_args: &GenerellArgs) {
    if cfg!(debug_assertions) || generell_args.debug() {
        eprintln!("Error debug: {error:?}");
    } else {
        eprintln!("Error: {error}");
    }
    std::process::exit(1);
}
