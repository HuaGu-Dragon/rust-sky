use std::{fmt::Display, time::Duration};

use tower_http::trace::OnResponse;
use tracing::info;

#[derive(Debug, Clone, Copy)]
pub struct LatencyLayer;

impl<B> OnResponse<B> for LatencyLayer {
    fn on_response(
        self,
        response: &axum::http::Response<B>,
        latency: std::time::Duration,
        _span: &tracing::Span,
    ) {
        info!(latency = %DurationWrapper(latency), status = %response.status(), "finish processing request");
    }
}

struct DurationWrapper(Duration);

impl Display for DurationWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.as_millis() > 0 {
            write!(f, "{} ms", self.0.as_millis())
        } else {
            write!(f, "{} µs", self.0.as_micros())
        }
    }
}
