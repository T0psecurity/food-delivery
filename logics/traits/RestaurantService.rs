use crate::impls::{
    types::{FoodId, OrderId,},
};
use ink::prelude::{
    string::String,
};

#[openbrush::trait_definition]
pub trait RestaurantService {

    #[ink(message)]
    fn add_food(
        &mut self,
        food_name: String,
        description: String,
        price: u128,
        eta: u64,
    );

    #[ink(message)]
    fn update_food(
        &mut self,
        food_id: FoodId,
        food_name: String,
        description: String,
        price: u128,
        eta: u64,
    );

    #[ink(message)]
    fn confirm_order(
        &mut self,
        order_id: OrderId,
    );

    #[ink(message)]
    fn deliver_order(
        &mut self,
        order_id: OrderId,
    );
    
}
