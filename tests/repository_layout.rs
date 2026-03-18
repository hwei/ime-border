use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn expected_files_exist() {
    let repo_root = repo_root();
    let required = [
        repo_root.join("openspec").join("project.md"),
        repo_root.join("openspec").join("config.yaml"),
        repo_root
            .join("openspec")
            .join("specs")
            .join("formal-cli-contract")
            .join("spec.md"),
        repo_root
            .join("openspec")
            .join("specs")
            .join("repository-governance")
            .join("spec.md"),
        repo_root.join("tools").join("new_openspec_change.py"),
        repo_root.join("tools").join("openspec_backlog.py"),
        repo_root.join("src").join("main.rs"),
        repo_root.join("Cargo.toml"),
    ];

    for path in required {
        assert!(path.exists(), "Missing expected path: {}", path.display());
    }
}
