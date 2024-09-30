#![no_std]

use crate::time::get_system_time_milliseconds;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

pub struct Wait {
    target_time: u64,
}

impl Wait {
    pub fn new(ms: u64) -> Self {
        let current_time = get_system_time_milliseconds();
        Self {
            target_time: current_time + ms,
        }
    }

    // Synchronous wait method
    pub fn swait(&self) {
        while get_system_time_milliseconds() < self.target_time {}
    }
}

impl Future for Wait {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let current_time = get_system_time_milliseconds();
        if current_time >= self.target_time {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

// Returns a Wait struct, allowing the user to choose swait or await
pub fn wait(ms: u64) -> Wait {
    Wait::new(ms)
}
