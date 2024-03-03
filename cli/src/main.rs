mod runner;
mod server;
mod runtime;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> anyhow::Result<()> {
    let _ = runner::run()?; // TODO improve error handling
    Ok(())
}
