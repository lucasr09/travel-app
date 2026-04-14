# Travel App API

A full-stack travel tracking system built with Rust.  
It provides a backend API for managing trips, countries, and travel statistics, with a future frontend built using Leptos.

---

## Overview

This project tracks personal travel history and generates insights based on visited countries and trip patterns.

It is designed as a learning project to explore:

- REST API design in Rust
- Database-driven applications
- Data aggregation and statistics
- Full-stack Rust architecture

---

## Features

### Backend (Axum)
- REST API built with Axum
- SQLite database integration
- CRUD operations for trips
- Country management system
- Aggregated travel statistics
- JOIN queries between trips and countries

### Data Insights
- Total trips tracking
- Countries visited count
- Average trip rating
- Most common trip type
- Structured travel history per country

### External Data
- Country data import via external REST API

---

## Tech Stack

- Rust
- Axum
- SQLite
- Tokio (async runtime)
- Serde (serialization)
- Reqwest (HTTP client)
- Leptos (frontend, in progress)

---

## API Endpoints

### Countries
- `GET /countries` – list all countries
- `POST /countries/import` – import countries from external API

### Trips
- `GET /trips` – list all trips
- `POST /trips` – create a new trip

### Statistics
- `GET /stats` – travel insights and aggregates

---

## Project Structure
