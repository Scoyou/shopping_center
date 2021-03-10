pub use crate::store_item::StoreItem;
pub use crate::cart_item::CartItem;
pub use crate::purchased_item::PurchasedItem;
pub use crate::cart::Cart;

pub struct Shopper {
    pub name: String,
    pub cart: Cart,
    pub purchased_items: Vec<PurchasedItem>,
    pub money: f64,
}

impl Shopper {
    pub fn view_purchased_items(&self) -> &Vec<PurchasedItem> {
        &self.purchased_items
    }
    pub fn view_cart(&self) -> &Cart {
        &self.cart
    }
    pub fn check_wallet(&self) -> &f64 {
        &self.money
    }
}