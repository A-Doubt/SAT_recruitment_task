use chrono::Datelike;
use rand::Rng;
use regex::Regex;


#[macro_use] extern crate rocket;

pub struct Params {
	year_of_production: u32,
	distance: u32,
	fuel_usage_per_100_km: f32,
}

// Task stated "Dissel", but I think it was a typo, so changed to "Diesel".
#[allow(non_snake_case)] // for query params
#[get("/calculateDieselUsageForDistance?<distance>&<yearOfProduction>&<fuelUsagePer100KM>")]
fn fuel_usage(distance: u32, yearOfProduction: u32, fuelUsagePer100KM: f32) -> String {
	
	let fuel_consumption_params:Params = Params {
		year_of_production: yearOfProduction,
		distance,
		fuel_usage_per_100_km: fuelUsagePer100KM,
	};

	let params_valid:bool = validate_fuel_consumption_params(&fuel_consumption_params);

	if !params_valid {
		return String::from("The provided query parameters are not valid.");
	}

	let result: f32 = 
		calculate_fuel_usage(fuel_consumption_params.distance, fuel_consumption_params.fuel_usage_per_100_km);

	format!("{}", result)
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


fn validate_fuel_consumption_params(params: &Params) -> bool {
	let current_year: i32 = chrono::Utc::now().year();

	if params.distance <= 0 {
		return false;
	}
	if params.fuel_usage_per_100_km <= 0.0 {
		return false;
	}

	// some arbitrary year of production
	if params.year_of_production < 1900 || params.year_of_production > current_year as u32 {
		return false;
	}

	true
}

fn calculate_fuel_usage(distance: u32, fuel_usage_per_100_km: f32) -> f32 {
	let result: f32 = (fuel_usage_per_100_km * distance as f32) / 100 as f32;

	result
}

fn validate_vin(vin: String) -> bool {
	let regex = Regex::new("^[a-zA-Z0-9]{11,17}$").unwrap();
	println!("{}", regex.is_match(&vin));

	regex.is_match(&vin)
}
