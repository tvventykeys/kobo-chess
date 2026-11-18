#!/usr/bin/env python3
"""Select an offline CC0 Lichess CSV subset and validate it with Pocket's rules."""
import argparse
import csv
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
SOURCE = "https://database.lichess.org/#puzzles"
BANDS = ((1, 1199), (1200, 1599), (1600, 1999), (2000, 65535))


def import_pack(args):
    if args.csv.resolve() == args.output.resolve():
        raise ValueError("Source and output must be different files")
    with (ROOT / "packs/lichess-20-source.csv").open(newline="") as demo:
        excluded = {row["PuzzleId"] for row in csv.DictReader(demo)}
    with args.csv.open("rb") as stream:
        source_hash = hashlib.file_digest(stream, "sha256").hexdigest()

    with tempfile.TemporaryDirectory(prefix=".lichess-import-", dir=args.output.parent) as temp:
        rows_path = Path(temp) / "rows.jsonl"
        output = Path(temp) / "starter.idx"
        counts = [0] * len(BANDS)
        csv.field_size_limit(8192)
        with args.csv.open(encoding="utf-8", newline="") as source, rows_path.open(
            "w", encoding="utf-8", newline="\n"
        ) as rows:
            reader = csv.DictReader(source)
            required = {"PuzzleId", "FEN", "Moves", "Rating", "Themes"}
            fields = reader.fieldnames or []
            if len(set(fields)) != len(fields) or not required.issubset(fields):
                raise ValueError("Expected unique Lichess puzzle CSV columns")
            policy = f"First {args.per_band} rows per inclusive rating band {BANDS}; demo IDs excluded; order by puzzle ID"
            print(json.dumps(dict(source=SOURCE, license="CC0-1.0", input_sha256=source_hash,
                                  selection=policy)), file=rows)
            for row in reader:
                if None in row or any(row[key] is None for key in required):
                    raise ValueError(f"Malformed CSV record ending on line {reader.line_num}")
                puzzle_id = row["PuzzleId"]
                if puzzle_id in excluded:
                    continue
                rating = int(row["Rating"])
                if not 1 <= rating <= 65535:
                    raise ValueError(f"Invalid rating on CSV line {reader.line_num}")
                band = next(i for i, (low, high) in enumerate(BANDS) if low <= rating <= high)
                if counts[band] == args.per_band:
                    continue
                print(json.dumps([puzzle_id, row["FEN"], row["Moves"], rating, row["Themes"].split()],
                                 separators=(",", ":")), file=rows)
                counts[band] += 1
                if all(n == args.per_band for n in counts):
                    break
        if any(n != args.per_band for n in counts):
            raise ValueError(f"Insufficient rows per rating band: {counts}; wanted {args.per_band} each")
        subprocess.run([str(args.binary), "--build-lichess-index", rows_path, output], check=True)
        subprocess.run([str(args.binary), "--validate-lichess-index", output], check=True)
        output.replace(args.output)
    print(f"Wrote {sum(counts)} puzzles ({counts} by band), input SHA-256 {source_hash}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("csv", type=Path, help="local UTF-8 Lichess puzzle export, decompressed first")
    parser.add_argument("output", type=Path)
    parser.add_argument("--per-band", type=int, default=5000)
    parser.add_argument("--binary", type=Path, default=Path(
        os.environ.get("CARGO_TARGET_DIR", "/workspace/build/cobalt")) / "debug/polgar-pocket")
    args = parser.parse_args()
    if not 1 <= args.per_band <= 5000:
        parser.error("--per-band must be between 1 and 5000")
    try:
        import_pack(args)
    except (ValueError, OSError, csv.Error, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Import failed: {error}\n")


if __name__ == "__main__":
    main()
