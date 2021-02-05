use std::process::Command;
use std::path::Path;
use anyhow::Result;

#[allow(dead_code)]
pub fn commits_count<P: AsRef<Path>>(repo_path: P, revision: &str) -> Result<usize> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .arg("rev-list")
        .arg(revision)
        .arg("--count")
        .output()?;

    let out = std::str::from_utf8(&output.stdout)?
        .trim()
        .parse::<usize>()?;
    Ok(out)
}

#[allow(dead_code)]
pub fn commit_associated_branches<P: AsRef<Path>>(repo_path: P, revision: &str) -> Result<Vec<String>> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .arg("branch")
        .arg("--contains")
        .arg(revision)
        .output()?;

    let out = std::str::from_utf8(&output.stdout)?
        .split_whitespace()
        .filter(|s| s != &"*")
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();
    
    Ok(out)
}

pub fn repo_size<P: AsRef<Path>>(repo_path: P) -> Result<usize> {
    // statCount        = "count:"
	// statSize         = "size:"
	// statInpack       = "in-pack:"
	// statPacks        = "packs:"
	// statSizePack     = "size-pack:"
	// statPrunePackage = "prune-package:"
	// statGarbage      = "garbage:"
	// statSizeGarbage  = "size-garbage:"
    let output = Command::new("git")
        .current_dir(repo_path)
        .arg("count-objects")
        .arg("-v")
        .output()?;

    let size = std::str::from_utf8(&output.stdout)?
        .split("\n")
        .filter(|s| s.contains("size:") || s.contains("size-pack:"))
        .map(|s| s.split(":").last().unwrap().trim())
        .map(|v| v.parse::<usize>().unwrap())
        .sum();
    
    Ok(size)
}