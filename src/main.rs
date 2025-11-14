mod matching_engine;
use matching_engine::{engine::{MatchingEngine, TradingPair}, orderbook::{BidOrAsk, Order, Orderbook}};
use rust_decimal::prelude::*;
fn main() {
    let buy_order = Order::new(BidOrAsk::Bid,5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid,2.45);

    //let sell_order = Order::new(BidOrAsk::Ask,2.45);
    let mut orderbook = Orderbook::new();

    orderbook.add_order(dec!(4.4),buy_order);
    orderbook.add_order(dec!(4.4),buy_order_from_bob);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_order(dec!(20), sell_order);

    //println!("{:?}", orderbook);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("SOL".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    let eth_pair = TradingPair::new("ETH".to_string(), "USD".to_string());
    engine.place_limit_order(pair, dec!(10.000), buy_order).unwrap();
}
