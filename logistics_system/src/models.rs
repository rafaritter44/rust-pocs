use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransportPricing {
    pub base_rate_per_km: f64,
    pub rate_per_kg: f64,
    pub rate_per_m3: f64,
}

#[derive(Debug, Deserialize)]
pub struct PricingConfig {
    pub truck: TransportPricing,
    pub boat: TransportPricing,
    pub rail: TransportPricing,
}

#[derive(Debug)]
pub enum TransportType {
    Truck,
    Boat,
    Rail,
}

#[derive(Debug)]
pub struct ShipmentRequest {
    pub volume_m3: f64,
    pub weight_kg: f64,
    pub transport: TransportType,
}