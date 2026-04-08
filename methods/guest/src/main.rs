use risc0_zkvm::{guest::env, sha::{Sha256 as _, Impl as Sha256}};
use core::{Order, OrderBook, Trade};
extern crate alloc;
use alloc::collections::VecDeque;

fn main() {
    let mut book: OrderBook = env::read();
    let num_words: u32 = env::read();
    let mut order_data = alloc::vec![0u32; num_words as usize];
    env::read_slice(&mut order_data);
    let num_orders = order_data.len() / 4;
    // Insert new orders into the book
    for chunk in order_data.chunks_exact(4) {
        let order = Order::new(chunk[0], chunk[1], chunk[2], chunk[3] != 0);
        if order.is_bid {
            book.bids.entry(order.price)
                .or_insert(VecDeque::with_capacity(num_orders))
                .push_back(order);
        } else {
            book.asks.entry(order.price)
                .or_insert(VecDeque::with_capacity(num_orders))
                .push_back(order);
        }
    }
    let mut trades = Vec::with_capacity(num_orders);
    while let Some(trade) = match_orders(&mut book) {
        trades.push(trade);
    }

    let book_bytes = bincode::serialize(&book).unwrap();
    let book_hash = Sha256::hash_bytes(&book_bytes);
    // write updated order book hash to the journal
    env::commit_slice(book_hash.as_bytes());

    env::write(&(&trades, &book));
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
