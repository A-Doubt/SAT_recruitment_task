<h1>SAT recruitment task</h1>

## Getting started

### Installing Rust
This project uses **Rust** so in order to run it, you need to have Rust installed on your computer.
To install it follow the instruction on [Rust's website](https://www.rust-lang.org/).

Once  `rustup`  is installed, ensure the latest toolchain is installled by running the command:
`rustup default stable`

### Starting the application
To run this application clone this repository and then build and run it using *cargo run*
```
git clone https://github.com/A-Doubt/SAT_recruitment_task.git
cd SAT_recruitment_task
cargo run
```
</br>

### Using the application (API endpoints)
In order to use the application visit `http://localhost:8000`.
</br>
The REST API is described below.
</br>
#### Calculate fuel consumption
`GET /calculateDieselUsageForDistance?<distance>&<yearOfProduction>&<fuelUsagePer100KM>`
</br>
</br>
where:
distance = a positive integer (The unit of measurement is KM)  
yearOfProduction = a positive integer between 1900 and 2100  
fuelUsagePer100KM = a positive floating point number  
</br>
example: http://localhost:8000/calculateDieselUsageForDistance?distance=100&yearOfProduction=2000&fuelUsagePer100KM=6.5
</br>
</br>
Returns fuel consumption on specified distance.
</br>
#### Calculate the probability of unit injector failure
`GET /probabilityOfUnitInjectorFail?<vin>`
</br>
where:
vin = a string of 11-17 characters
example: http://localhost:8000/probabilityOfUnitInjectorFail?vin=1234567890abcdef\
</br>
Returns the probability of engine failure.
</br>
#### Testing
To run automated tests run
`cargo test`.
