use near_metrics::{try_create_histogram, try_create_histogram_vec, Histogram, HistogramVec};
use once_cell::sync::Lazy;

pub(crate) static HANDLE_MESSAGE_TIME: Lazy<Histogram> = Lazy::new(|| {
    try_create_histogram(
        "indexer_for_explorer_handle_message_time",
        "Latency of handling a streamer message",
    )
    .unwrap()
});

pub(crate) static STORE_TIME: Lazy<HistogramVec> = Lazy::new(|| {
    try_create_histogram_vec(
        "indexer_for_explorer_store_time",
        "Latency of storing an object in the DB",
        &["object"],
        Some(vec![
            50., 100., 300., 500., 700., 800., 900., 950., 1000., 1050., 1100., 1150., 1200.,
            1250., 1300.,
        ]),
    )
    .unwrap()
});