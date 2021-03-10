mod shopper;
pub use crate::shopper::Shopper;
mod store_item;
pub use crate::store_item::StoreItem;
mod cart_item;
pub use crate::cart_item::CartItem;
mod purchased_item;
pub use crate::purchased_item::PurchasedItem;
mod cart;
pub use crate::cart::Cart;

fn main() {
    let mut apples = StoreItem::new("apples", 3.50, 20.0);
    let oranges = StoreItem::new("oranges", 1.25, 10.0);

    let store_items = vec![&apples, &oranges];

    list_store_items(store_items);

    let cart = Cart {
        items: vec![],
        total: 0.0,
    };

    let mut shopper = Shopper {
        name: String::from("Scott"),
        cart,
        purchased_items: vec![],
        money: 100.17,
    };

    let quantity = 2.0;
    
    add_item_to_cart(&mut shopper, &mut apples, quantity);

    println!("cart: {:?}", shopper.view_cart());
    println!("money: ${}", shopper.check_wallet());

    println!("{}", apples.quantity)
}

fn list_store_items(store_items: Vec<&StoreItem>)  {
    println!("~~~ Items ~~~");
    for item in store_items {
        println!("{}, ${}, {}", item.name, item.price, item.quantity)
    };
}

fn add_item_to_cart(shopper: &mut Shopper, item: &mut StoreItem, quantity: f64) {
    item.quantity -= quantity;
    shopper.cart.add_to_cart(item, quantity);
}

