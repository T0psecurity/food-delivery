use crate::impls::types::{Data, OrderStatus, DeliveryStatus, Food, FoodId, OrderId, Delivery, RestaurantId, CustomerId, };
use crate::traits::RestaurantService::RestaurantService;
use ink::prelude::{
    string::String,
    vec::Vec,
};
use openbrush::{
    traits::{Storage},
};

pub trait RestaurantServiceEvents {

    fn emit_add_food_event(
        &self,
        food_id: FoodId,
        food_name: String,
        restaurant_id: RestaurantId,
        description: String,
        price: u128,
        eta: u64,
    );
    
    fn emit_update_food_event(
        &self,
        food_id: FoodId,
        food_name: String,
        description: String,
        price: u128,
        eta: u64,
    );

    fn emit_confirm_order_event(
        &self,
        order_id: OrderId,
        eta: u64,
    );

    fn emit_deliver_order_event(
        &self,
        order_id: OrderId,
        restaurant_id: RestaurantId,
        customer_id: CustomerId,
        delivery_address: String,
    );
}

impl<T> RestaurantService for T
where
    T: Storage<Data>,
{
    default fn add_food(
        &mut self,
        food_name: String,
        description: String,
        price: u128,
        eta: u64,
    ) {
        let restaurant_account = T::env().caller();
        let restaurant_id = self.data::<Data>().restaurant_account_id.get(&restaurant_account).unwrap();
        assert!(self.data::<Data>().restaurant_whitelist.contains(&restaurant_account), "Only restaurant can add food!");
        let food_id = self.data::<Data>().food_id;
        self.data::<Data>().food_id += 1;
        let food = Food {
            food_name,
            restaurant_id,
            description,
            price,
            eta,
            timestamp: T::env().block_timestamp(),
        };
        self.data::<Data>().food_data.insert(&food_id, &food);
        let mut food_vec = self.data::<Data>().restaurant_food_data.get(&restaurant_id).unwrap_or(Vec::new());
        food_vec.push(food_id);
        self.data::<Data>().restaurant_food_data.insert(&restaurant_id, &food_vec);
        let food_name = self.data::<Data>().food_data.get(&food_id).unwrap().food_name;
        let description = self.data::<Data>().food_data.get(&food_id).unwrap().description;
        self.emit_add_food_event(
            food_id,
            food_name,
            restaurant_id,
            description,
            price,
            eta,
        );
    }

    default fn update_food(
        &mut self,
        food_id: FoodId,
        food_name: String,
        description: String,
        price: u128,
        eta: u64,
    ) {
        let restaurant_account = T::env().caller();
        let restaurant_id = self.data::<Data>().restaurant_account_id.get(&restaurant_account).unwrap();
        assert!(self.data::<Data>().restaurant_whitelist.contains(&restaurant_account), "Only restaurant can update food!");
        assert!(self.data::<Data>().food_data.get(&food_id).unwrap().restaurant_id == restaurant_id, "Not owner of this food!");
        let food = Food {
            food_name,
            restaurant_id,
            description,
            price,
            eta,
            timestamp: T::env().block_timestamp(),
        };
        self.data::<Data>().food_data.insert(&food_id, &food);
        let food_name = self.data::<Data>().food_data.get(&food_id).unwrap().food_name;
        let description = self.data::<Data>().food_data.get(&food_id).unwrap().description;
        self.emit_update_food_event(
            food_id,
            food_name,
            description,
            price,
            eta,
        );
    }

    default fn confirm_order(
        &mut self,
        order_id: OrderId,
    ) {
        assert!(self.data::<Data>().order_data.contains(&order_id), "Order not exist!");
        let restaurant_account = T::env().caller();
        let restaurant_id = self.data::<Data>().restaurant_account_id.get(&restaurant_account).unwrap();
        let food_id = self.data::<Data>().order_data.get(&order_id).unwrap().food_id;
        assert!(self.data::<Data>().restaurant_whitelist.contains(&restaurant_account), "Only restaurant can confirm order!");
        assert!(self.data::<Data>().food_data.get(&food_id).unwrap().restaurant_id == restaurant_id, "Not owner of this order!");
        let mut order = self.data::<Data>().order_data.get(&order_id).unwrap();
        let status = OrderStatus::OrderConfirmed;
        order.status = status;
        let food_id = self.data::<Data>().order_data.get(&order_id).unwrap().food_id;
        let eta = self.data::<Data>().food_data.get(&food_id).unwrap().eta;
        order.eta = eta;
        self.data::<Data>().order_data.insert(&order_id, &order);
        self.emit_confirm_order_event(
            order_id,
            eta,
        );
    }

    default fn deliver_order(
        &mut self,
        order_id: OrderId,
    ) {
        assert!(self.data::<Data>().order_data.contains(&order_id), "Order not exist!");
        let restaurant_account = T::env().caller();
        let restaurant_id = self.data::<Data>().restaurant_account_id.get(&restaurant_account).unwrap();
        let food_id = self.data::<Data>().order_data.get(&order_id).unwrap().food_id;
        assert!(self.data::<Data>().restaurant_whitelist.contains(&restaurant_account), "Only restaurant can confirm order!");
        assert!(self.data::<Data>().food_data.get(&food_id).unwrap().restaurant_id == restaurant_id, "Not owner of this order!");
        let mut order = self.data::<Data>().order_data.get(&order_id).unwrap();
        let status = OrderStatus::WaitingDeliver;
        order.status = status;
        self.data::<Data>().order_data.insert(&order_id, &order);
        let delivery_id = self.data::<Data>().delivery_id;
        self.data::<Data>().delivery_id += 1;
        let restaurant_id = self.data::<Data>().order_data.get(&order_id).unwrap().restaurant_id;
        let customer_id = self.data::<Data>().order_data.get(&order_id).unwrap().customer_id;
        let deliver_id = 0;
        let delivery_address = self.data::<Data>().order_data.get(&order_id).unwrap().delivery_address;
        let status = DeliveryStatus::Waiting;
        let timestamp = T::env().block_timestamp();
        let delivery = Delivery {
            order_id,
            restaurant_id,
            customer_id,
            deliver_id,
            delivery_address,
            status,
            timestamp,
        };
        self.data::<Data>().delivery_data.insert(&delivery_id, &delivery);
        let delivery_address = self.data::<Data>().order_data.get(&order_id).unwrap().delivery_address;
        self.emit_deliver_order_event(
            order_id,
            restaurant_id,
            customer_id,
            delivery_address,
        );
    }
}

impl<T> RestaurantServiceEvents for T
where
    T: Storage<Data>
{
    fn emit_add_food_event(
        &self,
        food_id: FoodId,
        food_name: String,
        restaurant_id: RestaurantId,
        description: String,
        price: u128,
        eta: u64,
    ) {}
    
    fn emit_update_food_event(
        &self,
        food_id: FoodId,
        food_name: String,
        description: String,
        price: u128,
        eta: u64,
    ) {}

    fn emit_confirm_order_event(
        &self,
        order_id: OrderId,
        eta: u64,
    ) {}

    fn emit_deliver_order_event(
        &self,
        order_id: OrderId,
        restaurant_id: RestaurantId,
        customer_id: CustomerId,
        delivery_address: String,
    ) {}
}