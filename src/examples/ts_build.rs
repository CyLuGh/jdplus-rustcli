use jdplus_rustcli::communication_manager::CommunicationManager;
use jdplus_rustcli::models::{AggregationType, Frequency};

use chrono::NaiveDate;

#[tokio::main]
async fn main() {
    let manager = CommunicationManager::builder().build();

    let input = vec![(NaiveDate::from_ymd(2022, 1, 1), 1.0),(NaiveDate::from_ymd(2022, 2, 1), 1.0)];
    let result = manager.build_ts_data(input,AggregationType::Sum,Frequency::Yearly,true,true).await;

    println!("{:?}", result);
}