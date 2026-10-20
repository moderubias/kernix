//! Allocation instrumentation is compiled only into a separate profiling binary.
#[cfg(feature = "allocation-counting")]
mod enabled {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicUsize, Ordering::Relaxed};
    pub struct Counter;
    static ACTIVE: AtomicBool = AtomicBool::new(false);
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    static REALLOCS: AtomicUsize = AtomicUsize::new(0);
    static BYTES: AtomicUsize = AtomicUsize::new(0);
    static LIVE: AtomicIsize = AtomicIsize::new(0);
    static PEAK: AtomicUsize = AtomicUsize::new(0);
    fn allocated(size: usize) {
        BYTES.fetch_add(size, Relaxed);
        let live = LIVE.fetch_add(size as isize, Relaxed) + size as isize;
        PEAK.fetch_max(live.max(0) as usize, Relaxed);
    }
    // SAFETY: every request is forwarded to System with its original pointer/layout;
    // counters never access user memory and change no allocator ownership semantics.
    unsafe impl GlobalAlloc for Counter {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let p = unsafe { System.alloc(layout) };
            if !p.is_null() && ACTIVE.load(Relaxed) {
                CALLS.fetch_add(1, Relaxed);
                allocated(layout.size());
            }
            p
        }
        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            let p = unsafe { System.alloc_zeroed(layout) };
            if !p.is_null() && ACTIVE.load(Relaxed) {
                CALLS.fetch_add(1, Relaxed);
                allocated(layout.size());
            }
            p
        }
        unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
            if ACTIVE.load(Relaxed) {
                LIVE.fetch_sub(layout.size() as isize, Relaxed);
            }
            unsafe { System.dealloc(p, layout) };
        }
        unsafe fn realloc(&self, p: *mut u8, layout: Layout, size: usize) -> *mut u8 {
            let out = unsafe { System.realloc(p, layout, size) };
            if !out.is_null() && ACTIVE.load(Relaxed) {
                REALLOCS.fetch_add(1, Relaxed);
                LIVE.fetch_sub(layout.size() as isize, Relaxed);
                allocated(size);
            }
            out
        }
    }
    pub fn count(operation: impl FnOnce()) -> serde_json::Value {
        for counter in [&CALLS, &REALLOCS, &BYTES, &PEAK] {
            counter.store(0, Relaxed);
        }
        LIVE.store(0, Relaxed);
        assert!(!ACTIVE.swap(true, Relaxed));
        struct Guard;
        impl Drop for Guard {
            fn drop(&mut self) {
                ACTIVE.store(false, Relaxed);
            }
        }
        let guard = Guard;
        operation();
        drop(guard);
        serde_json::json!({"allocation_calls":CALLS.load(Relaxed),"reallocation_calls":REALLOCS.load(Relaxed),
            "requested_bytes":BYTES.load(Relaxed),"peak_live_requested_bytes":PEAK.load(Relaxed),
            "live_requested_byte_delta_after_drop":LIVE.load(Relaxed)})
    }
}
#[cfg(feature = "allocation-counting")]
#[global_allocator]
static ALLOCATOR: enabled::Counter = enabled::Counter;

pub fn count(operation: impl FnOnce()) -> serde_json::Value {
    #[cfg(feature = "allocation-counting")]
    {
        enabled::count(operation)
    }
    #[cfg(not(feature = "allocation-counting"))]
    {
        let _ = operation;
        panic!("resources require the allocation-counting binary")
    }
}

#[cfg(all(test, feature = "allocation-counting"))]
mod tests {
    #[test]
    fn allocator_probe_counts_owned_output_and_reallocation() {
        let result = super::count(|| {
            let mut values = Vec::<u8>::with_capacity(64);
            std::hint::black_box(&mut values);
            values.reserve(128);
            std::hint::black_box(&mut values);
            drop(values);
        });
        assert_eq!(result["allocation_calls"], 1);
        assert_eq!(result["reallocation_calls"], 1);
        assert!(result["requested_bytes"].as_u64().unwrap() >= 192);
        assert!(result["peak_live_requested_bytes"].as_u64().unwrap() >= 128);
        assert_eq!(result["live_requested_byte_delta_after_drop"], 0);
    }
}
