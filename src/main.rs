use clap::{Arg, Command, arg};
use reqwest::blocking::get;
use scraper::Html;
use std::error::Error;
use teams::get_teams;

mod teams;
mod utils;

fn cli() -> Command {
	Command::new("hltv-api")
		.about("CLI for querying HLTV data")
		.subcommand_required(true)
		.arg_required_else_help(true)
		.subcommand(
			Command::new("teams").about("Query teams info").args([
				arg!([limit])
					.required(false)
					.help("Limit the number of teams to query")
					.default_value("20")
					.value_parser(clap::value_parser!(u32)),
				arg!(-n --name <NAME>)
					.required(false)
					.help("Name of the team to query"),
			]),
		)
		.subcommand(
			Command::new("matches")
				.about("Query recent or upcoming matches")
				.arg(
					Arg::new("upcoming")
						.short('u')
						.long("upcoming")
						.help("Only show upcoming matches")
						.action(clap::ArgAction::SetTrue),
				),
		)
		.subcommand(
			Command::new("events").about("Query HLTV events").arg(
				Arg::new("ongoing")
					.short('o')
					.long("ongoing")
					.help("Only show ongoing events")
					.action(clap::ArgAction::SetTrue),
			),
		)
}

fn main() {
	let args = cli().get_matches();

	match args.subcommand() {
		Some(("teams", sub_m)) => {
			let name = sub_m.get_one::<String>("name");
			let limit = sub_m.get_one::<u32>("limit");

			get_teams(name, limit);
		}

		Some(("matches", sub_m)) => {
			let upcoming = sub_m.get_flag("upcoming");
			if upcoming {
				println!("Showing upcoming matches");
			} else {
				println!("Showing recent matches");
			}
		}

		Some(("events", sub_m)) => {
			let ongoing = sub_m.get_flag("ongoing");
			if ongoing {
				println!("Showing ongoing events");
			} else {
				println!("Showing all events");
			}
		}

		_ => {
			println!("Unknown command");
		}
	}
}
