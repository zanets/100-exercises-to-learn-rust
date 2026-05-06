// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(name: String, quan: u32, price: u32) -> Order {
        Order::check_name(&name);
        Order::check_quan(quan);
        Order::check_price(price);
        Order {
            product_name: name,
            quantity: quan,
            unit_price: price,
        }
    }

    fn check_name(inp: &String) {
        if inp == "" || inp.len() > 300 {
            panic!("fail");
        }
    }

    fn check_quan(inp: u32) {
        if inp <= 0 {
            panic!("fail");
        }
    }

    fn check_price(inp: u32) {
        if inp <= 0 {
            panic!("fail");
        }
    }

    pub fn set_product_name(&mut self, inp: String) {
        Order::check_name(&inp);
        self.product_name = inp;
    }

    pub fn set_quantity(&mut self, inp: u32) {
        Order::check_quan(inp);
        self.quantity = inp;
    }

    pub fn set_unit_price(&mut self, inp: u32) {
        Order::check_price(inp);
        self.unit_price = inp;
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}
