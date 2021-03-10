use crate::cart_item::CartItem;
use crate::store_item::StoreItem;

#[derive(Debug)]
pub struct Cart {
    pub items: Vec<CartItem>,
    pub total: f64
}

impl Cart {
    pub fn add_to_cart(&mut self, item: &StoreItem, quantity: f64) -> &Vec<CartItem> {
        let item = CartItem::new(item.name, item.price, quantity, item.price * quantity);
        self.items.push(item);

        &self.items
    }

    pub fn remove_from_cart(&mut self, item: &CartItem, quantity: f64) -> &Vec<CartItem> {
        let item_iter = self.items.iter();

        for i in item_iter {
            if i.name == item.name {
                println!("{}", i.name)
            }
        };

        &self.items
    }
}