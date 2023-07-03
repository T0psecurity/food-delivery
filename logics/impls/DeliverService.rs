use crate::impls::{
    types::{ Data, DeliveryId, DeliveryStatus, OrderStatus },
};
use crate::traits::DeliverService::DeliverService;
use openbrush::{
    traits::Storage,
};

pub trait DeliverServiceEvents {

    fn emit_pickup_delivery_event(
        &self,
        delivery_id: DeliveryId,
    );
}

impl<T> DeliverService for T
where
    T: Storage<Data>,
{
    default fn confirm_delivery(
        &mut self,
        delivery_id: DeliveryId,
    ) {
        let caller = T::env().caller();
        assert!(self.data::<Data>().deliver_whitelist.contains(&caller), "only deliver can confirm devliery");
        assert!(self.data::<Data>().delivery_data.get(&delivery_id).unwrap().status == DeliveryStatus::Waiting, "this delivery is already picked up!");
        let mut delivery = self.data::<Data>().delivery_data.get(&delivery_id).unwrap();
        let status = DeliveryStatus::PickUp;
        delivery.status = status;
        self.data::<Data>().delivery_data.insert(&delivery_id, &delivery);
        let order_id = self.data::<Data>().delivery_data.get(&delivery_id).unwrap().order_id;
        let order_status = OrderStatus::OrderDelivered;
        let mut order = self.data::<Data>().order_data.get(&order_id).unwrap();
        order.status = order_status;
        self.data::<Data>().order_data.insert(&order_id, &order);
        self.emit_pickup_delivery_event(delivery_id);
    }
}

impl<T> DeliverServiceEvents for T
where
    T: Storage<Data>
{
    default fn emit_pickup_delivery_event(
        &self,
        delivery_id: DeliveryId,
    ) {}
}