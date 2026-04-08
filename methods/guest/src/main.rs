use risc0_zkvm::{guest::env, sha::{Sha256 as _, Impl as Sha256}};
use core::{Input, OrderBook, Trade};
extern crate alloc;
use alloc::collections::{VecDeque};

fn main() {
    // read the input
    let mut input: Input = env::read();
    // Add orders to order_book
    input.new_orders.iter().for_each(|order| {
        if order.is_bid {
            input.book.bids.entry(order.price)
                .or_insert(VecDeque::with_capacity(input.new_orders.len()))
                .push_back(order.clone());
        } else {
            input.book.asks.entry(order.price)
            .or_insert(VecDeque::with_capacity(input.new_orders.len()))
            .push_back(order.clone());
        }
    });
    let mut trades = Vec::with_capacity(input.new_orders.len());
    while let Some(trade) = match_orders(&mut input.book) {
        trades.push(trade);
    }

    let book_bytes = bincode::serialize(&input.book).unwrap();
    let book_hash = Sha256::hash_bytes(&book_bytes);
    // write updated order book hash to the journal
    env::commit_slice(book_hash.as_bytes());

    env::write(&trades);
    env::write(&input.book);
}

fn match_orders(order_book: &mut OrderBook) -> Option<Trade> {
    let mut bid_entry = order_book.bids.last_entry()?;
    let bid = bid_entry.get_mut().front_mut()?;
    let mut ask_entry = order_book.asks.first_entry()?;
    let ask = ask_entry.get_mut().front_mut()?;
    if bid.price >= ask.price {
        let qty = bid.qty.min(ask.qty);
        let trade = Trade::new(bid.id, ask.id, ask.price, qty);
        if bid.qty - qty == 0 {
            order_book.bids.pop_last();
        } else {
            bid.qty -= qty;
        }
        if ask.qty - qty == 0 {
            order_book.asks.pop_first();
        } else {
            ask.qty -= qty;
        }
        return Some(trade);
    }
    return None;
}