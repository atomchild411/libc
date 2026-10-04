#!/usr/bin/env python3
"""Rust extern declarations for libc's IRIX module, from clang's AST of the IRIX headers as our
toolchain sees them (wrappers included): usage gen_fns.py ast.json names.txt known_types.txt"""
import json, re, sys

ast = json.load(open(sys.argv[1]))
want = [l.strip() for l in open(sys.argv[2]) if l.strip()]
known = set(l.strip() for l in open(sys.argv[3]) if l.strip())

# Parameter names: IRIX's prototypes have none, so borrow libc's own for the same function and arity.
import glob as _glob
pnames = {}
if len(sys.argv) > 4:
    for f in _glob.glob(sys.argv[4] + '/**/*.rs', recursive=True):
        txt = open(f, errors='replace').read()
        for m in re.finditer(r'pub fn (\w+)\(([^)]*)\)', txt):
            names = [p.split(':')[0].strip() for p in m.group(2).split(',') if ':' in p]
            pnames.setdefault((m.group(1), len(names)), names)

decls = {}
for d in ast.get('inner', []):
    if d.get('kind') != 'FunctionDecl' or d.get('name') not in want or d.get('isImplicit'):
        continue
    decls.setdefault(d['name'], []).append(d)

BASE = {
    'int': 'c_int', 'signed int': 'c_int', 'unsigned int': 'c_uint', 'unsigned': 'c_uint',
    'long': 'c_long', 'long int': 'c_long', 'unsigned long': 'c_ulong', 'unsigned long int': 'c_ulong',
    'short': 'c_short', 'unsigned short': 'c_ushort', 'char': 'c_char', 'signed char': 'c_schar',
    'unsigned char': 'c_uchar', 'long long': 'c_longlong', 'unsigned long long': 'c_ulonglong',
    'double': 'c_double', 'float': 'c_float', 'void': 'c_void', '_Bool': 'bool',
}
PRIM = set(BASE.values()) | {'size_t', 'ssize_t'}
missing_types = {}

def rty(t, name, ret=False):
    t = t.strip()
    t = re.sub(r'\b(__restrict|restrict|volatile)\b', '', t)
    t = re.sub(r'\s+', ' ', t).strip()
    if '(*)' in t or '(' in t:
        raise ValueError('function pointer: ' + t)
    m = re.match(r'^(.*?)\s*\[(\d*)\]$', t)
    if m:  # array parameter decays to a pointer
        t = m.group(1) + ' *'
    stars = t.count('*')
    parts = [p.strip() for p in t.split('*')]
    base = parts[0]
    consts = [ 'const' in p.split() for p in parts]  # consts[0]: base const; consts[i]: ith pointer const
    base = re.sub(r'\bconst\b', '', base).strip()
    base = {'timespec_t': 'struct timespec', 'sigevent_t': 'struct sigevent', 'idtype_t': 'enum idtype'}.get(base, base)
    base = re.sub(r'^(struct|union|enum) ', lambda m: m.group(0), base)
    if base in BASE:
        r = BASE[base]
    elif base.startswith('enum '):
        r = 'c_int'
    elif base.startswith(('struct ', 'union ')):
        n = base.split(' ', 1)[1]
        r = 'crate::' + n
        if n not in known:
            missing_types.setdefault(n, []).append(name)
    else:
        r = base if base in PRIM else 'crate::' + base
        if base not in known and base not in PRIM:
            missing_types.setdefault(base, []).append(name)
    if stars == 0:
        if r == 'c_void':
            return None if ret else r
        return r
    # innermost pointer gets the base's constness
    s = r
    for i in range(stars):
        c = consts[0] if i == 0 else consts[i]
        s = ('*const ' if c else '*mut ') + s
    return s

out, skipped = [], []
for n in want:
    ds = decls.get(n)
    if not ds:
        skipped.append((n, 'no declaration'))
        continue
    d = ds[-1]
    if any(x.get('storageClass') == 'static' or x.get('inline') for x in ds):
        skipped.append((n, 'inline/static in the headers'))
        continue
    label = None
    for x in ds:
        for a in x.get('inner', []):
            if a.get('kind') == 'AsmLabelAttr':
                label = a.get('label')
    qt = d['type']['qualType']
    variadic = qt.rstrip().endswith('...)')
    params = [a for a in d.get('inner', []) if a.get('kind') == 'ParmVarDecl']
    try:
        ret = rty(qt[:qt.index('(')], n, ret=True)
        ps = []
        for i, p in enumerate(params):
            borrowed = pnames.get((n, len(params)))
            pn = p.get('name') or (borrowed[i] if borrowed else f'arg{i}')
            if pn in ('type', 'fn', 'ref', 'mod', 'in', 'match', 'loop', 'move', 'self', 'impl', 'use', 'where', 'box', 'priv'):
                pn += '_'
            ps.append(f'{pn}: {rty(p["type"]["qualType"], n)}')
        if variadic:
            ps.append('...')
    except ValueError as e:
        skipped.append((n, str(e)))
        continue
    line = ''
    if label:
        line += f'    #[link_name = "{label}"]\n'
    sig = f'    pub fn {n}({", ".join(ps)})' + (f' -> {ret}' if ret else '') + ';'
    if len(sig) > 100:
        sig = f'    pub fn {n}(\n' + ''.join(f'        {p},\n' for p in ps) + '    )' + (f' -> {ret}' if ret else '') + ';'
    out.append(line + sig)

print('\n'.join(out))
sys.stderr.write('SKIPPED\n' + ''.join(f'  {n}: {why}\n' for n, why in skipped))
sys.stderr.write('MISSING TYPES\n' + ''.join(f'  {t}: {", ".join(sorted(set(v)))}\n' for t, v in sorted(missing_types.items())))
