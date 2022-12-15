use std::fmt::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use bindgen::{Bindings, Builder, MacroTypeVariation};

const SUPPORTED_ARCHES: &[SupportedArch] = &[
    SupportedArch {
        linux_name: "arm64",
        rust_name: "aarch64",
        clang_target: "aarch64-arm-linux-gnu",
    },
    SupportedArch {
        linux_name: "x86",
        // Not exactly right, linux bundles x86 and x86_64
        rust_name: "x86_64",
        clang_target: "x86_64-linux-gnu",
    },
];

const GENERATE: &[GenSpec] = &[
    GenSpec {
        headers: &["linux/auxvec.h"],
        allow_vars: &["AT.*"],
        allow_types: &[],
        allow_functions: &[],
    },
    GenSpec {
        headers: &["linux/fcntl.h"],
        allow_vars: &["O_.*"],
        allow_types: &[],
        allow_functions: &[],
    },
    GenSpec {
        headers: &["linux/errno.h"],
        allow_vars: &["E.*"],
        allow_types: &[],
        allow_functions: &[],
    },
    GenSpec {
        headers: &["linux/stat.h"],
        allow_vars: &[],
        allow_types: &["stat.*"],
        allow_functions: &[],
    },
    GenSpec {
        headers: &["linux/signal.h"],
        allow_vars: &["SIG.*"],
        allow_types: &["__sig.*", "sigact.*"],
        allow_functions: &["__sig.*"],
    },
    GenSpec {
        headers: &["linux/poll.h"],
        allow_vars: &["POLL.*"],
        allow_types: &["poll.*"],
        allow_functions: &[],
    },
    GenSpec {
        headers: &["linux/ioctl.h"],
        allow_vars: &["_IO.*"],
        allow_types: &[],
        allow_functions: &[],
    },
    GenSpec {
        headers: &["linux/termios.h"],
        allow_vars: &["TC.*", "TIO.*"],
        allow_types: &["term.*"],
        allow_functions: &[],
    },
    GenSpec {
        headers: &["linux/time.h"],
        allow_vars: &["CLOCK.*"],
        allow_types: &[],
        allow_functions: &[],
    },
    GenSpec {
        headers: &["linux/wait.h"],
        allow_vars: &["W.*"],
        allow_types: &[],
        allow_functions: &[],
    },
    GenSpec {
        headers: &["linux/elf.h"],
        allow_vars: &[],
        allow_types: &["Elf.*_Ehdr", "Elf.*_Shdr", "Elf.*_Dyn", "Elf.*_Sym"],
        allow_functions: &[],
    },
    GenSpec {
        headers: &["types.h"],
        allow_vars: &[],
        allow_types: &["DT.*"],
        allow_functions: &[],
    },
];

#[derive(Debug, Copy, Clone)]
struct SupportedArch {
    linux_name: &'static str,
    rust_name: &'static str,
    clang_target: &'static str,
}

#[derive(Default)]
struct GeneratedBinds {
    bind_name: String,
    arch: Vec<ArchGenerated>,
}

struct GenSpec {
    headers: &'static [&'static str],
    allow_vars: &'static [&'static str],
    allow_types: &'static [&'static str],
    allow_functions: &'static [&'static str],
}

#[derive(Debug)]
struct ArchGenerated {
    arch: SupportedArch,
    bindings: Bindings,
}

#[tokio::main]
async fn main() -> Result<()> {
    let out_path = PathBuf::from("linux-rust-bindings/src");
    let mut arch_specific = vec![];
    let headers = headers();
    for arch in SUPPORTED_ARCHES {
        let incl = PathBuf::from("include-kernel-headers")
            .join(arch.rust_name)
            .join("sysroot")
            .join("include");
        let mut builder = bindgen::builder()
            .clang_arg("-std=gnu11")
            .clang_arg(format!("--target={}", arch.clang_target))
            .detect_include_paths(false)
            .clang_arg(format!("-I{}", path_like_to_str(&incl)?))
            .layout_tests(false)
            .default_macro_constant_type(MacroTypeVariation::Signed)
            .use_core()
            // Address family
            .allowlist_var("AF_.*")
            // Clone constants
            .allowlist_var("CLONE_.*")
            // mmap
            .allowlist_var("MAP.*")
            // Signals
            .allowlist_var("_N.*")
            .allowlist_var("SA_.*")
            // User `mode` constants, file permissions etc
            .allowlist_var("S_.*")
            // Memory protection
            .allowlist_var("PROT.*")
            // Clone args
            .allowlist_type("clone_.*")
            // sigset
            .allowlist_type("sigact.*")
            // socketaddr
            .allowlist_type("sock.*")
            .allowlist_type("win.*")
            // timespec
            .allowlist_type("__kernel_time.*")
            // signal functions
            .allowlist_type("__sig.*")
            // signal functions
            .allowlist_function("__sig.*");
        for header in &headers {
            builder = builder.header(enrich_header(header, arch.rust_name)?);
        }
        let out = builder.generate()?;
        arch_specific.push(ArchGenerated {
            arch: *arch,
            bindings: out,
        });
    }
    write_bindings(
        &out_path,
        GeneratedBinds {
            bind_name: "bindings".to_string(),
            arch: arch_specific,
        },
    )
    .await?;
    Ok(())
}

fn incl_dir(arch_rust_name: &str) -> PathBuf {
    PathBuf::from("include-kernel-headers")
        .join(arch_rust_name)
        .join("sysroot")
        .join("include")
}

fn base_builder(arch: &SupportedArch) -> Result<Builder> {
    let incl = incl_dir(arch.rust_name);
    Ok(bindgen::builder()
        .clang_arg("-std=gnu11")
        .clang_arg(format!("--target={}", arch.clang_target))
        .detect_include_paths(false)
        .clang_arg(format!("-I{}", path_like_to_str(&incl)?))
        .layout_tests(false)
        .default_macro_constant_type(MacroTypeVariation::Signed)
        .use_core())
}

fn headers() -> Vec<PathBuf> {
    vec![
        // Arch platform specifics
        PathBuf::from("arch.h"),
        // typedefs, uint_t etc
        PathBuf::from("asm").join("stat.h"),
        PathBuf::from("linux").join("mman.h"),
        PathBuf::from("linux").join("socket.h"),
        PathBuf::from("asm").join("socket.h"),
        PathBuf::from("linux").join("un.h"),
        // Wnohang etc
        PathBuf::from("linux").join("wait.h"),
        // Clockids
        PathBuf::from("linux").join("time.h"),
        PathBuf::from("linux").join("time_types.h"),
        // Clone stuff
        PathBuf::from("linux").join("sched.h"),
    ]
}

fn enrich_header(input: &Path, arch: &str) -> Result<String> {
    let enriched = PathBuf::from("include-kernel-headers")
        .join(arch)
        .join("sysroot")
        .join("include")
        .join(input);
    path_like_to_str(enriched)
}

async fn write_bindings(out_dir: &Path, generated: GeneratedBinds) -> Result<()> {
    let mod_dir = out_dir.join(&generated.bind_name);
    let mod_dir_md = tokio::fs::metadata(&mod_dir).await;
    if mod_dir_md.is_ok() {
        tokio::fs::remove_dir_all(&mod_dir).await?;
    }
    let mod_file = out_dir.join(format!("{}.rs", generated.bind_name));
    let mut mods = String::new();
    tokio::fs::create_dir_all(&mod_dir).await?;
    for gen in &generated.arch {
        let name = format!("{}_{}", generated.bind_name, gen.arch.linux_name);
        let target = mod_dir.join(format!("{name}.rs"));
        tokio::fs::write(&target, gen.bindings.to_string()).await?;
        let _ = mods.write_fmt(format_args!(
            "#[cfg(target_arch = \"{}\")]\npub mod {name};\n",
            gen.arch.rust_name
        ));
        let _ = mods.write_fmt(format_args!(
            "#[cfg(target_arch = \"{}\")]\npub use {name}::*;\n",
            gen.arch.rust_name
        ));
    }
    tokio::fs::write(&mod_file, mods).await?;

    Ok(())
}

#[inline]
fn path_like_to_str<T: AsRef<Path>>(path: T) -> Result<String> {
    let rf = path.as_ref();
    let rf_os = rf.as_os_str();
    let rf_utf8 = rf_os
        .to_str()
        .context(format!("Could not convert path like {rf:?} to utf8"))?;
    Ok(rf_utf8.to_string())
}
