use rand::Rng;
use regex::Regex;

#[macro_use] extern crate rocket;

// Task stated "Dissel", but I think it was a typo, so changed to "Diesel".
#[get("/calculateDieselUsageForDistance")]
fn fuel_usage() -> String {
	String::from("fuel_usage endpoint")
}

#[get("/probabilityOfUnitInjectorFail?<vin>")]
fn injector_fail(vin: String) -> String {
	let is_vin_valid: bool = validate_vin(vin);

	if !is_vin_valid {
		return String::from("The VIN provided is not valid. VIN should be 11-17 characters long and contain only letters and numbers.");
	}

	let random_num: i32 = rand::thread_rng().gen_range(0..=100);
	let fail_probability: f32 = (random_num as f32) / 100.0;

	format!("{}", fail_probability.to_string())
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![fuel_usage, injector_fail])
}


fn validate_vin(vin: String) -> bool {
	let regex = Regex::new("^[a-zA-Z0-9]{11,17}$").unwrap();
	println!("{}", regex.is_match(&vin));

	regex.is_match(&vin)
}
