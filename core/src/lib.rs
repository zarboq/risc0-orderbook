
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
    pub bids: BTreeMap<u32, VecDeque<Order>>,
    pub asks: BTreeMap<u32, VecDeque<Order>>
}

impl OrderBook {
    pub fn new(bids: BTreeMap<u32, VecDeque<Order>>, asks: BTreeMap<u32, VecDeque<Order>>) -> Self {
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
    pub id: u32,
    pub price: u32,
    pub qty: u32,
    pub is_bid: bool,
}

impl Order {
    pub fn new(id: u32, price: u32, qty: u32, is_bid: bool) -> Self {
        Self { id, price, qty, is_bid }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Trade {
    pub buyer_id: u32,
    pub seller_id: u32,
    pub price: u32,
    pub qty: u32
}

impl Trade {
    pub fn new(buyer_id: u32, seller_id: u32, price: u32, qty: u32) -> Self {
        Self { buyer_id, seller_id, price, qty }
    }
}