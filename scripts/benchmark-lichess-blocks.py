#!/usr/bin/env python3
"""Measure the proposed bounded-read archive using a validated PCSIDX01 sample."""
import argparse
import bisect
from collections import Counter
import json
import math
from pathlib import Path
import random
import statistics
import struct
import tempfile
import time
import zlib

ENTRY = struct.Struct("<5sIHB")
BLOCK = struct.Struct("<5s5sIIIH")
MAX_ROWS = 256
MAX_RAW = 65_535
SHARD_BLOCKS = 64
FACET = struct.Struct("<HB")


def inspect(index_path, target):
    data = index_path.read_bytes()
    if data[:8] != b"PCSIDX01":
        raise ValueError("Expected a validated PCSIDX01 file")
    metadata, count = struct.unpack_from("<II", data, 8)
    start = 16 + metadata
    payload = memoryview(data)[start + count * ENTRY.size:]
    ids, first_ids, offsets = [], [], []
    facets = bytearray()
    sides = Counter()
    with tempfile.TemporaryFile() as archive:
        rows = []
        size = offset = max_raw = max_compressed = 0

        def flush_block():
            nonlocal size, max_raw, max_compressed
            first_ids.append(rows[0][0])
            raw = b"\n".join(record for _, record in rows)
            compressed = zlib.compress(raw, level=6)
            entry = BLOCK.pack(rows[0][0], rows[-1][0], archive.tell(), len(compressed), len(raw), len(rows))
            offsets.append(entry)
            archive.write(compressed)
            max_raw = max(max_raw, len(raw))
            max_compressed = max(max_compressed, len(compressed))
            rows.clear()
            size = 0

        for puzzle_id, end, rating, mask in ENTRY.iter_unpack(data[start:start + count * ENTRY.size]):
            record = bytes(payload[offset:end])
            row = json.loads(record)
            side = {"b": "White", "w": "Black"}[row[1].split()[1]]
            assert 80 <= rating <= 1500 and puzzle_id == row[0].encode()
            sides[side] += 1
            facets += FACET.pack(rating, mask | (int(side == "Black") << 5))
            ids.append(puzzle_id)
            if rows and (len(rows) == MAX_ROWS or size + len(record) + 1 > MAX_RAW):
                flush_block()
            rows.append((puzzle_id, record))
            size += len(record) + 1
            offset = end
        if rows:
            flush_block()
        assert sides["White"] and sides["Black"]
        compressed_bytes = archive.tell()
        archive.flush()
        samples = []
        for puzzle_id in random.Random(42).sample(ids, min(1000, count)):
            before = time.perf_counter_ns()
            block = bisect.bisect_right(first_ids, puzzle_id) - 1
            _, _, offset, length, raw_length, records = BLOCK.unpack(offsets[block])
            archive.seek(offset)
            raw = zlib.decompress(archive.read(length))
            assert len(raw) == raw_length
            lines = raw.splitlines()
            assert len(lines) == records
            match = next(line for line in lines if line[2:7] == puzzle_id)
            assert json.loads(match)[0].encode() == puzzle_id
            samples.append((time.perf_counter_ns() - before) / 1000)

    projected_blocks = math.ceil(len(offsets) * target / count)
    return dict(
        preview_count=count, solver_sides=sides, index_bytes=len(data),
        blocks=len(offsets), largest_uncompressed_block=max_raw,
        largest_compressed_block=max_compressed, compressed_payload_bytes=compressed_bytes,
        directory_bytes=len(offsets) * BLOCK.size, facet_bytes=len(facets),
        warm_lookup_us_median=round(statistics.median(samples), 1),
        warm_lookup_us_p95=round(sorted(samples)[math.ceil(0.95 * len(samples)) - 1], 1),
        target_count=target, projected_block_count=projected_blocks,
        projected_shard_count=math.ceil(projected_blocks / SHARD_BLOCKS),
        projected_bytes=round(compressed_bytes * target / count) + projected_blocks * BLOCK.size + target * FACET.size,
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("index", type=Path)
    parser.add_argument("--target-count", type=int, default=1_947_528)
    args = parser.parse_args()
    if args.target_count < 1:
        parser.error("target count must be positive")
    print(json.dumps(inspect(args.index, args.target_count), indent=2))


if __name__ == "__main__":
    main()
