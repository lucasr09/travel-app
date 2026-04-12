use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub continent: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Trip {
    pub id: Option<i32>,
    pub country_id: i32,
    pub start_date: String,
    pub end_date: String,
    pub trip_type: String,
    pub rating: i32,
    pub notes: String,
}

#[derive(Serialize)]
pub struct TripWithCountry {
    pub id: i32,
    pub country: String,
    pub start_date: String,
    pub end_date: String,
    pub trip_type: String,
    pub rating: i32,
    pub notes: String,
}

#[derive(Serialize)]
pub struct Stats {
    pub countries_visited: i32,
    pub total_trips: i32,
    pub average_rating: f64,
    pub most_common_trip_type: String,
}

#[derive(Deserialize)]
pub struct ApiCountry {
    pub name: ApiName,
    pub cca2: String,
    pub region: String,
}

#[derive(Deserialize)]
pub struct ApiName {
    pub common: String,
}