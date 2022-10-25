extern crate core;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hyper::http::HeaderValue;
use rust_sm_slim::Payload;
use std::time::Duration;

fn naive(header_val: Option<&HeaderValue>) -> Payload {
    if header_val.is_none() {
        panic!("Header value is none");
    }
    let header_val = header_val.unwrap();
    let val = header_val.to_str();
    if let Err(e) = val {
        panic!("{}", e);
    }
    let val = val.unwrap();
    let commas: Vec<usize> = val.match_indices(",").map(|(i, _)| i).collect();
    if commas.len() > 1 {
        panic!("Too many commas");
    }
    let comma_loc = *commas.get(0).unwrap();
    if comma_loc == 0 || comma_loc == val.len() {
        panic!("values must be non-empty");
    }
    let (sleep_val, resp_size) = val.split_at(comma_loc);
    let sleep_duration_millis: Result<u64, _> = sleep_val.parse();
    if let Err(e) = sleep_duration_millis {
        panic!("{}", e);
    }
    // convert to chars, remove first char, convert back
    let mut resp_size_chars = resp_size.chars();
    resp_size_chars.next();
    let resp_size: Result<u32, _> = resp_size_chars.as_str().parse();
    if let Err(e) = resp_size {
        panic!("{}", e);
    }
    Payload {
        response_size: resp_size.unwrap(),
        sleep_time: Duration::from_millis(sleep_duration_millis.unwrap()),
    }
}

fn header_parse_benchmark(c: &mut Criterion) {
    let cust_attr = HeaderValue::from_static("45322,152355");
    let val = Some(&cust_attr);
    let mut group = c.benchmark_group("header-parsing-group");
    group.bench_function("naive", |b| b.iter(|| naive(black_box(val))));
    group.bench_function("from_header", |b| {
        b.iter(|| Payload::from_header(black_box(val)).unwrap())
    });
    group.finish()
}

criterion_group!(benches, header_parse_benchmark);
criterion_main!(benches);
