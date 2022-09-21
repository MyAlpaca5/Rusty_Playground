use polars::prelude::*;
use rand;
use std::{thread, time};

use super::core::ResultProducer;

pub fn run_query(mut result_producer: ResultProducer) {
    let start_time = time::Instant::now();

    for _ in 1..=12 {
        let data = some_random_long_computation();
        let data_json = serde_json::to_string(&data).unwrap();
        result_producer.add_result(data_json);
    }

    let end_time = time::Instant::now();
    log::info!("Query Took: {:.2?}", end_time - start_time);
}

fn some_random_long_computation() -> DataFrame {
    let rand_duration = time::Duration::from_secs(rand::random::<u64>() % 3);
    thread::sleep(rand_duration);

    let s1 = Series::new("fruit", &["Apple", "Orange", "Pear"]);
    let s2 = UInt32Chunked::new(
        "price",
        &[rand::random::<u32>(), rand::random(), rand::random()],
    )
    .into_series();

    DataFrame::new(vec![s1, s2]).unwrap()
}
