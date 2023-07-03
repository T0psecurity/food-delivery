use crate::impls::types::{Data, Restaurant, Deliver, DeliverId, RestaurantId};
use crate::traits::ManagerService::ManagerService;
use ink::prelude::{
    string::String,
};
use openbrush::{
    traits::{AccountId, Storage},
};

pub trait ManagerServiceEvents {

    fn emit_add_deliver_event(
        &self,
        deliver_id: DeliverId,
        deliver_name: String,
        deliver_address: String,
        phone_number: String,
    );

    fn emit_add_restaurant_event(
        &self,
        restaurant_id: RestaurantId,
        restaurant_name: String,
        restaurant_address: String,
        phone_number: String,
    );
}

impl<T> ManagerService for T
where
    T: Storage<Data>,
{
    default fn add_restaurant(
        &mut self,
        restaurant_account: AccountId,
        restaurant_name: String,
        restaurant_address: String,
        phone_number: String,
    ) {
        let caller = T::env().caller();
        assert!(caller != self.data::<Data>().manager, "Only manager can add restaurant!");
        assert!(!self.data::<Data>().restaurant_whitelist.contains(&restaurant_account), "already exist restaurant!");
        let restaurant_id = self.data::<Data>().restaurant_id;
        self.data::<Data>().restaurant_id += 1;
        let restaurant = Restaurant {
            restaurant_account,
            restaurant_name,
            restaurant_address,
            phone_number,
        };
        self.data::<Data>().restaurants.insert(&restaurant_id, &restaurant);
        self.data::<Data>().restaurant_account_id.insert(&restaurant_account, &restaurant_id);
        self.data::<Data>().restaurant_whitelist.push(restaurant_account);
        let restaurant_name = self.data::<Data>().restaurants.get(&restaurant_id).unwrap().restaurant_name;
        let restaurant_address = self.data::<Data>().restaurants.get(&restaurant_id).unwrap().restaurant_address;
        let phone_number = self.data::<Data>().restaurants.get(&restaurant_id).unwrap().phone_number;
        self.emit_add_restaurant_event(
            restaurant_id,
            restaurant_name,
            restaurant_address,
            phone_number,
        );
    }

    default fn add_deliver(
        &mut self,
        deliver_account: AccountId,
        deliver_name: String,
        deliver_address: String,
        phone_number: String,
    ) {
        let caller = T::env().caller();
        assert!(caller == self.data::<Data>().manager, "Only manager can add deliver!");
        assert!(!self.data::<Data>().deliver_whitelist.contains(&deliver_account), "already exist deliver!");
        let deliver_id = self.data::<Data>().deliver_id;
        self.data::<Data>().deliver_id += 1;
        let deliver = Deliver {
            deliver_account,
            deliver_name,
            deliver_address,
            phone_number,
        };
        self.data::<Data>().delivers.insert(&deliver_id, &deliver);
        self.data::<Data>().deliver_account_id.insert(&deliver_account, &deliver_id);
        self.data::<Data>().deliver_whitelist.push(deliver_account);
        let deliver_name = self.data::<Data>().delivers.get(&deliver_id).unwrap().deliver_name;
        let deliver_address = self.data::<Data>().delivers.get(&deliver_id).unwrap().deliver_address;
        let phone_number = self.data::<Data>().delivers.get(&deliver_id).unwrap().phone_number;
        self.emit_add_deliver_event(
            deliver_id,
            deliver_name,
            deliver_address,
            phone_number,
        );
    }

    default fn change_manager(
        &mut self,
        new_account: AccountId,
    ) {
        let caller = T::env().caller();
        assert!(caller == self.data::<Data>().manager, "Only manager can add deliver!");
        self.data::<Data>().manager = new_account;
    }
}

impl<T> ManagerServiceEvents for T
where
    T: Storage<Data>
{
    default fn emit_add_deliver_event(
        &self,
        deliver_id: DeliverId,
        deliver_name: String,
        deliver_address: String,
        phone_number: String,
    ) {}

    default fn emit_add_restaurant_event(
        &self,
        restaurant_id: RestaurantId,
        restaurant_name: String,
        restaurant_address: String,
        phone_number: String,
    ) {}
}