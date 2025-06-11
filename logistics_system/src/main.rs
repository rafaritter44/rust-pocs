mod models;
mod pricing;

use crate::models::{ShipmentRequest, TransportType};
use crate::pricing::{load_config, calculate_price};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config("config.toml")?;

    let requests = vec![
        ShipmentRequest {
            volume_m3: 1.0,
            weight_kg: 5.0,
            transport: TransportType::Truck,
        },
        ShipmentRequest {
            volume_m3: 2.0,
            weight_kg: 10.0,
            transport: TransportType::Boat,
        },
        ShipmentRequest {
            volume_m3: 1.5,
            weight_kg: 7.5,
            transport: TransportType::Rail,
        },
    ];

    let distance_km = 1000.0;

    for request in requests {
        let price = calculate_price(&config, &request, distance_km);
        println!(
            "Transport: {:?}, Volume: {} m³, Weight: {} kg, Distance: {} km -> Price: ${:.2}",
            request.transport, request.volume_m3, request.weight_kg, distance_km, price
        );
    }

    Ok(())
}