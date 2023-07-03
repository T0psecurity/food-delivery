#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]

mod contract {
    use ink::codegen::EmitEvent;
    use ink::codegen::Env;
    use ink::prelude::string::String;
    use logics::{
        impls::{
            CustomerService::CustomerServiceEvents,
            DeliverService::DeliverServiceEvents,
            ManagerService::ManagerServiceEvents,
            RestaurantService::RestaurantServiceEvents,
            types::{Data, OrderId, FoodId, RestaurantId, CustomerId, DeliverId, DeliveryId},
        },
        traits::{
            CustomerService::CustomerService,
            DeliverService::DeliverService,
            Get::Get,
            ManagerService::ManagerService,
            RestaurantService::RestaurantService,
        },
    };
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Storage, Default)]
    pub struct FoodOrder {
        #[storage_field]
        food_order_data: Data,
    }

    impl CustomerService for FoodOrder {}

    impl DeliverService for FoodOrder {}

    impl Get for FoodOrder {}

    impl ManagerService for FoodOrder {}
    
    impl RestaurantService for FoodOrder {}

    impl FoodOrder {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance.manager = Self::env().caller();
            instance
        }
    }

    #[ink(event)]
    pub struct SubmitOrderEvent {
        order_id: OrderId,
        food_id: FoodId,
        restaurant_id: RestaurantId,
        customer_id: CustomerId,
        delivery_address: String,
        phone_number: String,
    }

    #[ink(event)]
    pub struct ConfirmDeliveryEvent {
        order_id: OrderId,
    }

    #[ink(event)]
    pub struct AddFoodEvent {
        food_id: FoodId,
        food_name: String,
        restaurant_id: RestaurantId,
        restaurant_address: String,
        phone_number: String,
        description: String,
        price: u128,
        eta: u64,
    }

    #[ink(event)]
    pub struct UpdateFoodEvent {
        food_id: FoodId,
        food_name: String,
        description: String,
        price: u128,
        eta: u64,
    }

    #[ink(event)]
    pub struct ConfirmOrderEvent {
        order_id: OrderId,
        eta: u64,
    }

    #[ink(event)]
    pub struct DeliverOrderEvent {
        order_id: OrderId,
        restaurant_id: RestaurantId,
        customer_id: CustomerId,
        delivery_address: String,
    }

    #[ink(event)]
    pub struct AddDeliverEvent {
        deliver_id: DeliverId,
        deliver_name: String,
        deliver_address: String,
        phone_number: String,
    }

    #[ink(event)]
    pub struct AddRestaurantEvent {
        restaurant_id: RestaurantId,
        restaurant_name: String,
        restaurant_address: String,
        phone_number: String,
    }

    impl CustomerServiceEvents for CustomerService {
        fn emit_submit_order_event(
            &self,
            order_id: OrderId,
            food_id: FoodId,
            restaurant_id: RestaurantId,
            customer_id: CustomerId,
            delivery_address: String,
            phone_number: String,
        ) {
            self.env().emit_event(SubmitOrderEvent {
                order_id,
                food_id,
                restaurant_id,
                customer_id,
                delivery_address,
                phone_number,
            })
        }

        fn emit_confirm_delivery_event(
            &self,
            order_id: OrderId,
        ) {
            self.env().emit_event(ConfirmDeliveryEvent {
                order_id,
            })
        }
    }

    impl DeliverServiceEvents for FoodOrder {
        fn emit_pickup_delivery_event(
            &self,
            delivery_id: DeliveryId,
        ) {
            self.env().emit_event(ConfirmDeliveryEvent {
                delivery_id,
            })
        }
    }

    impl ManagerServiceEvents for FoodOrder {
        fn emit_add_deliver_event(
            &self,
            deliver_id: DeliverId,
            deliver_name: String,
            deliver_address: String,
            phone_number: String,
        ) {
            self.env().emit_event(AddDeliverEvent {
                deliver_id,
                deliver_name,
                deliver_address,
                phone_number,
            })
        }
    
        fn emit_add_restaurant_event(
            &self,
            restaurant_id: RestaurantId,
            restaurant_name: String,
            restaurant_address: String,
            phone_number: String,
        ) {
            self.env().emit_event(AddRestaurantEvent {
                restaurant_id,
                restaurant_name,
                restaurant_address,
                phone_number,
            })
        }
    }

    impl RestaurantServiceEvents for FoodOrder {
        fn emit_add_food_event(
            &self,
            food_id: FoodId,
            food_name: String,
            restaurant_id: RestaurantId,
            description: String,
            price: u128,
            eta: u64,
        ) {
            self.env().emit_event(AddFoodEvent {
                food_id,
                food_name,
                restaurant_id,
                description,
                price,
                eta,
            })
        }
    
        fn emit_update_food_event(
            &self,
            food_id: FoodId,
            food_name: String,
            description: String,
            price: u128,
            eta: u64,
        ) {
            self.env().emit_evet(UpdateFoodEvent {
                food_id,
                food_name,
                description,
                price,
                eta,
            })
        }
    
        fn emit_confirm_order_event(
            &self,
            order_id: OrderId,
            eta: u64,
        ) {
            self.env().emit_event(ConfirmOrderEvent {
                order_id,
                eta
            })
        }
    
        fn emit_deliver_order_event(
            &self,
            order_id: OrderId,
            restaurant_id: RestaurantId,
            customer_id: CustomerId,
            delivery_address: String,
        ) {
            self.env().emit_event(DeliverOrderEvent {
                order_id,
                restaurant_id,
                customer_id,
                delivery_address,
            })
        }
    }
}
