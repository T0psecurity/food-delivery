use crate::impls::types::{Data, OrderId, Order, RestaurantId, CustomerId, Food, FoodId, DeliverId, DeliveryId, Delivery};
use crate::traits::Get::Get;
use ink::prelude::{
    vec::Vec,
};
use openbrush::{
    traits::{Storage},
};

impl<T> Get for T
where
    T: Storage<Data>,
{
    default fn get_eta(&self, order_id: OrderId) -> u64 {
        assert!(self.data::<Data>().order_data.contains(&order_id), "Order does not exist!");
        let timestamp = self.data::<Data>().order_data.get(&order_id).unwrap().timestamp;
        let cur_timestamp = T::env().block_timestamp();
        let order_eta = self.data::<Data>().order_data.get(&order_id).unwrap().eta;
        let eta = order_eta - (cur_timestamp - timestamp);
        if eta > 0 {
            eta
        } else {
            0
        }
    }

    default fn get_order_from_id(&self, order_id: OrderId) -> Order {
        assert!(self.data::<Data>().order_data.contains(&order_id), "Order does not exist!");
        self.data::<Data>().order_data.get(&order_id).unwrap()
    }

    default fn get_order_from_restaurant(&self, restaurant_id: RestaurantId) -> Vec<u64> {
        assert!(self.data::<Data>().restaurants.contains(&restaurant_id), "Restaurant does not exist!");
        let order_data = self.data::<Data>().restaurant_order_data.get(&restaurant_id).unwrap_or(Vec::new());
        order_data        
    }

    default fn get_order_from_customer(&self, customer_id: CustomerId) -> Vec<u64> {
        assert!(self.data::<Data>().customers.contains(&customer_id), "Restaurant does not exist!");
        let mut order_vec: Vec<Order> = Vec::new();
        let order_data = self.data::<Data>().customer_order_data.get(&customer_id).unwrap_or(Vec::new());
        order_data
    }

    default fn get_order_all(&self, from: u64, to: u64) -> Vec<Order> {
        let mut order_vec: Vec<Order> = Vec::new();
        if to < self.data::<Data>().order_id {
            for i in from..to {
                order_vec.push(self.data::<Data>().order_data.get(&i).unwrap());
            }
        } else {
            for i in from..self.data::<Data>().order_id {
                order_vec.push(self.data::<Data>().order_data.get(&i).unwrap());
            }
        }
        order_vec
    }

    default fn get_food_from_id(&self, food_id: FoodId) -> Food {
        assert!(self.data::<Data>().food_data.contains(&food_id), "Order does not exist!");
        self.data::<Data>().food_data.get(&food_id).unwrap()
    }

    default fn get_food_from_restaurant(&self, restaurant_id: RestaurantId) -> Vec<u64> {
        assert!(self.data::<Data>().restaurants.contains(&restaurant_id), "Restaurant does not exist!");
        let mut food_vec: Vec<Food> = Vec::new();
        let food_data = self.data::<Data>().restaurant_food_data.get(&restaurant_id).unwrap_or(Vec::new());
        food_data        
    }

    default fn get_food_all(&self, from: u64, to: u64) -> Vec<Food> {
        let mut food_vec: Vec<Food> = Vec::new();
        if to < self.data::<Data>().food_id {
            for i in from..to {
                food_vec.push(self.data::<Data>().food_data.get(&i).unwrap());
            }
        } else {
            for i in from..self.data::<Data>().food_id {
                food_vec.push(self.data::<Data>().food_data.get(&i).unwrap());
            }
        }
        food_vec
    }

    default fn get_delivery_from_id(&self, delivery_id: DeliveryId) -> Delivery {
        assert!(self.data::<Data>().delivery_data.contains(&delivery_id), "Order does not exist!");
        self.data::<Data>().delivery_data.get(&delivery_id).unwrap()
    }

    default fn get_delivery_from_deliver(&self, deliver_id: DeliverId) -> Vec<u64> {
        assert!(self.data::<Data>().delivers.contains(&deliver_id), "Restaurant does not exist!");
        let mut delivery_vec: Vec<Delivery> = Vec::new();
        let delivery_data = self.data::<Data>().deliver_delivery_data.get(&deliver_id).unwrap_or(Vec::new());
        delivery_data        
    }

    default fn get_delivery_all(&self, from: u64, to: u64) -> Vec<Delivery> {
        let mut delivery_vec: Vec<Delivery> = Vec::new();
        if to < self.data::<Data>().delivery_id {
            for i in from..to {
                delivery_vec.push(self.data::<Data>().delivery_data.get(&i).unwrap());
            }
        } else {
            for i in from..self.data::<Data>().delivery_id {
                delivery_vec.push(self.data::<Data>().delivery_data.get(&i).unwrap());
            }
        }
        delivery_vec
    }
}