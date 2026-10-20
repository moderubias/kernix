#!/usr/bin/env python3
"""Check numerical exports and Tape methods against the reviewed VJP inventory."""
import argparse
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / 'docs/OPERATION-SUPPORT.json'
REPORT = ROOT / 'docs/AUTOGRAD-COVERAGE.md'
CONTROL = {'new', 'len', 'is_empty', 'clear', 'variable', 'constant', 'value',
           'detach', 'backward', 'backward_with'}


def code_only(source):
    # This checks the project's explicit export convention, not arbitrary Rust syntax.
    pattern = r'//[^\n]*|/\*[\s\S]*?\*/|r(#+)?"[\s\S]*?"\1|"(?:\\.|[^"\\])*"'
    return re.sub(pattern, lambda m: '\n' * m[0].count('\n'), source)


def methods(source, typename):
    source = code_only(source)
    found = set()
    for match in re.finditer(r'\bimpl\b[^{};]*\b' + typename + r'\b[^{};]*\{', source):
        start = match.end()
        depth, end = 1, start
        while depth and end < len(source):
            depth += (source[end] == '{') - (source[end] == '}')
            end += 1
        if depth:
            raise ValueError('unclosed impl ' + typename)
        body = source[start:end - 1]
        found.update(re.findall(r'\bpub\s+fn\s+(\w+)', body))
        found.update(re.findall(r'\b(?:binary|unary)!\(\s*(\w+)', body))
    return found


def discover(root=ROOT):
    ops = root / 'crates/kernix-ops/src'
    exports = code_only((ops / 'ops.rs').read_text(encoding='utf-8'))
    if re.search(r'pub use[^;]*::\*', exports):
        raise ValueError('numerical inventory requires explicit exports, not wildcard reexports')
    names = set(re.findall(r'^pub fn\s+(\w+)', exports, re.M))
    names.update(re.findall(r'^pub fn\s+(\w+)', code_only((ops / 'lib.rs').read_text(encoding='utf-8')), re.M))
    for match in re.finditer(r'pub use (\w+)::\{([^}]+)\};', exports):
        module, values = match.groups()
        body = code_only((ops / 'ops' / (module + '.rs')).read_text(encoding='utf-8'))
        declarations = set(re.findall(r'\bpub\s+fn\s+(\w+)', body))
        declarations.update(re.findall(
            r'^(?:binary_op|comparison|assign_op|checked_binary|float_unary)!\(\s*(\w+)',
            body, re.M))
        for name in re.findall(r'\b[a-z][a-z_0-9]*\b', values):
            if name not in declarations:
                raise ValueError(f'unrecognized function export: {module}::{name}')
            names.add(name)
    for module in ['neural', 'linalg']:
        body = (ops / (module + '.rs')).read_text(encoding='utf-8')
        names.update(module + '::' + n for n in re.findall(r'^pub fn\s+(\w+)', code_only(body), re.M))
    for typename in ['Lu', 'Cholesky', 'Qr']:
        names.update(f'linalg::{typename}::{n}' for n in methods((ops / 'linalg.rs').read_text(encoding='utf-8'), typename))
    names.update('vision::RgbFrame::' + n for n in methods((ops / 'vision.rs').read_text(encoding='utf-8'), 'RgbFrame'))
    names.update(['Tensor::reshape_view', 'Tensor::permute', 'Tensor::slice_axis'])
    tape = methods((root / 'crates/kernix/src/autograd.rs').read_text(encoding='utf-8'), 'Tape') - CONTROL
    return names, tape


def render(data):
    lines = [f"# Forward and reverse-mode coverage ({data['series']} series)", '',
             'Generated from `OPERATION-SUPPORT.json`; run `python tools/check_operation_support.py`.', '',
             'Scope: exported CPU numerical free functions, dense factor methods, RGB frame',
             'methods and the Tensor reshape/permutation/slice counterparts of Tape operations.',
             'Shape/layout/storage lifecycle APIs, configuration, optimizers and checkpoints',
             'are outside this numerical inventory. Every public Tape method is classified;',
             'control/lifetime/backward methods are explicitly excluded from recording operations.', '',
             '`supported` means a first-order Tape VJP for the documented domain; `partial`',
             'means a narrower domain than the forward API. `unsupported` means no Tape entry',
             'point, including free explicit backward helpers (no higher-order recording).',
             '`not_applicable` identifies inspection/construction or discrete-data helpers.',
             'No operation silently detaches: unsupported recording requires explicit caller code.', '',
             '| Forward operation | VJP | Tape method | Contract / limitation |',
             '|---|---|---|---|']
    for row in data['operations']:
        tape = ', '.join('`' + x + '`' for x in row['tape']) or '—'
        lines.append(f"| `{row['operation']}` | {row['vjp']} | {tape} | {row['contract']} |")
    lines += ['', 'The checker detects missing/new exports, stale Tape method names, duplicate rows,',
              'unclassified recording methods and missing regression-test evidence. It does not',
              'prove derivative correctness; numerical tests and explicit domain review do that.', '']
    return '\n'.join(lines)


def validate(data, names, tape, root=ROOT):
    manifest = (root / 'Cargo.toml').read_text(encoding='utf-8')
    version = re.search(r'^version\s*=\s*"([^"]+)"', manifest, re.M).group(1)
    if data['series'] != '.'.join(version.split('.')[:2]):
        raise ValueError('coverage series does not match the workspace version')
    for path in ['README.md', 'crates/kernix/README.md']:
        if f'kernix = "{version}"' not in (root / path).read_text(encoding='utf-8'):
            raise ValueError('installation version drift in ' + path)
    rows = data['operations']
    keys = [r['operation'] for r in rows]
    if len(keys) != len(set(keys)):
        raise ValueError('duplicate operations')
    if set(keys) != names:
        raise ValueError(f'inventory drift: missing={sorted(names-set(keys))}, stale={sorted(set(keys)-names)}')
    recorded = set()
    for row in rows:
        if row['vjp'] not in {'supported', 'partial', 'unsupported', 'not_applicable'}:
            raise ValueError('invalid VJP status')
        if not row['contract'] or bool(row['tape']) != (row['vjp'] in {'supported', 'partial'}):
            raise ValueError('invalid recording contract: ' + row['operation'])
        recorded.update(row['tape'])
        if row['tape'] and not row['tests']:
            raise ValueError('missing VJP test evidence')
        for evidence in row['tests']:
            path, name = evidence.split('::')
            if not re.search(r'\bfn\s+' + re.escape(name) + r'\b', (root / path).read_text(encoding='utf-8')):
                raise ValueError('missing test: ' + evidence)
    if recorded != tape:
        raise ValueError(f'Tape drift: missing={sorted(tape-recorded)}, stale={sorted(recorded-tape)}')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true', help='regenerate the Markdown table after reviewed JSON edits')
    args = parser.parse_args()
    data = json.loads(DATA.read_text(encoding='utf-8'))
    names, tape = discover()
    validate(data, names, tape)
    output = render(data)
    if args.write:
        REPORT.write_text(output, encoding='utf-8')
    elif REPORT.read_text(encoding='utf-8') != output:
        raise ValueError('coverage Markdown is stale; run with --write')
    print(f'{len(names)} numerical entries; {len(tape)} recording methods classified')


if __name__ == '__main__':
    main()
