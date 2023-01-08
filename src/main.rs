use daily_ruster_man::cli::app_args::*;
use daily_ruster_man::{core, AppError, AppResult};
fn main() {
    let args = CliArgs::parse();
    if let Err(error) = handle_commands(&args) {
        exit_with_error(&error);
    }
}

fn handle_commands(args: &CliArgs) -> AppResult {
    return match args {
        CliArgs::List(_) => {
            for daily in core::fetch_all_daily_names() {
                println!("{daily}");
            }

            Ok(())
        }
        CliArgs::Edit(command_arg) => {
            let edit_query = command_arg.get_date_query()?;
            match edit_query {
                EditByDate::None => core::open_today(),
                EditByDate::Range(past_or_future) => {
                    core::open_by_future_past_range(&past_or_future)
                }
                EditByDate::DayOfYear(day_of_year) => core::open_by_day_of_year(&day_of_year),
                EditByDate::DayMonthYear(day_month_year) => {
                    core::open_by_year_month_day(&day_month_year)
                }
            }
        }
    };
}

fn exit_with_error(error: &AppError) {
    if cfg!(debug_assertions) {
        eprintln!("Error debug: {:?}", error);
    } else {
        eprintln!("Error: {}", error);
    }
    std::process::exit(1);
}
