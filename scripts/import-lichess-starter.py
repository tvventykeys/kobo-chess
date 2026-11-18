#!/usr/bin/env python3
"""Select an offline CC0 Lichess CSV subset and validate it with Pocket's rules."""
import argparse
import csv
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
SOURCE = "https://database.lichess.org/#puzzles"
MIN_RATING = 80
MAX_RATING = 1500


def import_pack(args):
    if args.csv.resolve() == args.output.resolve():
        raise ValueError("Source and output must be different files")
    with (ROOT / "packs/lichess-20-source.csv").open(newline="") as demo:
        excluded = {row["PuzzleId"] for row in csv.DictReader(demo)}
    with tempfile.TemporaryDirectory(prefix=".lichess-import-", dir=args.output.parent) as temp:
        rows_path = Path(temp) / "rows.jsonl"
        output = Path(temp) / "starter.idx"
        payload = Path(temp) / "payload.jsonl"
        digest = hashlib.sha256()
        count = 0
        csv.field_size_limit(8192)
        with args.csv.open("rb") as source, payload.open("w", encoding="utf-8", newline="\n") as rows:
            def input_lines():
                for line in source:
                    digest.update(line)
                    yield line.decode("utf-8")

            reader = csv.DictReader(input_lines())
            required = {"PuzzleId", "FEN", "Moves", "Rating", "Themes"}
            fields = reader.fieldnames or []
            if len(set(fields)) != len(fields) or not required.issubset(fields):
                raise ValueError("Expected unique Lichess puzzle CSV columns")
            for row in reader:
                if None in row or any(row[key] is None for key in required):
                    raise ValueError(f"Malformed CSV record ending on line {reader.line_num}")
                rating = int(row["Rating"])
                puzzle_id = row["PuzzleId"]
                if not MIN_RATING <= rating <= MAX_RATING or puzzle_id in excluded:
                    continue
                print(json.dumps([puzzle_id, row["FEN"], row["Moves"], rating, row["Themes"].split()],
                                 separators=(",", ":")), file=rows)
                count += 1
                if count == args.count:
                    break
        if count != args.count:
            raise ValueError(f"Found {count} eligible puzzles; wanted {args.count}")
        source_hash = digest.hexdigest()
        policy = (
            f"First {count} rows rated {MIN_RATING}–{MAX_RATING} inclusive, both solver sides. "
            "Demo IDs excluded. Sorted by puzzle ID. Input digest covers CSV prefix consumed."
        )
        with rows_path.open("wb") as rows, payload.open("rb") as staged:
            header = dict(source=SOURCE, license="CC0-1.0", input_sha256=source_hash, selection=policy)
            rows.write((json.dumps(header) + "\n").encode())
            shutil.copyfileobj(staged, rows)
        subprocess.run([str(args.binary), "--build-lichess-index", rows_path, output], check=True)
        subprocess.run([str(args.binary), "--validate-lichess-index", output], check=True)
        output.replace(args.output)
    print(f"Wrote {count} puzzles rated {MIN_RATING}–{MAX_RATING}, input prefix SHA-256 {source_hash}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("csv", type=Path, help="local UTF-8 Lichess puzzle export, decompressed first")
    parser.add_argument("output", type=Path)
    parser.add_argument("--count", type=int, default=20_000)
    parser.add_argument("--binary", type=Path, default=Path(
        os.environ.get("CARGO_TARGET_DIR", "/workspace/build/cobalt")) / "debug/polgar-pocket")
    args = parser.parse_args()
    if not 1 <= args.count <= 20_000:
        parser.error("--count must be between 1 and 20000")
    try:
        import_pack(args)
    except (ValueError, OSError, csv.Error, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Import failed: {error}\n")


if __name__ == "__main__":
    main()
