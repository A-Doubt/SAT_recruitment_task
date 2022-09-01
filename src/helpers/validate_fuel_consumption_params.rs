use chrono::Datelike;
use super::params::Params;

pub fn validate_fuel_consumption_params(params: &Params) -> bool {
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