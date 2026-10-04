# IRIX declarations generator

`gen_fns.py` writes the generated `extern "C"` block at the end of `src/unix/irix/mod.rs` from
clang's AST of the IRIX headers, compiled the way this target's C toolchain compiles them
(IRIX 6.5.22 headers plus our clang's IRIX header wrappers):

    mipseb-sgi-irix6.5-clang -fsyntax-only -Xclang -ast-dump=json fndecls.c > ast.json
    gen_fns.py ast.json functions.txt known-types.txt path/to/libc/src > fns.rs

`known-types.txt` lists the types the module defines (`pub type`/`pub struct` names in
`src/unix/mod.rs` and `src/unix/irix/mod.rs`). The script reports functions it skipped (inline in
the headers, function-pointer parameters) and types the module still lacks. It keeps asm labels
(`__irix_*` replacements) as `link_name`s and borrows parameter names from libc's other platforms.
