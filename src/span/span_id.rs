// Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

use std::cell::Cell;
use std::sync::atomic::{AtomicU16, Ordering};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct SpanId(pub u32);

impl SpanId {
    pub fn new(id: u32) -> Self {
        SpanId(id)
    }
}

pub struct DefaultIdGenerator;

static NEXT_SNOWFLAKE_ID_PREFIX: AtomicU16 = AtomicU16::new(0);
fn next_snowflake_id_prefix() -> u16 {
    NEXT_SNOWFLAKE_ID_PREFIX.fetch_add(1, Ordering::AcqRel)
}

thread_local! {
    static SNOWFLACK_ID_GENERATOR: Cell<(u16, u16)> = Cell::new((next_snowflake_id_prefix(), 0))
}

impl DefaultIdGenerator {
    #[inline]
    pub fn next_id() -> SpanId {
        SNOWFLACK_ID_GENERATOR.with(|g| {
            let (mut prefix, mut suffix) = g.get();

            if suffix == std::u16::MAX {
                suffix = 0;
                prefix = next_snowflake_id_prefix();
            }
            suffix += 1;

            g.set((prefix, suffix));

            SpanId::new(((prefix as u32) << 16) | (suffix as u32))
        })
    }
}
