use crate::impls::{
    types::{FoodId, OrderId, RestaurantId,},
};
use ink::prelude::{
    string::String,
};
#[openbrush::trait_definition]
pub trait CustomerService {

    #[ink(message)]
    fn add_customer(
        &mut self,
        customer_name: String,
        customer_address: String,
        phone_number: String
    );

    #[ink(message, payable)]
    fn submit_order(
        &mut self, 
        food_id: FoodId,
        restaurant_id: RestaurantId,
        delivery_address: String,
        phone_number: String,
    );

    #[ink(message, payable)]
    fn confrim_delivery(
        &mut self,
        order_id: OrderId,
    );

}