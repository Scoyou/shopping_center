#[derive(Debug)]
pub struct CartItem {
    pub name: &'static str,
    pub price: f64,
    pub quantity: f64,
    pub total_price: f64,
}

impl CartItem {
    pub fn new(name: &'static str, price: f64, quantity: f64, total_price: f64) -> Self {
        CartItem {
            name,
            price,
            quantity,
            total_price,
        }
    }
}