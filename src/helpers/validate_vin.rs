use regex::Regex;

pub fn validate_vin(vin: String) -> bool {
	let regex = Regex::new("^[a-zA-Z0-9]{11,17}$").unwrap();
	println!("{}", regex.is_match(&vin));
	
	regex.is_match(&vin)
}
