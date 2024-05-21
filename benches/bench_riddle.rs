use binggan::*;
use rustc_hash::FxHashMap;

// Switch allocators
//#[global_allocator]
//pub static GLOBAL: PeakMemAlloc<jemallocator::Jemalloc> = PeakMemAlloc::new(jemallocator::Jemalloc);

#[global_allocator]
pub static GLOBAL: &PeakMemAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

fn get_unique_numbers(num_elements: u64) -> Vec<u32> {
    let ids: Vec<u32> = (0..num_elements as u32).collect();
    ids
}

fn test_fx_hash_map(input: &[u32]) {
    let mut map = FxHashMap::default();
    for &id in input {
        let hash_map_capacity = map.capacity();
        *map.entry(id).or_insert(0) += 1;
        if map.capacity() != hash_map_capacity {
            // Uncomment to better observe strace calls
            //let hash_map_new_cap = map.capacity();
            //dbg!(hash_map_new_cap);
            //std::thread::sleep(std::time::Duration::from_millis(40));
        }
    }
    black_box(map);
}

fn main() {
    // execute if FAST env is set
    if std::env::var("FAST").is_ok() {
        println!("FAST TRICK START");
        use std::alloc::{alloc_zeroed, dealloc, Layout};
        let num_bytes_str = std::env::var("NUM_BYTES").unwrap_or("4000000".to_string());
        let num_bytes = num_bytes_str.parse::<usize>().unwrap();
        unsafe {
            let layout = Layout::from_size_align(num_bytes, 1).unwrap();
            let ptr = alloc_zeroed(layout);
            dealloc(ptr, layout);
        }
        println!("FAST TRICK END");
    }
    let mut inputs: Vec<(String, Vec<u32>)> = Vec::new();
    inputs.push(("100k num elem".to_string(), get_unique_numbers(100_000)));
    let mut group = InputGroup::new_with_inputs(inputs);
    group.config().set_num_iter(5);
    group.set_name("perf_riddle");
    group.throughput(|input| input.len() * 8);

    group.set_alloc(&GLOBAL);
    group.config().enable_perf();

    group.register("FxHashMap", move |input| test_fx_hash_map(input));
    group.run();
}
