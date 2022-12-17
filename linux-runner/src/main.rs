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
    GenSpec::new_vars("aux", &["linux/auxvec.h"], &["AT.*"]),
    GenSpec::new("elf", &["linux/elf.h"], &[], &["Elf.*_Ehdr", "Elf.*_Shdr", "Elf.*_Dyn", "Elf.*_Sym"], &[]),
    GenSpec::new_vars("errno", &["linux/errno.h"], &["E.*"]),
    GenSpec::new_vars("fcntl", &["linux/fcntl.h"], &["O_.*", "AT_.*"]),
    GenSpec::new_vars("ioctl", &["linux/ioctl.h"], &["_IO.*"]),
    GenSpec::new_vars("mman", &["linux/mman.h"], &["MAP.*", "PROT.*"]),
    GenSpec::new("poll", &["linux/poll.h"], &["POLL.*"], &["poll.*"], &[]),
    GenSpec::new("sched", &["linux/sched.h"], &["CLONE.*"], &["clone.*"], &[]),
    GenSpec::new("signal", &["linux/signal.h"], &["SIG.*", "SA.*"], &["__sig.*", "sigact.h"], &[]),
    GenSpec::new("socket", &["linux/un.h", "asm/socket.h"], &["AF.*"], &["sock.*"], &[]),
    GenSpec::new("stat", &["asm/stat.h", "linux/stat.h"], &["S_.*"], &["stat.*"], &[]),
    GenSpec::new("termios", &["linux/termios.h"], &["TC.*", "TIO.*"], &["term.*", "winsize"], &[]),
    GenSpec::new("time", &["linux/time.h", "linux/time_types.h"], &["CLOCK.*"], &["__kernel_timespec"], &[]),
    GenSpec::new_vars("types", &["types.h"], &["DT.*"]),
    GenSpec::new("utsname", &["linux/utsname.h"], &[], &["new_uts.*"], &[]),
    GenSpec::new_vars("wait", &["linux/wait.h"], &["W.*"]),
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
    mod_name: &'static str,
    headers: &'static [&'static str],
    allow_vars: &'static [&'static str],
    allow_types: &'static [&'static str],
    allow_functions: &'static [&'static str],
}

impl GenSpec {
    const fn new_vars(mod_name: &'static str, headers: &'static [&'static str], allow_vars: &'static [&'static str]) -> Self {
        Self { mod_name, headers, allow_vars, allow_types: &[], allow_functions: &[] }
    }

    const fn new(mod_name: &'static str, headers: &'static [&'static str], allow_vars: &'static [&'static str], allow_types: &'static [&'static str], allow_functions: &'static [&'static str]) -> Self {
        Self { mod_name, headers, allow_vars, allow_types, allow_functions }
    }
}

#[derive(Debug)]
struct ArchGenerated {
    arch: SupportedArch,
    bindings: Bindings,
}

#[tokio::main]
async fn main() -> Result<()> {
    let out_path = PathBuf::from("linux-rust-bindings/src");
    for gen in GENERATE {
        let mut arch_specific = vec![];
        for arch in SUPPORTED_ARCHES {
            let mut base = base_builder(arch)?;
            base = complete_builder(arch, gen, base)?;
            arch_specific.push(ArchGenerated {
                arch: *arch,
                bindings: base.generate()?,
            })
        }
        write_bindings(&out_path, GeneratedBinds {
            bind_name: gen.mod_name.to_string(),
            arch: arch_specific,
        }).await?;
    }
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

fn complete_builder(arch: &SupportedArch, gen: &GenSpec, mut builder: Builder) -> Result<Builder> {
    for header in gen.headers {
        builder = builder.header(enrich_header(header, arch.rust_name)?);
    }
    for var in gen.allow_vars {
        builder = builder.allowlist_var(var);
    }
    for t in gen.allow_types {
        builder = builder.allowlist_type(t);
    }
    for func in gen.allow_functions {
        builder = builder.allowlist_function(func);
    }
    Ok(builder)
}

fn enrich_header(input: &str, arch: &str) -> Result<String> {
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
