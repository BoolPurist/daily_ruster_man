#![allow(clippy::uninlined_format_args)]
use std::process::ExitCode;

use date_validation_types::ValidatedYear;
use daily_ruster_man::{
    cli::app_args::*,
    core::{
        list_queries,
        open_actions::{self, OpenResult},
        process_handling::RealProcessExecuter,
        app_options::AppOptions,
        date_models::open_by::OpenByMonthInYear,
        delete_actions::{self, DeletionResult},
    },
};
use daily_ruster_man::prelude::*;
use env_logger::Env;

fn main() -> ExitCode {
    let cli_args = CliArgs::parse();

    init_logger(cli_args.args());
    set_up_env(cli_args.args());

    if let Err(error) = handle_commands(&cli_args) {
        print_error(&error, cli_args.args());
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
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
        AppCommands::Edit(command_arg) => {
            let edit_query = command_arg.command().to_advance_now()?;
            let open_result = open_actions::open_by_date(
                &RealProcessExecuter::default(),
                edit_query,
                &app_options,
                command_arg.option(),
            );

            report_open_result(open_result)
        }
        AppCommands::MonthEdit(args) => {
            let month_in_year: OpenByMonthInYear = args.command().to_valid_ym_pair()?;
            let open_result = open_actions::open_by_month_year(
                &RealProcessExecuter::default(),
                month_in_year,
                &app_options,
                args.option(),
            );

            report_open_result(open_result)
        }
        AppCommands::YearEdit(year_edit) => {
            let open_result = if let Some(year_given) = year_edit.year() {
                let year_given: ValidatedYear = year_given.try_into()?;

                open_actions::open_by_year(
                    &RealProcessExecuter::default(),
                    year_given,
                    &app_options,
                    year_edit.option(),
                )
            } else {
                open_actions::open_by_current_year(
                    &RealProcessExecuter::default(),
                    &app_options,
                    year_edit.option(),
                )
            };

            report_open_result(open_result)
        }
        AppCommands::Delete(to_delete) => {
            let validated = to_delete.date().to_advance_now()?;
            let has_delteted = delete_actions::delete_day_journal(
                validated,
                to_delete.common_arg(),
                &app_options,
            )?;

            report_deletion_result(has_delteted);
            Ok(())
        }
        AppCommands::DeleteMonth(to_delete) => {
            let validated = to_delete.month().to_valid_ym_pair()?;
            let has_delteted = delete_actions::delete_month_journal(
                validated,
                to_delete.common_arg(),
                &app_options,
            )?;

            report_deletion_result(has_delteted);
            Ok(())
        }
        AppCommands::DeleteYear(to_delete) => {
            let validated: ValidatedYear = (*to_delete.year()).try_into()?;
            let has_delteted = delete_actions::delete_year_journal(
                validated,
                to_delete.common_arg(),
                &app_options,
            )?;

            report_deletion_result(has_delteted);
            Ok(())
        }
    };

    fn report_open_result(from_open_action: OpenResult) -> AppResult {
        let may_content = from_open_action?;
        if let Some(content) = may_content {
            println!("{}", content);
        }

        Ok(())
    }

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

fn print_error(error: &AppError, generell_args: &GenerellArgs) {
    if cfg!(debug_assertions) || generell_args.debug() {
        eprintln!("Error debug: {error:#?}");
    } else {
        eprintln!("Error: {error}");
    }
}
