use actix::prelude::*;
use actix_web::{server, App, HttpRequest};
use analytics::analytics::{Analytics, AnalyticsBuilder, AnalyticsWorker};
use analytics::http::HttpClient;
use analytics::message::{BatchMessage, Track};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

#[derive(Clone)]
struct AppState {
    analytics: Analytics,
}

fn index(req: &HttpRequest<AppState>) -> &'static str {
    req.state().analytics.try_send(BatchMessage::Track(Track {
        user_id: "joe".to_owned(),
        event: "Foo Bar".to_owned(),
    })).unwrap();

    "Hello, world!"
}

fn main() {
    let (analytics, mut analytics_worker) = AnalyticsBuilder::new("BCMWl4ymIAWYrBHi71nv4TtjBOHpeS0a".to_owned()).build();
    let state = AppState { analytics: analytics };

    thread::spawn(move || {
        loop {
            analytics_worker.try_consume().unwrap();
        }
    });

    server::new(move || App::with_state(state.clone()).resource("/", |r| r.f(index)))
        .bind("localhost:8088")
        .unwrap()
        .run();
}
