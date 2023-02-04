//! Code utilized and modified from [matklad/cargo-xtask](https://github.com/matklad/cargo-xtask/blob/master/examples/hello-world/xtask/src/main.rs)

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
};

use man::prelude::*;

type DynError = Box<dyn std::error::Error>;
const PKG_NAME: &str = "wavefncollapse";

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn run() -> Result<(), DynError> {
    let task: Option<String> = env::args().nth(1);
    match task.as_deref() {
        Some("dist") => run_dist()?,
        Some("doc") => run_dist_doc()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        r#"xtask 0.1.0
A cargo-xtask automation tool

USAGE:
    cargo xtask [COMMAND]...

ARGS:
    dist            builds application and man pages
    doc             builds rustdoc documentation
"#
    )
}

fn run_dist_doc() -> Result<(), DynError> {
    let _ = fs::remove_dir_all(&dir_docs());

    dist_doc_xtask()?;

    Ok(())
}

/// Removes a directory at this path, after removing all its contents. Use carefully!
fn run_dist() -> Result<(), DynError> {
    let _ = fs::remove_dir_all(&dist_dir());
    fs::create_dir_all(&dist_dir())?;

    dist_binary()?;
    dist_manpage()?;

    Ok(())
}

fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}

fn dir_docs() -> PathBuf {
    project_root().join("docs/")
}

fn dist_doc_xtask() -> Result<(), DynError> {
    let cargo: String = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status: ExitStatus = Command::new(cargo)
        .current_dir(project_root())
        .args(&["doc", "--release", "--no-deps", "--open", "--bin", PKG_NAME])
        .status()?;

    if !status.success() {
        Err("cargo doc failed")?;
    }

    let dst: PathBuf = project_root().join(format!("target/doc").as_str());
    let copy_from = dst.to_string_lossy();
    let copy_to = dir_docs();
    let copy_to = copy_to.to_string_lossy();

    match Command::new("cp")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok()
    {
        true => {
            eprintln!("copying `target/doc` directory to `docs/`");
            let status: ExitStatus = Command::new("cp")
                .args(&["-r", &copy_from, &copy_to])
                .status()?;
            if !status.success() {
                Err("failed to copy to directory with `cp`")?;
            }
        }
        false => {
            eprintln!("no `cp` utility found");
        }
    }

    Ok(())
}

fn dist_binary() -> Result<(), DynError> {
    let cargo: String = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status: ExitStatus = Command::new(cargo)
        .current_dir(project_root())
        .args(&["build", "--release"])
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }

    let dst: PathBuf = project_root().join(format!("target/release/{PKG_NAME}").as_str());

    fs::copy(&dst, dist_dir().join(PKG_NAME))?;

    match Command::new("strip")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok()
    {
        true => {
            eprintln!("stripping the binary");
            let status: ExitStatus = Command::new("strip").arg(&dst).status()?;
            if !status.success() {
                Err("strip failed")?;
            }
        }
        false => {
            eprintln!("no `strip` utility found");
        }
    }

    Ok(())
}

fn dist_manpage() -> Result<(), DynError> {
    let page = Manual::new(PKG_NAME)
        .about("Wave function collapse")
        .render();
    fs::write(
        dist_dir().join(format!("{PKG_NAME}.man")),
        &page.to_string(),
    )?;

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
