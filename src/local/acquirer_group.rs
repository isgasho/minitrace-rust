// Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

use crate::local::span_line::SPAN_LINE;
use crate::trace::acquirer::AcquirerGroup;

/// Returns registered acquirers from current thread, or `None` if there're no
/// registered acquires.
pub fn merge_registered_local_acquirers(event: &'static str) -> Option<AcquirerGroup> {
    SPAN_LINE.with(|span_line| {
        let mut span_line = span_line.borrow_mut();
        span_line.merge_registered_acquirers(event)
    })
}
