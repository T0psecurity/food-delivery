use crate::impls::types::{Data, OrderStatus, Customer, Order, FoodId, RestaurantId, OrderId, CustomerId};
use crate::traits::CustomerService::CustomerService;

use ink::prelude::{
    string::String,
    vec::Vec,
};
use openbrush::{
    traits::Storage,
};

pub trait CustomerServiceEvents {

    fn emit_submit_order_event(
        &self,
        order_id: OrderId,
        food_id: FoodId,
        restaurant_id: RestaurantId,
        customer_id: CustomerId,
        delivery_address: String,
        phone_number: String,
    );

    fn emit_confirm_delivery_event(
        &self,
        order_id: OrderId,
    );
}

impl<T> CustomerService for T
where
    T: Storage<Data>,
{
    default fn add_customer(
        &mut self,
        customer_name: String,
        customer_address: String,
        phone_number: String
    ) {
        let customer_account = T::env().caller();
        assert!(!self.data::<Data>().customer_whitelist.contains(&customer_account), "alread exist customer!");
        let customer = Customer {
            customer_account,
            customer_name,
            customer_address,
            phone_number,
        };
        let customer_id = self.data::<Data>().customer_id;
        self.data::<Data>().customer_id += 1;
        self.data::<Data>().customers.insert(&customer_id, &customer);
        self.data::<Data>().customer_whitelist.push(customer_account);
        self.data::<Data>().customer_account_id.insert(&customer_account, &customer_id);
    }

    default fn submit_order(
        &mut self, 
        food_id: FoodId,
        restaurant_id: RestaurantId,
        delivery_address: String,
        phone_number: String,
    ) {
        let customer_account = T::env().caller();
        assert!(self.data::<Data>().customer_whitelist.contains(&customer_account), "only customer can submit order!");
        let customer_id = self.data::<Data>().customer_account_id.get(&customer_account).unwrap();
        let order_id = self.data::<Data>().order_id;
        let deliver_id = 0;
        let price = T::env().transferred_value();
        assert!(self.data::<Data>().food_data.get(&food_id).unwrap().price == price, "you must pay same of price!");
        let eta = 0;
        let timestamp = T::env().block_timestamp();
        let status = OrderStatus::OrderSubmitted;
        let order = Order {
            food_id,
            restaurant_id,
            customer_id,
            deliver_id,
            delivery_address,
            status,
            timestamp,
            price,
            eta,
        };
        let order_id = self.data::<Data>().order_id;
        self.data::<Data>().order_id += 1;
        self.data::<Data>().order_data.insert(&order_id, &order);
        let mut customer_vec = self.data::<Data>().customer_order_data.get(&customer_id).unwrap_or(Vec::new());
        customer_vec.push(order_id);
        self.data::<Data>().customer_order_data.insert(&customer_id, &customer_vec);
        let mut restaurant_vec = self.data::<Data>().restaurant_order_data.get(&restaurant_id).unwrap_or(Vec::new());
        restaurant_vec.push(order_id);
        self.data::<Data>().restaurant_order_data.insert(&restaurant_id, &restaurant_vec);
        let delivery_address = self.data::<Data>().order_data.get(&order_id).unwrap().delivery_address;
        let customer_id = self.data::<Data>().order_data.get(&order_id).unwrap().customer_id;
        let phone_number = self.data::<Data>().customers.get(&customer_id).unwrap().phone_number;
        self.emit_submit_order_event(
            order_id,
            food_id,
            restaurant_id,
            customer_id,
            delivery_address,
            phone_number,
        );
    }

    default fn confrim_delivery(
        &mut self,
        order_id: OrderId,
    ) {
        let customer_account = T::env().caller();
        assert!(self.data::<Data>().customer_whitelist.contains(&customer_account), "only customer can submit order!");
        let customer_id = self.data::<Data>().customer_account_id.get(&customer_account).unwrap();
        assert!(self.data::<Data>().order_data.get(&order_id).unwrap().customer_id == customer_id, "not customer of this order!");
        let mut order = self.data::<Data>().order_data.get(&order_id).unwrap();
        let status = OrderStatus::DeliveryAcceptted;
        order.status = status;
        self.data::<Data>().order_data.insert(&order_id, &order);
        self.emit_confirm_delivery_event(
            order_id,
        );
    }
}

impl<T> CustomerServiceEvents for T
where
    T: Storage<Data>,
{
    default fn emit_submit_order_event(
        &self,
        order_id: OrderId,
        food_id: FoodId,
        restaurant_id: RestaurantId,
        customer_id: CustomerId,
        delivery_address: String,
        phone_number: String,
    ) {}

    default fn emit_confirm_delivery_event(
        &self,
        order_id: OrderId,
    ) {}
}