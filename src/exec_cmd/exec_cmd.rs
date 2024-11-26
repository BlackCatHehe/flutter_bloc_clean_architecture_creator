use anyhow::{anyhow, Ok, Result};
use std::process::Command;

#[allow(dead_code)]
pub fn exec(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .expect("exec command error");
    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("exit '{}' error", cmd))
    }
}
