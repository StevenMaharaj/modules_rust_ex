mod feed;
mod deribit;

use deribit::deribit_feed::DeribitOrderFeed;
use feed::OrderFeed;

fn main() {
    println!("Hello, world!");

    let deribit_feed = DeribitOrderFeed {
        url: String::from("https://deribit.com"),
    };
    deribit_feed.start();
}
