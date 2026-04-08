
#![cfg_attr(not(test), no_std)]

use alloc::collections::{BTreeMap, VecDeque};

use serde::{Deserialize, Serialize};
extern crate alloc;
use alloc::{vec::Vec, boxed::Box};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Input {
    pub book: OrderBook,
    pub new_orders: Vec<Order>,
}

impl Input {
    pub fn new(book: OrderBook, new_orders: Vec<Order>) -> Self {
        Self { book, new_orders }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderBook {
    pub bids: BTreeMap<u64, VecDeque<Order>>,
    pub asks: BTreeMap<u64, VecDeque<Order>>
}

impl OrderBook {
    pub fn new(bids: BTreeMap<u64, VecDeque<Order>>, asks: BTreeMap<u64, VecDeque<Order>>) -> Self {
        Self {bids, asks}
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Output {
    pub book_hash: Box<[u8]>,
}

impl Output {
    pub fn new(book_hash: Box<[u8]>) -> Self {
        Self { book_hash }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Order {
    pub id: u64,
    pub price: u64,  // fixed-point, e.g. 1000 = $10.00
    pub qty: u64,
    pub is_bid: bool,
}

impl Order {
    pub fn new(id: u64, price: u64, qty: u64, is_bid: bool) -> Self {
        Self { id, price, qty, is_bid }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Trade {
    pub buyer_id: u64,
    pub seller_id: u64,
    pub price: u64,
    pub qty: u64
}

impl Trade {
    pub fn new(buyer_id: u64, seller_id: u64, price: u64, qty: u64) -> Self {
        Self { buyer_id, seller_id, price, qty }
    }
}