//! This build script is currently half-assed. Cross-compilation + advanced features
//! will likely not work yet.
//!
//! If you update Lua, simply run (manually) `make amalg` first.
//! (plus `find . -type f -name '*.o' -delete` to delete the object files)
//!
//! This will generate the `ljamalg.c` file, which is important for building this library
//! Otherwise you cannot build the library because LuaJIT will complain that `lj_ffdef.h` is missing
//! `lj_ffdef.h` is (probably) generated at build-time, however the Makefile is so unreadable that
//! I cannot see where the file is being generated and the LuaJIT devs simply don't want to provide any
//! answers because they think that `make` is the greatest build system ever and have never heard of cargo.
//!
//! This solution consumes more memory, but at least the library builds.
//!
//! The only thing that has to be adjusted now are the flags.

#![allow(non_snake_case)]

extern crate bindgen;
extern crate gcc;

use gcc::Build;
use std::env;
use std::path::{Path, PathBuf};

const LUA_VERSION_FOLDER_NAME: &str = "LuaJIT-2.0.5";


fn main() {

    // setup paths (valid for LuaJIT 2.0.5)
    let path_src = format!("src/{}", LUA_VERSION_FOLDER_NAME);

    // LuaJIT/dynasm
    // let path_src_dynasm  = format!("{}/dynasm", path_src);
    // LuaJIT/src
    let path_src_src  = format!("{}/src", path_src);
    // LuaJIT/src/jit
    // let path_src_jit    = format!("{}/jit", path_src_src);
    // LuaJIT/src/host
    // let path_src_host    = format!("{}/host", path_src_src);

    // setup compile flags, see LuaJIT/src/Makefile

    let mut HOST_XCFLAGS = Vec::<&str>::new();
    let mut XCFLAGS = Vec::<&str>::new();
    let mut TARGET_ARCH = Vec::<&str>::new();

    let mut c = Build::new();
    c.include(path_src_src.clone());
    c.static_flag(true);

    let profile = env::var("PROFILE").unwrap();

    if env::var("CARGO_FEATURE_FORCE_32_BIT").is_ok() {
        c.flag("-m32");
    }

    if env::var("CARGO_FEATURE_OPTIMIZE_SIZE").is_ok() {
        c.flag("-Os");
    }

    if profile == "debug" {
        c.flag("-g");
    } else {
        c.flag("-O2");
    }

    c.flag("-fomit-frame-pointer");

    // panic if more than one target architecture is defined in the features
    // we have to check the env vars seperately
    let arch_arm         = env::var("CARGO_FEATURE_ARCH_ARM");
    let arch_mips        = env::var("CARGO_FEATURE_ARCH_MIPS");
    let arch_mipsel      = env::var("CARGO_FEATURE_ARCH_MIPSEL");
    let arch_powerpc     = env::var("CARGO_FEATURE_ARCH_POWERPC");
    let arch_x64         = env::var("CARGO_FEATURE_ARCH_X64");
    let arch_x86         = env::var("CARGO_FEATURE_ARCH_X86");
    let arch_dynamic     = env::var("CARGO_FEATURE_ARCH_DYNAMIC");
    let arch_powerpc_spe = env::var("CARGO_FEATURE_ARCH_POWERPC_SPE");

    let compile_arch = {
        let architectures = vec![
            ("arch_arm", arch_arm),
            ("arch_mips", arch_mips),
            ("arch_mipsel", arch_mipsel),
            ("arch_powerpc", arch_powerpc),
            ("arch_x64", arch_x64),
            ("arch_x86", arch_x86),
            ("arch_dynamic", arch_dynamic),
            ("arch_powerpc_spe", arch_powerpc_spe)];

        let matches: Vec<usize> =
        architectures.iter()
            .enumerate()
            .filter(|&(_, a)| a.1.is_ok())
            .map(|(idx, _)| idx)
            .collect();

        let valid_architectures: Vec<&str> = matches.into_iter().map(|a| architectures[a].0).collect();
        let arch_count = valid_architectures.len();

        if arch_count > 1 {
            panic!(format!("

luajit-sys v{}: compile-error:
multiple architectures specified in feature flags, please decide for one
architecture or use `arch_dynamic` for determining the ASM format at runtime:

archtectures_specified:\t{}
architectures:\t{:?}

    ", env!("CARGO_PKG_VERSION"), arch_count, valid_architectures));
        }

        valid_architectures.into_iter().next().unwrap_or("arch_dynamic")
    };

    if compile_arch == "arch_x86" {
        c.flag("-march=i686");
    }

    if env::var("CARGO_FEATURE_SSE").is_ok() {
        if env::var("CARGO_FEATURE_SSE").is_ok() {
            c.flag("-msse2");
        } else {
            c.flag("-msse");
        }
    }

    c.flag("-Wall");

    if env::var("CARGO_FEATURE_EXTRA_WARNINGS").is_ok() {
      c.flag("-Wextra");
      c.flag("-Wdeclaration-after-statement");
      c.flag("-Wredundant-decls");
      c.flag("-Wshadow");
      c.flag("-Wpointer-arith");
    }

    if env::var("CARGO_FEATURE_ENABLE_LUA52_COMPAT").is_ok() {
        XCFLAGS.push("-DLUAJIT_ENABLE_LUA52COMPAT");
    }

    if env::var("CARGO_FEATURE_ENABLE_NUMMODE_1").is_ok() {
        if env::var("CARGO_FEATURE_ENABLE_NUMMODE_2").is_ok() {
            XCFLAGS.push("-DLUAJIT_NUMMODE=2");
        } else {
            XCFLAGS.push("-DLUAJIT_NUMMODE=1");
        }
    }

    if compile_arch != "arch_x64" {
        if env::var("CARGO_FEATURE_USE_SYSMALLOC").is_ok() {
            XCFLAGS.push("-DLUAJIT_USE_SYSMALLOC");
        }
    }

    if env::var("CARGO_FEATURE_USE_VALGRIND").is_ok() {
        XCFLAGS.push("-DLUAJIT_USE_VALGRIND");
    }

    if env::var("CARGO_FEATURE_USE_GDBJIT").is_ok() {
        XCFLAGS.push("-DLUAJIT_USE_GDBJIT");
    }

    if env::var("CARGO_FEATURE_USE_APICHECK").is_ok() {
        XCFLAGS.push("-DLUA_USE_APICHECK");
    }

    if env::var("CARGO_FEATURE_USE_ASSERT").is_ok() {
        XCFLAGS.push("-DLUA_USE_ASSERT");
    }

    if env::var("CARGO_FEATURE_USE_ASSERT").is_ok() {
        XCFLAGS.push("-DLUA_USE_ASSERT");
    }

    if compile_arch == "arch_mipsel" {
        TARGET_ARCH.push("-D__MIPSEL__=1");
    }

    // TODO: only preprocess arch header for architecture detection
    // $(shell $(TARGET_CC) $(TARGET_TCFLAGS) -E lj_arch.h -dM)

    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();

    if host != target {
        if target.contains("windows") {
            HOST_XCFLAGS.push("-malign-double");
            HOST_XCFLAGS.push("-DLUAJIT_OS=LUAJIT_OS_WINDOWS");
        } else if target.contains("linux") {
            HOST_XCFLAGS.push("-DLUAJIT_OS=LUAJIT_OS_LINUX");
        } else if target.contains("darwin") || target.contains ("ios") {
            HOST_XCFLAGS.push("-DLUAJIT_OS=LUAJIT_OS_OSX");
        } else {
            HOST_XCFLAGS.push("-DLUAJIT_OS=LUAJIT_OS_OTHER");
        }
    }

    let lua_target_arch = match compile_arch {
            "arch_x64"         => "-DLUAJIT_TARGET=LUAJIT_ARCH_x64",
            "arch_x86"         => "-DLUAJIT_TARGET=LUAJIT_ARCH_x86",
            "arch_arm"         => "-DLUAJIT_TARGET=LUAJIT_ARCH_arm",
            "arch_powerpc"     => "-DLUAJIT_TARGET=LUAJIT_ARCH_ppc",
            "arch_powerpc_spe" => "-DLUAJIT_TARGET=LUAJIT_ARCH_ppcspe",
            "arch_mipsel"   |
            "arch_mips"        => "-DLUAJIT_TARGET=LUAJIT_ARCH_mips",
            _                  => panic!(format!("unsupported target architecture: {:?}", compile_arch)),
        };

    TARGET_ARCH.push(lua_target_arch);

/*
    let LJCORE_O = vec![

        "lj_gc",        "lj_err",       "lj_char",      "lj_bc",            "lj_obj",
        "lj_str",       "lj_tab",       "lj_func",      "lj_udata",         "lj_meta",      "lj_debug",
        "lj_state",     "lj_dispatch",  "lj_vmevent",   "lj_vmmath",        "lj_strscan",
        "lj_api",       "lj_lex",       "lj_parse",     "lj_bcread",        "lj_bcwrite",   "lj_load",
        "lj_ir",        "lj_opt_mem",   "lj_opt_fold",  "lj_opt_narrow",
        "lj_opt_dce",   "lj_opt_loop",  "lj_opt_split", "lj_opt_sink",
        "lj_mcode",     "lj_snap",      "lj_record",    "lj_crecord",       "lj_ffrecord",
        "lj_asm",       "lj_trace",     "lj_gdbjit",
        "lj_ctype",     "lj_cdata",     "lj_cconv",     "lj_ccall",         "lj_ccallback",
        "lj_carith",    "lj_clib",      "lj_cparse",
        "lj_lib",       "lj_alloc",     "lib_aux",

        /*LJLIB_O*/
        "lib_base",     "lib_math",     "lib_bit",      "lib_string",       "lib_table",
        "lib_io",       "lib_os",       "lib_package",  "lib_debug",        "lib_jit",      "lib_ffi",

        "lib_init",
    ];
*/
    for flag in HOST_XCFLAGS {   c.flag(flag); }
    for flag in XCFLAGS      {   c.flag(flag); }
    for flag in TARGET_ARCH  {   c.flag(flag); }

    // end compile flags
    c.file(format!("{}/ljamalg.c", Path::new(&path_src_src).canonicalize().unwrap().to_string_lossy() ));
    c.compile("libluajit.a");

    // see src/lua.hpp (the C++ wrapper) for the header files
    // TODO: there is a file called luaconf.h, not sure if it's important.
    // It is not listed in the C++ header so I thought it's not important for actual functionality
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::Stable_1_19)
        .header(format!("{}/lua.h", path_src_src))
        .header(format!("{}/lauxlib.h", path_src_src))
        .header(format!("{}/lualib.h", path_src_src))
        .header(format!("{}/luajit.h", path_src_src))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings!");
}
