#[derive(Debug)]
pub struct StoreItem {
    pub name: &'static str,
    pub price: f64,
    pub quantity: f64,
}

impl StoreItem {
    pub fn new(name: &'static str, price: f64, quantity: f64) -> Self {
        StoreItem {
            name,
            price,
            quantity
        }
    }
}