mod matching_engine;
use matching_engine::orderbook::{Order, BidOrAsk, Orderbook};
fn main() {
    let buy_order = Order::new(BidOrAsk::Bid,5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid,2.45);

    //let sell_order = Order::new(BidOrAsk::Ask,2.45);
    let mut orderbook = Orderbook::new();

    orderbook.add_order(4.4,buy_order);
    orderbook.add_order(4.4,buy_order_from_bob);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_order(20.0, sell_order);

    println!("{:?}", orderbook);
}
