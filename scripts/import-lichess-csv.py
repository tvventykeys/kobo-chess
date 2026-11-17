#!/usr/bin/env python3
"""Stream a small CC0 CSV subset into a validated Pocket pack, using its rules."""
import argparse
import csv
import json
import os
from pathlib import Path
import subprocess
import tempfile

parser = argparse.ArgumentParser()
parser.add_argument('csv', type=Path)
parser.add_argument('output', type=Path)
parser.add_argument('--count', type=int, default=20)
parser.add_argument('--min-rating', type=int, default=800)
parser.add_argument('--max-rating', type=int, default=1500)
parser.add_argument('--binary', type=Path, default=Path(os.environ.get('CARGO_TARGET_DIR', '/workspace/build/cobalt')) / 'debug/polgar-pocket')
args = parser.parse_args()
if not 1 <= args.count <= 100:
    parser.error('select 1–100 puzzles')
rows = []
with args.csv.open(newline='') as stream:
    for row in csv.DictReader(stream):
        if args.min_rating <= int(row['Rating']) <= args.max_rating:
            rows.append({'puzzleId': row['PuzzleId'], 'fen': row['FEN'], 'moves': row['Moves'],
                         'rating': int(row['Rating']), 'themes': row['Themes'].split(),
                         'gameUrl': 'https://lichess.org/training/' + row['PuzzleId']})
            if len(rows) == args.count:
                break
if len(rows) != args.count:
    parser.error('not enough matching source puzzles')
with tempfile.TemporaryDirectory(prefix='pocket-import-') as temp:
    selected = Path(temp) / 'selected.json'
    result = Path(temp) / 'pack.jsonl'
    selected.write_text(json.dumps(rows))
    subprocess.run([str(args.binary), '--import-lichess', str(selected), str(result)], check=True)
    lines = [json.loads(s) for s in result.read_text().splitlines()]
    lines[0]['title'] = 'Lichess · Pocket Twenty' if args.count == 20 else 'Lichess · Selected study'
    for puzzle in lines[1:]:
        puzzle['explanation'] = 'Themes: ' + ', '.join(puzzle['tags']) + '.'
    result.write_text(''.join(json.dumps(s, ensure_ascii=False, separators=(',', ':')) + '\n' for s in lines))
    subprocess.run([str(args.binary), '--validate-pack', str(result)], check=True)
    args.output.write_bytes(result.read_bytes())
print(f'Wrote {len(rows)} validated puzzles to {args.output}')
