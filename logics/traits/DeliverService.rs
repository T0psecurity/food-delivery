use crate::impls::{
    types::{DeliveryId},
};

#[openbrush::trait_definition]
pub trait DeliverService {

    #[ink(message)]
    fn confirm_delivery(
        &mut self,
        delivery_id: DeliveryId,
    );
    
}