
// use dominio::prelude::*;
use dominio::value_objects::{Currency, Quantity, Symbol, Instrument, Price, Spread};
use rust_decimal::Decimal;

fn main() {
    // 🔸 Escenario: evaluar si existe oportunidad entre spot y perp.

    // 1 BTC de tamaño
    let qty = Quantity::new(Decimal::new(1, 0));

    // Precio spot y precio perp (ficticios)
    let spot_price = Price::new(Decimal::new(30_000, 0), Currency::USD);
    let perp_price = Price::new(Decimal::new(30_100, 0), Currency::USD);

    // Costos totales
    let spot_value = spot_price.total(qty);
    let perp_value = perp_price.total(qty);

    println!("Comprar {:?} BTC spot @ {:?}  → {:?}", qty, spot_price, spot_value);
    println!("Vender  {:?} BTC perp @ {:?}  → {:?}", qty, perp_price, perp_value);

    // Spread y BPS
    let spread = Spread::between(spot_price, perp_price);
    let bps = spread.basis_points(spot_price);

    println!("Spread absoluto: {:?} ({:.2} bps)", spread, bps);

    // Instrument example
    let btc_usd_okx = Instrument::new(Symbol::new("BTC", "USD"), Some("OKX"));
    println!("Instrumento comercializado: {}", btc_usd_okx.full_code());

    // Lógica mínima de oportunidad
    if spread.amount > Decimal::new(50, 0) {
        println!("🚀 Posible oportunidad de arbitraje: spread > $50");
    } else {
        println!("Sin oportunidad: spread insuficiente");
    }
}
