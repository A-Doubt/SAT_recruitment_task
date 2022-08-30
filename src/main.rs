#[macro_use] extern crate rocket;

// Task stated "Dissel", but I think it was a typo, so changed to "Diesel".
#[get("/calculateDieselUsageForDistance")]
fn fuel_usage() -> String {
	String::from("fuel_usage endpoint")
}

#[get("/probabilityOfUnitInjectorFail")]
fn injector_fail() -> String {
	String::from("injector_fail endpoint")
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![fuel_usage, injector_fail])
}
