use tokio::runtime::{Builder, Runtime};

pub fn get_rt(workers: usize) -> Runtime {
    let mut builder = Builder::new_multi_thread();
    builder.enable_all();
    
    if workers > 0 {
        builder.worker_threads(workers);
    }
    
    builder.build().expect("Failed to build runtime")
}