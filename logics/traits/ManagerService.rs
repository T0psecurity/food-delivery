use ink::prelude::string::String;
use openbrush::traits::AccountId;

#[openbrush::trait_definition]
pub trait ManagerService {

    #[ink(message)]
    fn add_restaurant(
        &mut self,
        restaurant_account: AccountId,
        restaurant_name: String,
        restaurant_address: String,
        phone_number: String,
    );

    #[ink(message)]
    fn add_deliver(
        &mut self,
        deliver_account: AccountId,
        deliver_name: String,
        deliver_address: String,
        phone_number: String,
    );

    #[ink(message)]
    fn change_manager(
        &mut self,
        new_account: AccountId,
    );
}