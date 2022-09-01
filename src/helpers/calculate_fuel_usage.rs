pub fn calculate_fuel_usage(distance: u32, fuel_usage_per_100_km: f32) -> f32 {
	let result: f32 = (fuel_usage_per_100_km * distance as f32) / 100 as f32;
	
	result
}
