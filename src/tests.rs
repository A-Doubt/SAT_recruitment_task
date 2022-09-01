use super::*;

mod test {
	use super::rocket;
	use rocket::http::Status;
	use rocket::local::blocking::Client;
	
	#[test]
	fn test_not_found() {
		let client = Client::tracked(rocket()).unwrap();
		let res = client.get("/fake").dispatch();
		
		assert_eq!(res.status(), Status::NotFound);
	}
	
	#[test]
	fn fuel_usage_ok() {
		let client = Client::tracked(rocket()).unwrap();
		let res = 
			client.get("/calculateDieselUsageForDistance?distance=100&yearOfProduction=2000&fuelUsagePer100KM=5.5")
			.dispatch();
		
		assert_eq!(res.status(), Status::Ok);
		assert_eq!(res.into_string().unwrap(), "5.5");
	}
	
	#[test]
	fn injector_fail_ok() {
		let client = Client::tracked(rocket()).unwrap();
		let res = 
			client.get("/probabilityOfUnitInjectorFail?vin=1234567890abcdef")
			.dispatch();
		
		assert_eq!(res.status(), Status::Ok);
	}
	
}
