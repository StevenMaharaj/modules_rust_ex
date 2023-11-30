


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataROrders {
    #[serde(rename = "is_liquidation")]
    pub is_liquidation: bool,
    #[serde(rename = "risk_reducing")]
    pub risk_reducing: bool,
    #[serde(rename = "order_type")]
    pub order_type: String,
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: u64,
    #[serde(rename = "order_state")]
    pub order_state: String,
    #[serde(rename = "average_price")]
    pub average_price: f64,
    #[serde(rename = "time_in_force")]
    pub time_in_force: String,
    #[serde(rename = "filled_amount")]
    pub filled_amount: f64,
    #[serde(rename = "max_show")]
    pub max_show: f64,
    #[serde(rename = "reject_post_only")]
    pub reject_post_only: Option<bool>,
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "reduce_only")]
    pub reduce_only: bool,
    #[serde(rename = "post_only")]
    pub post_only: bool,
    #[serde(rename = "last_update_timestamp")]
    pub last_update_timestamp: f64,
    pub replaced: bool,
    pub web: bool,
    pub api: bool,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    pub direction: String,
    pub mmp: bool,
    pub amount: f64,
    pub price: f64,
    pub label: String,
}