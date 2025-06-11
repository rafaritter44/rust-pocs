use crate::models::{PricingConfig, ShipmentRequest, TransportType, TransportPricing};
use std::fs;

pub fn load_config(path: &str) -> Result<PricingConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    toml::from_str(&content)
}

pub fn calculate_price(config: &PricingConfig, request: &ShipmentRequest, distance_km: f64) -> f64 {
    let transport_config: &TransportPricing = match request.transport {
        TransportType::Truck => &config.truck,
        TransportType::Boat => &config.boat,
        TransportType::Rail => &config.rail,
    };

    transport_config.base_rate_per_km * distance_km
        + transport_config.rate_per_kg * request.weight_kg
        + transport_config.rate_per_m3 * request.volume_m3
}