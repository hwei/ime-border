from pathlib import Path
import unittest


REPO_ROOT = Path(__file__).resolve().parents[1]


class PackageLayoutTests(unittest.TestCase):
    def test_expected_files_exist(self) -> None:
        required = [
            REPO_ROOT / "openspec" / "project.md",
            REPO_ROOT / "openspec" / "config.yaml",
            REPO_ROOT / "openspec" / "specs" / "formal-cli-contract" / "spec.md",
            REPO_ROOT / "openspec" / "specs" / "repository-governance" / "spec.md",
            REPO_ROOT / "tools" / "new_openspec_change.py",
            REPO_ROOT / "tools" / "openspec_backlog.py",
            REPO_ROOT / "cli" / "python" / "ime_control" / "cli.py",
        ]
        for path in required:
            self.assertTrue(path.exists(), f"Missing expected path: {path}")
