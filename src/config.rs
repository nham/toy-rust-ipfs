use std::path::PathBuf;

pub const DEFAULT_REPO_ROOT: &'static str = "~/";
pub const DEFAULT_REPO_PATH: &'static str = ".rust-ipfs";
pub const DEFAULT_CONFIG_FILE: &'static str = "config";
pub const ENV_NAME_REPO_DIR: &'static str = "IPFS_PATH";

pub fn repo_path_to_config_file(mut repo_path: PathBuf) -> PathBuf {
    repo_path.push(DEFAULT_CONFIG_FILE);
    repo_path
}
