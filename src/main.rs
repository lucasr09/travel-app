mod db;
mod models;

use axum::{
    routing::{get, post},
    Json, Router,
    extract::State,
};
use models::*;
use rusqlite::Connection;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

type Db = Arc<Mutex<Connection>>;

#[tokio::main]
async fn main() {
    let conn = db::init_db().unwrap();
    let state: Db = Arc::new(Mutex::new(conn));

    let app = Router::new()
        .route("/countries", get(get_countries).post(import_countries))
        .route("/trips", get(get_trips).post(create_trip))
        .route("/stats", get(get_stats))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("http://localhost:3000");

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn get_countries(State(state): State<Db>) -> Json<Vec<Country>> {
    let conn = state.lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT id, name, continent, code FROM countries")
        .unwrap();

    let countries = stmt
        .query_map([], |row| {
            Ok(Country {
                id: row.get(0)?,
                name: row.get(1)?,
                continent: row.get(2)?,
                code: row.get(3)?,
            })
        })
        .unwrap()
        .map(|c| c.unwrap())
        .collect();

    Json(countries)
}

async fn import_countries(State(state): State<Db>) -> &'static str {
    let url = "https://restcountries.com/v3.1/all";

    let response = reqwest::get(url).await.unwrap();
    let data: Vec<ApiCountry> = response.json().await.unwrap();

    let conn = state.lock().unwrap();

    for country in data {
        let _ = conn.execute(
            "INSERT INTO countries (name, continent, code)
             VALUES (?1, ?2, ?3)",
            (
                country.name.common,
                country.region,
                country.cca2,
            ),
        );
    }

    "imported"
}

async fn get_trips(State(state): State<Db>) -> Json<Vec<TripWithCountry>> {
    let conn = state.lock().unwrap();

    let mut stmt = conn
        .prepare(
            "SELECT
                trips.id,
                countries.name,
                trips.start_date,
                trips.end_date,
                trips.trip_type,
                trips.rating,
                trips.notes
             FROM trips
             JOIN countries ON trips.country_id = countries.id"
        )
        .unwrap();

    let trips = stmt
        .query_map([], |row| {
            Ok(TripWithCountry {
                id: row.get(0)?,
                country: row.get(1)?,
                start_date: row.get(2)?,
                end_date: row.get(3)?,
                trip_type: row.get(4)?,
                rating: row.get(5)?,
                notes: row.get(6)?,
            })
        })
        .unwrap()
        .map(|t| t.unwrap())
        .collect();

    Json(trips)
}

async fn create_trip(
    State(state): State<Db>,
    Json(trip): Json<Trip>,
) -> &'static str {
    let conn = state.lock().unwrap();

    conn.execute(
        "INSERT INTO trips (country_id, start_date, end_date, trip_type, rating, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            trip.country_id,
            trip.start_date,
            trip.end_date,
            trip.trip_type,
            trip.rating,
            trip.notes,
        ),
    )
    .unwrap();

    "ok"
}

async fn get_stats(State(state): State<Db>) -> Json<Stats> {
    let conn = state.lock().unwrap();

    let total_trips: i32 = conn
        .query_row("SELECT COUNT(*) FROM trips", [], |r| r.get(0))
        .unwrap();

    let countries_visited: i32 = conn
        .query_row("SELECT COUNT(DISTINCT country_id) FROM trips", [], |r| r.get(0))
        .unwrap();

    let average_rating: f64 = conn
        .query_row("SELECT AVG(rating) FROM trips", [], |r| {
            let v: Option<f64> = r.get(0)?;
            Ok(v.unwrap_or(0.0))
        })
        .unwrap();

    let most_common_trip_type: String = conn
        .query_row(
            "SELECT trip_type, COUNT(*) as c
             FROM trips
             GROUP BY trip_type
             ORDER BY c DESC
             LIMIT 1",
            [],
            |r| r.get(0),
        )
        .unwrap_or("none".to_string());

    Json(Stats {
        countries_visited,
        total_trips,
        average_rating,
        most_common_trip_type,
    })
}