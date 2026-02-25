mod modules;

use modules::lifetime::lifetime;
use modules::std::{collection, env, file_system, path, process, sync, thread, time};

fn main() {
    file_system::file_system();
    path::path_manipulation();
    env::env();
    collection::collection();
    thread::threading();
    sync::sync_module();

    lifetime();

    time::time_data();
    process::proses();
}
