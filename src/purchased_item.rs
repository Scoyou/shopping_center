#[derive(Debug)]
pub struct PurchasedItem {
    pub name: &'static str,
    pub price: f64,
    pub quantity: u32,
    pub total_price: f64,
}

impl PurchasedItem {
    pub fn new(name: &'static str, price: f64, quantity: u32, total_price: f64) -> Self {
        PurchasedItem {
            name,
            price,
            quantity,
            total_price
        }
    }
}