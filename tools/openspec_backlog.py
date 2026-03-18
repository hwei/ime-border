import argparse
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Optional

from openspec_change_meta import CHANGES_DIR, ChangeMeta, list_non_archived_change_dirs, load_meta

STATUS_ORDER = {"active": 0, "queued": 1, "blocked": 2, "superseded": 3}
PRIORITY_ORDER = {"P0": 0, "P1": 1, "P2": 2}


@dataclass(frozen=True)
class ChangeRecord:
    name: str
    path: Path
    meta: ChangeMeta


def load_change_records(changes_dir: Optional[Path] = None) -> list[ChangeRecord]:
    changes_dir = CHANGES_DIR if changes_dir is None else changes_dir
    records = []
    for change_dir in list_non_archived_change_dirs(changes_dir):
        meta_path = change_dir / "meta.yaml"
        if meta_path.exists():
            records.append(ChangeRecord(change_dir.name, change_dir, load_meta(meta_path)))
    return records


def rank_records(records: list[ChangeRecord]) -> list[ChangeRecord]:
    return sorted(
        records,
        key=lambda record: (
            STATUS_ORDER[record.meta.status],
            PRIORITY_ORDER[record.meta.priority],
            record.name,
        ),
    )


def build_parser():
    parser = argparse.ArgumentParser(description="Filter and rank non-archived OpenSpec changes.")
    subparsers = parser.add_subparsers(dest="command", required=True)
    for name in ("list", "next"):
        subparser = subparsers.add_parser(name)
        subparser.add_argument("--json", action="store_true")
    return parser


def main(argv=None):
    parser = build_parser()
    args = parser.parse_args(argv)
    records = rank_records(load_change_records())
    if args.command == "next":
        records = records[:1]
    if args.json:
        print(
            json.dumps(
                [
                    {
                        "name": record.name,
                        "path": str(record.path),
                        "status": record.meta.status,
                        "priority": record.meta.priority,
                    }
                    for record in records
                ],
                ensure_ascii=False,
                indent=2,
            )
        )
    else:
        for record in records:
            print(record.name)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
