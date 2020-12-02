use crate::config;
use crate::proc;
use rebuilderd_common::errors::*;
use std::path::Path;
use std::time::Duration;

pub async fn diffoscope(a: &str, b: &str, settings: &config::Diffoscope) -> Result<String> {
    let mut args: Vec<&str> = settings.args.iter().map(AsRef::as_ref).collect();
    args.push("--");
    args.push(a);
    args.push(b);

    let timeout = settings.timeout.unwrap_or(3600); // 1h

    let opts = proc::Options {
        timeout: Duration::from_secs(timeout),
        limit: settings.max_bytes,
        kill_at_size_limit: true,
    };
    let bin = Path::new("diffoscope");
    let (_success, output) = proc::run(bin, &args, opts).await?;
    Ok(output)
}
