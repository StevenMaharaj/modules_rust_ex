
use super::super::feed::OrderFeed;

pub struct DeribitOrderFeed {
    pub url: String,
}

impl OrderFeed for DeribitOrderFeed {
    fn start(&self) {
        println!("Starting Deribit order feed");
        self.helper();
    }
}

impl DeribitOrderFeed {
    fn helper(&self) {
        println!("DeribitOrderFeed helper");
    }
}


