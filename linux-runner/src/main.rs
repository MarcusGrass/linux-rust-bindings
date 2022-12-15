use std::fmt::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use bindgen::{Bindings, MacroTypeVariation};

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
            // We don't want everything, just what we're using, otherwise this repo will be huge
            // AT_ constants
            .allowlist_var("AT_.*")
            // Clock constants
            .allowlist_var("CLOCK_.*")
            // Clone constants
            .allowlist_var("CLONE_.*")
            // Errors
            .allowlist_var("E.*")
            .allowlist_var("_IO.*")
            // mmap
            .allowlist_var("MAP.*")
            // Signals
            .allowlist_var("_N.*")
            .allowlist_var("SIG.*")
            .allowlist_var("SA_.*")
            // User `mode` constants, file permissions etc
            .allowlist_var("S_.*")
            // Poll constants
            .allowlist_var("POLL.*")
            // Memory protection
            .allowlist_var("PROT.*")
            .allowlist_var("TC.*")
            .allowlist_var("TIO.*")
            // Open constants from fcntl
            .allowlist_var("O_.*")
            // Wait constants
            .allowlist_var("W.*")
            // Clone args
            .allowlist_type("clone_.*")
            // pollfd
            .allowlist_type("poll.*")
            .allowlist_type("term.*")
            // stat structs
            .allowlist_type("stat.*")
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
    let mut libc_compat_types = vec![];
    for arch in SUPPORTED_ARCHES {
        let incl = PathBuf::from("include-kernel-headers")
            .join(arch.rust_name)
            .join("sysroot")
            .join("include");
        let bindings = bindgen::builder()
            .clang_arg("-std=gnu11")
            .clang_arg(format!("--target={}", arch.clang_target))
            .detect_include_paths(false)
            .clang_arg(format!("-I{}", path_like_to_str(&incl)?))
            .layout_tests(false)
            .default_macro_constant_type(MacroTypeVariation::Signed)
            .use_core()
            // Need to put types separately, since it does some wild re-defs.
            .header(enrich_header(&PathBuf::from("types.h"), arch.rust_name)?)
            .allowlist_var("DT_.*")
            .generate()?;
        libc_compat_types.push(ArchGenerated {
            arch: *arch,
            bindings,
        })
    }
    write_bindings(
        &out_path,
        GeneratedBinds {
            bind_name: "nolibc".to_string(),
            arch: libc_compat_types,
        },
    )
    .await?;
    let mut elf_types = vec![];
    for arch in SUPPORTED_ARCHES {
        let incl = PathBuf::from("include-kernel-headers")
            .join(arch.rust_name)
            .join("sysroot")
            .join("include");
        let bindings = bindgen::builder()
            .clang_arg("-std=gnu11")
            .clang_arg(format!("--target={}", arch.clang_target))
            .detect_include_paths(false)
            .clang_arg(format!("-I{}", path_like_to_str(&incl)?))
            .layout_tests(false)
            .default_macro_constant_type(MacroTypeVariation::Signed)
            .use_core()
            // Need to put types separately, since it does some wild re-defs.
            .header(enrich_header(
                &PathBuf::from("linux").join("elf.h"),
                arch.rust_name,
            )?)
            // Elf header 64/32-bit
            .allowlist_type("Elf.*_Ehdr")
            // Elf section header 64/32-bit
            .allowlist_type("Elf.*_Shdr")
            // Elf dyn section 64/32-bit
            .allowlist_type("Elf.*_Dyn")
            // Elf symbol 64/32-bit
            .allowlist_type("Elf.*_Sym")
            .generate()?;
        elf_types.push(ArchGenerated {
            arch: *arch,
            bindings,
        })
    }
    write_bindings(
        &out_path,
        GeneratedBinds {
            bind_name: "elf".to_string(),
            arch: elf_types,
        },
    )
    .await?;
    Ok(())
}

fn headers() -> Vec<PathBuf> {
    vec![
        // Arch platform specifics
        PathBuf::from("arch.h"),
        // typedefs, uint_t etc
        PathBuf::from("std.h"),
        PathBuf::from("linux").join("auxvec.h"),
        PathBuf::from("linux").join("fcntl.h"),
        PathBuf::from("linux").join("errno.h"),
        PathBuf::from("linux").join("signal.h"),
        PathBuf::from("linux").join("stat.h"),
        PathBuf::from("asm").join("stat.h"),
        PathBuf::from("linux").join("mman.h"),
        PathBuf::from("linux").join("poll.h"),
        PathBuf::from("linux").join("socket.h"),
        PathBuf::from("asm").join("socket.h"),
        PathBuf::from("linux").join("un.h"),
        PathBuf::from("linux").join("signal.h"),
        // ioctl
        PathBuf::from("linux").join("ioctl.h"),
        // Termios
        PathBuf::from("linux").join("termios.h"),
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
