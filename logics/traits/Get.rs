use crate::impls::{
    types::{Food, FoodId, OrderId, DeliverId, CustomerId, RestaurantId, Delivery, DeliveryId, Order},
};
use ink::prelude::vec::Vec;

#[openbrush::trait_definition]
pub trait Get {

    #[ink(message)]
    fn get_eta(&self, order_id: OrderId) -> u64;

    #[ink(message)]
    fn get_order_from_id(&self, order_id: OrderId) -> Order;

    #[ink(message)]
    fn get_order_from_restaurant(&self, restaurant_id: RestaurantId) -> Vec<u64>;

    #[ink(message)]
    fn get_order_from_customer(&self, customer_id: CustomerId) -> Vec<u64>;

    #[ink(message)]
    fn get_order_all(&self, from: u64, to: u64) -> Vec<Order>;

    #[ink(message)]
    fn get_food_from_id(&self, food_id: FoodId) -> Food;

    #[ink(message)]
    fn get_food_from_restaurant(&self, restaurant_id: RestaurantId) -> Vec<u64>;

    #[ink(message)]
    fn get_food_all(&self, from: u64, to: u64) -> Vec<Food>;

    #[ink(message)]
    fn get_delivery_from_id(&self, delivery_id: DeliveryId) -> Delivery;

    #[ink(message)]
    fn get_delivery_from_deliver(&self, deliver_id: DeliverId) -> Vec<u64>;

    #[ink(message)]
    fn get_delivery_all(&self, from: u64, to: u64) -> Vec<Delivery>;
    
}