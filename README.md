# luajit-sys

## About this library

This library provide system bindings to the Lua API. The library is statically linked.
The current version is `2.0.5`. The version number for cargo is `2.0.50`
(as in `2.0.5.0`, but Cargo doesn't allow 4 digits for versioning). The last number is
reserved if the rust version (i.e. this library) has changes. However, this shouldn't
technically happen.

You can update this library for a new version of LuaJIT **HOWEVER**:

If you update Lua, you need to:

    - delete the /docs folder, useless for this
    - delete the /src/
    - run `make amalg` once (manually)
    - run `find . -type f -name '*.o' -delete` to delete the object files

This will generate the `ljamalg.c` file, which is important for building this
library. Otherwise you cannot build the library because LuaJIT will complain
that `lj_ffdef.h` is missing. `lj_ffdef.h` is (probably) generated at build-time,
however the LuaJIT Makefile is so unreadable that I cannot see where the file is
being generated and the LuaJIT devs simply don't want to provide any
answers because they think that `make` is the greatest build system ever and
have never heard of cargo.

This solution consumes more memory, but at least the library builds.

The only thing that has to be adjusted now are the flags. I've tried re-exporting
every LuaJIT compile flag that is possible and make it compatible with Rust,
however this is a work-in-progress.

# cargo features

Choose your target architecture (only one, otherwise it won't compile):

```
arch_arm = []
arch_mips = []
arch_mipsel = []
arch_powerpc = []
arch_powerpc_spe = []
arch_x64 = []
arch_x86 = []
```

The default is `arch_x64`. Then choose your additional flags:

- `force_32_bit`
- `extra_warnings`
- `enable_lua52_compat`
- `disable_jit`
- `nummode_1 | nummode_2`
- `use_sysmalloc`
- `use_valgrind`
- `use_gdb_jit`
- `use_apicheck`
- `use_assert`
- `optimize_size`
- `ps3`
- `cellos_lv2`

See the `Cargo.toml` file for information.

# Compatability with `rlua` and `hlua`

I will try to get `rlua` and `hlua` working. However, this library is currently using
the Lua 5.1 API, while `hlua` uses 5.2 and `rlua` uses 5.3 - I don't know if the APIs
will be compatible.
