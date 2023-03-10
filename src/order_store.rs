use std::{error::Error, fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct OrderStoreNewType(pub Box<dyn OrderStore>); // //dyn: dynamic implementation of orderStore -> so not pegged to a specific implementation but
                                                       // to a generalization
                                                       //struct OrderStoreNewType(...) -> struct type tuple which means
                                                       // that have values that are not referenced by name but by indexes inside the type.
                                                       // Box<dyn OrderStore> -> Means a 'Box' of something that implements the trait 'OrderStore'
                                                       // Box genereates like a pointer that points to the type inside (in this case 'dyn OrderStore')

impl OrderStoreNewType {
    pub fn new(repo: impl OrderStore) -> OrderStoreNewType {
        OrderStoreNewType(Box::new(repo)) // Box -> to put something in memory
                                          // repo -> the thing we are putting in memory
    }
}

impl Deref for OrderStoreNewType {
    type Target = dyn OrderStore; // stating that what we are returning is of whatever type that implements OrderStore
    fn deref(&self) -> &Self::Target {
        self.0.as_ref() // wil return the reference of whatever is in the position '0'
    }
}

/// Representation of an item of an order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    /// Id of the product.
    pub product_id: Uuid,
    /// Number of items of this product.
    pub quantity: i32,
}

/// Representation of an order in the system.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    /// An order is identified by its id.
    pub id: Uuid,
    /// Each order belongs to a user.
    pub user_id: Uuid,
    /// This holds the list of items included in the order.
    pub items: Vec<Item>,
}

impl Order {
    /// Creates a new order in the store for user `user_id` and returns it.
    pub fn new(user_id: Uuid) -> Order {
        Order {
            id: Uuid::new_v4(),
            user_id,
            items: vec![],
        }
    }
}

/// Type fos describing errors that result from trying to interact with an [`OrderStore`](OrderStore).
#[derive(Debug)]
pub enum OrderStoreError {
    /// The store is unavailable.
    #[allow(dead_code)]
    StoreUnavailable,
    /// Provided order id was not found in the store.
    OrderNotFound(Uuid),
    /// Provided item index is out of bounds for the provided order.
    ItemIndexOutOfBounds(usize),
}

impl Display for OrderStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStoreError::StoreUnavailable => {
                write!(f, "Store unavailable")
            }
            OrderStoreError::OrderNotFound(id) => {
                write!(f, "Order not found {}", id)
            }
            OrderStoreError::ItemIndexOutOfBounds(index) => {
                write!(f, "Item index out of bounds: {}", index)
            }
        }
    }
}

impl Error for OrderStoreError {}

/// A trait that defines the behavior of a type used to store orders.
#[async_trait::async_trait]
pub trait OrderStore: Send + Sync + 'static {
    // adding 'static' to avoid OrderStore to be deleted from memory before needed
    // adding 'Sync' and 'Send' since it is required by 'Arc'.
    /// Creates a new order associated to user `user_id`.
    ///
    /// Returns a copy of the order on success, otherwise it returns an error.
    ///
    /// # Errors
    ///
    /// Returns [`StoreUnavailable`](OrderStoreError::StoreUnavailable) if the Store cannot be used to create an order.
    async fn create_order(&self, user_id: Uuid) -> Result<Order, OrderStoreError>;

    /// Gets an order from its id.
    ///
    /// Returns a copy of the order on success, otherwise it returns an error.
    ///
    /// Returns [`StoreUnavailable`](OrderStoreError::StoreUnavailable) if the Store cannot be used to create an order.
    ///
    /// Returns [`OrderNotFound`](OrderStoreError::OrderNotFound) if there is no order with the provided id in the Store.
    async fn get_order(&self, order_id: Uuid) -> Result<Order, OrderStoreError>;

    /// Returns the list of orders that belong to the user with id `user_id` in the Store.
    ///
    /// Returns a copy of the list of orders on success, otherwise it returns an error.
    ///
    /// # Errors
    ///
    /// Returns [`StoreUnavailable`](OrderStoreError::StoreUnavailable) if the Store cannot be used to create an order.
    async fn list_orders(&self, user_id: Uuid) -> Result<Vec<Order>, OrderStoreError>;

    /// Adds an item to the order with id `order_id`.
    ///
    /// Returns an empty Ok on success, otherwise it returns an error.
    ///
    /// # Errors
    ///
    /// Returns [`StoreUnavailable`](OrderStoreError::StoreUnavailable) if the Store cannot be used to create an order.
    ///
    /// Returns [`OrderNotFound`](OrderStoreError::OrderNotFound) if there is no order with the provided id in the Store.
    async fn add_item(
        &self,
        order_id: Uuid,
        product_id: Uuid,
        quantity: i32,
    ) -> Result<(), OrderStoreError>;

    /// Adds an item to the order with id `order_id`.
    ///
    /// Returns an empty Ok on success, otherwise it returns an error.
    ///
    /// # Errors
    ///
    /// Returns [`StoreUnavailable`](OrderStoreError::StoreUnavailable) if the Store cannot be used to create an order.
    ///
    /// Returns [`OrderNotFound`](OrderStoreError::OrderNotFound) if there is no order with the provided id in the Store.
    ///
    /// Returns [`ItemIndexOutOfBounds`](OrderStoreError::ItemIndexOutOfBounds) if the item index doesn't exist in the order.
    async fn delete_item(&self, order_id: Uuid, index: usize) -> Result<(), OrderStoreError>;
}
