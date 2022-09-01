use rand::Rng;

mod helpers { 
	pub mod validate_vin; 
	pub mod calculate_fuel_usage;
	pub mod params;
	pub mod validate_fuel_consumption_params;
}

#[cfg(test)] mod tests;

#[macro_use] extern crate rocket;

// Task stated "Dissel", but I think it was a typo, so changed to "Diesel".
#[allow(non_snake_case)] // for query params
#[get("/calculateDieselUsageForDistance?<distance>&<yearOfProduction>&<fuelUsagePer100KM>")]
fn fuel_usage(distance: u32, yearOfProduction: u32, fuelUsagePer100KM: f32) -> String {
	
	let fuel_consumption_params:helpers::params::Params = helpers::params::Params {
		year_of_production: yearOfProduction,
		distance,
		fuel_usage_per_100_km: fuelUsagePer100KM,
	};
	
	let params_valid:bool = helpers::validate_fuel_consumption_params::validate_fuel_consumption_params(&fuel_consumption_params);
	
	if !params_valid {
		return String::from("The provided query parameters are invalid.");
	}
	
	let result: f32 = 
	helpers::calculate_fuel_usage::calculate_fuel_usage(fuel_consumption_params.distance, fuel_consumption_params.fuel_usage_per_100_km);
	
	format!("{}", result)
}


#[get("/probabilityOfUnitInjectorFail?<vin>")]
fn injector_fail(vin: String) -> String {
	let is_vin_valid: bool = helpers::validate_vin::validate_vin(vin);
	
	if !is_vin_valid {
		return String::from("The VIN provided is not valid. VIN should be 11-17 characters long and contain only letters and numbers.");
	}
	
	let random_num: i32 = rand::thread_rng().gen_range(0..=100);
	let fail_probability: f32 = (random_num as f32) / 100.0;
	
	format!("{}", fail_probability.to_string())
}

#[catch(404)]
fn e_404() -> String {
	format!
	("Resource not found.
	The available endpoints are:
	/calculateDieselUsageForDistance?distance=<distance>&yearOfProduction=<yearOfProduction>&fuelUsagePer100KM=fuelUsagePer100KM
	where:
	distance = a positive integer
	yearOfProduction = a positive integer between 1900 and 2100
	fuelUsagePer100KM = a positive floating point number
	
	/probabilityOfUnitInjectorFail?vin=<vin>
	where:
	vin = a string of 11-17 characters
	")
}

#[launch]
fn rocket() -> _ {
	rocket::build()
	.mount("/", routes![fuel_usage, injector_fail])
	.register("/", catchers![e_404])
}
