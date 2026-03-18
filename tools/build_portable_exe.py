from __future__ import annotations

import subprocess
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
LIBRARY_BIN = REPO_ROOT / ".conda" / "Library" / "bin"


def main() -> int:
    add_binary_args = []
    for dll_name in (
        "ffi.dll",
        "ffi-7.dll",
        "ffi-8.dll",
        "libcrypto-3-x64.dll",
        "liblzma.dll",
        "libbz2.dll",
    ):
        add_binary_args.extend(["--add-binary", f"{LIBRARY_BIN / dll_name};."])
    subprocess.run(
        [
            str(REPO_ROOT / ".conda" / "Scripts" / "pyinstaller.exe"),
            "--onefile",
            "--name",
            "ime-control",
            "--paths",
            str(REPO_ROOT / "cli" / "python"),
            *add_binary_args,
            str(REPO_ROOT / "cli" / "python" / "ime_control_main.py"),
        ],
        check=True,
        cwd=REPO_ROOT,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
