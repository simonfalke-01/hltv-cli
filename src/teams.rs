use crate::utils::type_of;

pub fn get_teams(name: Option<&String>, limit: Option<&u32>) {
	if !name.is_none() {
		println!("Querying team: {}", name.unwrap());
	} else {
		println!("Listing all teams");
	}

	if !limit.is_none() {
		println!("Limiting to {} teams", limit.unwrap());
		println!("{}", type_of(limit.unwrap()))
	}
}
