//! Shared test utilities for frencli tests.
//! 
//! Provides common helpers for setting up isolated test environments.

use std::path::{Path, PathBuf};
use std::collections::VecDeque;
use tempfile::TempDir;

/// Helper to recursively copy a directory (synchronous version)
/// This is used internally by setup_test_data_sync
#[allow(dead_code)]
fn copy_dir_all_sync(src: &Path, dst: &Path) -> std::io::Result<()> {
    let mut queue = VecDeque::new();
    queue.push_back((src.to_path_buf(), dst.to_path_buf()));
    
    while let Some((src_path, dst_path)) = queue.pop_front() {
        if src_path.is_dir() {
            std::fs::create_dir_all(&dst_path)?;
            for entry in std::fs::read_dir(&src_path)? {
                let entry = entry?;
                let entry_path = entry.path();
                let entry_dst = dst_path.join(entry_path.file_name().unwrap());
                
                if entry_path.is_dir() {
                    queue.push_back((entry_path, entry_dst));
                } else {
                    std::fs::copy(&entry_path, &entry_dst)?;
                }
            }
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

/// Helper to recursively copy a directory (async version)
/// This is used internally by setup_test_data_async
#[allow(dead_code)]
async fn copy_dir_all_async(src: &Path, dst: &Path) -> std::io::Result<()> {
    use tokio::fs;
    
    let mut queue = VecDeque::new();
    queue.push_back((src.to_path_buf(), dst.to_path_buf()));
    
    while let Some((src_path, dst_path)) = queue.pop_front() {
        if src_path.is_dir() {
            fs::create_dir_all(&dst_path).await?;
            let mut entries = fs::read_dir(&src_path).await?;
            
            while let Some(entry) = entries.next_entry().await? {
                let entry_path = entry.path();
                let entry_dst = dst_path.join(entry_path.file_name().unwrap());
                
                if entry_path.is_dir() {
                    queue.push_back((entry_path, entry_dst));
                } else {
                    fs::copy(&entry_path, &entry_dst).await?;
                }
            }
        } else {
            fs::copy(&src_path, &dst_path).await?;
        }
    }
    Ok(())
}

/// Helper to copy test_data into a temp directory (synchronous version)
#[allow(dead_code)] // Used by integration_tests.rs
pub fn setup_test_data_sync() -> Option<(TempDir, PathBuf)> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_data_src = workspace_root.join("test_data");
    
    if !test_data_src.exists() {
        return None;
    }
    
    let temp_dir = TempDir::new().ok()?;
    let test_data_dst = temp_dir.path().join("test_data");
    copy_dir_all_sync(&test_data_src, &test_data_dst).ok()?;
    
    Some((temp_dir, test_data_dst))
}

/// Helper to copy test_data into a temp directory (async version)
#[allow(dead_code)] // Used by list_tests.rs
pub async fn setup_test_data_async() -> Option<(TempDir, PathBuf)> {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_data_src = crate_root.join("test_data");
    
    if !test_data_src.exists() {
        return None;
    }
    
    let temp_dir = TempDir::new().unwrap();
    let test_data_dst = temp_dir.path().join("test_data");
    copy_dir_all_async(&test_data_src, &test_data_dst).await.ok()?;
    
    Some((temp_dir, test_data_dst))
}

/// RAII guard that restores the current directory when dropped.
/// 
/// Use this only when absolutely necessary (e.g., for functions that read from current directory).
/// Prefer using absolute paths and the working_directory parameter where possible.
#[allow(dead_code)] // Used by audit_tests.rs, undo_tests.rs, rename_tests.rs
pub struct DirGuard {
    original_dir: PathBuf,
}

impl DirGuard {
    #[allow(dead_code)] // Used by audit_tests.rs, undo_tests.rs, rename_tests.rs
    pub fn new(target_dir: &Path) -> Result<Self, std::io::Error> {
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(target_dir)?;
        Ok(DirGuard { original_dir })
    }
}

impl Drop for DirGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.original_dir);
    }
}

