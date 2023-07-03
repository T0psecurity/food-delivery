use ink::prelude::{
    string::String,
    vec::Vec,
};
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        ZERO_ADDRESS,
        Timestamp
    },
};

pub type FoodId = u64;
pub type OrderId = u64;
pub type DeliveryId = u64;
pub type CustomerId = u64;
pub type RestaurantId = u64;
pub type DeliverId = u64;


#[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum OrderStatus {
    OrderSubmitted,
    OrderConfirmed,
    WaitingDeliver,
    OrderDelivered,
    DeliveryAcceptted,
}

#[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum DeliveryStatus {
    Waiting,
    PickUp,
}

#[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Customer {
    pub customer_account: AccountId,
    pub customer_name: String,
    pub customer_address: String,
    pub phone_number: String,
}

impl Default for Customer {
    fn default() -> Self {
        Customer {
            customer_account: ZERO_ADDRESS.into(),
            customer_name: Default::default(),
            customer_address: Default::default(),
            phone_number: Default::default(),
        }
    }
}

#[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Restaurant {
    pub restaurant_account: AccountId,
    pub restaurant_name: String,
    pub restaurant_address: String,
    pub phone_number: String,
}

impl Default for Restaurant {
    fn default() -> Self {
        Restaurant {
            restaurant_account: ZERO_ADDRESS.into(),
            restaurant_name: Default::default(),
            restaurant_address: Default::default(),
            phone_number: Default::default(),
        }
    }
}

#[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Deliver {
    pub deliver_account: AccountId,
    pub deliver_name: String,
    pub deliver_address: String,
    pub phone_number: String,
}

impl Default for Deliver {
    fn default() -> Self {
        Deliver {
            deliver_account: ZERO_ADDRESS.into(),
            deliver_name: Default::default(),
            deliver_address: Default::default(),
            phone_number: Default::default(),
        }
    }
}

#[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Food {
    pub food_name: String,
    pub restaurant_id: RestaurantId,
    pub description: String,
    pub price: u128,
    pub eta: u64,
    pub timestamp: Timestamp,
}

impl Default for Food {
    fn default() -> Self {
        Food {
            food_name: Default::default(),
            restaurant_id: Default::default(),
            description: Default::default(),
            price: Default::default(),
            eta: Default::default(),
            timestamp: Default::default(),
        }
    }
}

#[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Order {
    pub food_id: FoodId,
    pub restaurant_id: RestaurantId,
    pub customer_id: CustomerId,
    pub deliver_id: DeliveryId,
    pub delivery_address: String,
    pub status: OrderStatus,
    pub timestamp: Timestamp,
    pub price: u128,
    pub eta: u64,
}

impl Default for Order {
    fn default() -> Self {
        Order {
            food_id: Default::default(),
            restaurant_id: Default::default(),
            customer_id: Default::default(),
            deliver_id: Default::default(),
            delivery_address: Default::default(),
            status: OrderStatus::OrderSubmitted,
            timestamp: Default::default(),
            price: Default::default(),
            eta: Default::default(),
        }
    }
}

#[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Delivery {
    pub order_id: OrderId,
    pub restaurant_id: RestaurantId,
    pub customer_id: CustomerId,
    pub deliver_id: DeliverId,
    pub delivery_address: String,
    pub status: DeliveryStatus,
    pub timestamp: Timestamp,
}

impl Default for Delivery {
    fn default() -> Self {
        Delivery {
            order_id: Default::default(),
            restaurant_id: Default::default(),
            customer_id: Default::default(),
            deliver_id: Default::default(),
            delivery_address: Default::default(),
            status: DeliveryStatus::Waiting,
            timestamp: Default::default(),
        }
    }
}

pub const FOODORDER_STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(FOODORDER_STORAGE_KEY)]

pub struct Data {
    pub manager: AccountId,
    pub food_id: u64,
    pub order_id: u64,
    pub delivery_id: u64,
    pub customer_id: u64,
    pub restaurant_id: u64,
    pub deliver_id: u64,
    pub customers: Mapping<CustomerId, Customer>,
    pub restaurants: Mapping<RestaurantId, Restaurant>,
    pub delivers: Mapping<DeliverId, Deliver>,
    pub food_data: Mapping<FoodId, Food>,
    pub order_data: Mapping<OrderId, Order>,
    pub delivery_data: Mapping<DeliveryId, Delivery>,
    pub restaurant_food_data: Mapping<RestaurantId, Vec<FoodId>>,
    pub restaurant_order_data: Mapping<RestaurantId, Vec<OrderId>>,
    pub customer_order_data: Mapping<CustomerId, Vec<OrderId>>,
    pub deliver_delivery_data: Mapping<DeliverId, Vec<DeliveryId>>,
    pub customer_whitelist: Vec<AccountId>,
    pub restaurant_whitelist: Vec<AccountId>,
    pub deliver_whitelist: Vec<AccountId>,
    pub customer_account_id: Mapping<AccountId, CustomerId>,
    pub restaurant_account_id: Mapping<AccountId, RestaurantId>,
    pub deliver_account_id: Mapping<AccountId, DeliverId>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            manager: ZERO_ADDRESS.into(),
            food_id: 1,
            order_id: 1,
            deliver_id: 1,
            customer_id: 1,
            restaurant_id: 1,
            delivery_id: 1,
            customers: Mapping::default(),
            restaurants: Mapping::default(),
            delivers: Mapping::default(),
            food_data: Mapping::default(),
            order_data: Mapping::default(),
            delivery_data: Mapping::default(),
            restaurant_food_data: Mapping::default(),
            restaurant_order_data: Mapping::default(),
            customer_order_data: Mapping::default(),
            deliver_delivery_data: Mapping::default(),
            customer_whitelist: Vec::new(),
            restaurant_whitelist: Vec::new(),
            deliver_whitelist: Vec::new(),
            customer_account_id: Mapping::default(),
            restaurant_account_id: Mapping::default(),
            deliver_account_id: Mapping::default(),
        }
    }
}


