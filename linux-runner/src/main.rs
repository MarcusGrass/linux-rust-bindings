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

fn generate_files() -> [GenSpec; 24] {
    [
        GenSpec::new("auxvec", &["linux/auxvec.h"], |bldr: Builder| {
            bldr.allowlist_var("AT.*")
        }),
        GenSpec::new("elf", &["linux/elf.h"], |bldr: Builder| {
            bldr.allowlist_type("Elf.*_Ehdr")
                .allowlist_type("Elf.*_Shdr")
                .allowlist_type("Elf.*_Dyn")
                .allowlist_type("Elf.*_Sym")
                .allowlist_type("Elf.*_Phdr")
                .allowlist_type("Elf.*_Rel.*")
                .allowlist_var("PT_.*")
                .allowlist_var("DT_.*")
        }),
        GenSpec::new("errno", &["linux/errno.h"], |bldr: Builder| {
            bldr.allowlist_var("E.*")
        }),
        GenSpec::new("epoll", &["linux/eventpoll.h"], |bldr: Builder| {
            bldr.allowlist_type("epoll.*").allowlist_var("E.*")
        }),
        GenSpec::new("fcntl", &["linux/fcntl.h"], |bldr: Builder| {
            bldr.allowlist_var("O_.*")
                .allowlist_var("AT_.*")
                .allowlist_var("F_.*")
        }),
        GenSpec::new("fs", &["linux/fs.h"], |bldr: Builder| {
            bldr.allowlist_var("RENAME.*").allowlist_var("SEEK.*")
        }),
        GenSpec::new("futex", &["linux/futex.h"], |bldr: Builder| {
            bldr.allowlist_var("FUTEX_.*")
        }),
        GenSpec::new(
            "hidio",
            &["linux/hid.h", "linux/hidraw.h", "linux/hiddev.h"],
            |bldr: Builder| bldr.allowlist_var("HID.*").allowlist_type("hid.*"),
        ),
        GenSpec::new("ioctl", &["linux/ioctl.h"], |bldr: Builder| {
            bldr.allowlist_var("_IO.*")
        }),
        GenSpec::new("io_uring", &["linux/io_uring.h"], |bldr: Builder| {
            bldr.allowlist_var("IO.*")
                .allowlist_type("io_uring_params")
                .allowlist_type("io_uring_sq.*")
                .allowlist_type("io_uring_cq.*")
                .allowlist_type("io_uring_op.*")
        }),
        GenSpec::new("mman", &["linux/mman.h"], |bldr: Builder| {
            bldr.allowlist_var("MAP.*").allowlist_var("PROT.*")
        }),
        GenSpec::new("mount", &["linux/mount.h"], |bldr: Builder| {
            bldr.allowlist_var("MS.*")
        }),
        GenSpec::new("poll", &["linux/poll.h"], |bldr: Builder| {
            bldr.allowlist_var("POLL.*").allowlist_type("poll.*")
        }),
        GenSpec::new("sched", &["linux/sched.h"], |bldr: Builder| {
            bldr.allowlist_var("CLONE.*").allowlist_type("clone.*")
        }),
        GenSpec::new("signal", &["linux/signal.h"], |bldr: Builder| {
            bldr.allowlist_var("SIG.*")
                .allowlist_var("SA.*")
                .allowlist_type("__sig.*")
                .allowlist_type("sigact.h")
        }),
        GenSpec::new(
            "socket",
            &["linux/net.h", "linux/un.h", "asm/socket.h"],
            |bldr: Builder| {
                bldr.allowlist_var("AF.*")
                    .allowlist_type("sock.*")
                    .allowlist_type("msg.*")
            },
        ),
        GenSpec::new("stat", &["asm/stat.h", "linux/stat.h"], |bldr: Builder| {
            bldr.allowlist_var("S_.*")
                .allowlist_var("STATX.*")
                .allowlist_type("stat.*")
        }),
        GenSpec::new("termios", &["linux/termios.h"], |bldr: Builder| {
            bldr.allowlist_var("TC.*")
                .allowlist_var("TIO.*")
                .allowlist_var("ECHO.*")
                //Input flags
                .allowlist_var("I.*")
                .allowlist_var("BRK.*")
                .allowlist_var("PAR.*")
                .allowlist_type("term.*")
                .allowlist_type("winsize")
        }),
        GenSpec::new(
            "time",
            &["linux/time.h", "linux/time_types.h"],
            |bldr: Builder| {
                bldr.allowlist_var("CLOCK.*")
                    .allowlist_type("__kernel_timespec")
                    // Really convenient to derive this here to make time types
                    .derive_eq(true)
                    .derive_partialeq(true)
                    .derive_ord(true)
                    .derive_partialord(true)
                    .derive_hash(true)
            },
        ),
        GenSpec::new("types", &["types.h"], |bldr: Builder| {
            bldr.allowlist_var("DT.*")
        }),
        GenSpec::new("uio", &["linux/uio.h"], |bldr: Builder| {
            bldr.allowlist_type("iovec.*")
        }),
        GenSpec::new("utsname", &["linux/utsname.h"], |bldr: Builder| {
            bldr.allowlist_type("new_uts.*")
        }),
        GenSpec::new(
            "usb",
            &["linux/usbdevice_fs.h", "linux/usbip.h", "linux/usb/ch9.h"],
            |bldr: Builder| bldr.allowlist_type("usb.*").allowlist_var("USB.*"),
        ),
        GenSpec::new("wait", &["linux/wait.h"], |bldr: Builder| {
            bldr.allowlist_var("W.*")
        }),
    ]
}

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
    modifier: Box<dyn BuilderModifier>,
}

trait BuilderModifier {
    fn modify(&self, builder: Builder) -> Builder;
}

impl<F> BuilderModifier for F
where
    F: Fn(Builder) -> Builder,
{
    fn modify(&self, builder: Builder) -> Builder {
        (self)(builder)
    }
}

impl GenSpec {
    fn new<B>(mod_name: &'static str, headers: &'static [&'static str], modifier: B) -> Self
    where
        B: BuilderModifier + 'static,
    {
        Self {
            mod_name,
            headers,
            modifier: Box::new(modifier),
        }
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
    for gen in generate_files() {
        generate(gen, &out_path).await?;
    }
    Ok(())
}

async fn generate(gen: GenSpec, out_path: &Path) -> Result<()> {
    let mut arch_specific = vec![];
    for arch in SUPPORTED_ARCHES {
        let mut base = base_builder(arch)?;
        base = complete_builder(arch, &gen, base)?;
        arch_specific.push(ArchGenerated {
            arch: *arch,
            bindings: base.generate()?,
        })
    }
    write_bindings(
        out_path,
        GeneratedBinds {
            bind_name: gen.mod_name.to_string(),
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
        .detect_include_paths(true)
        .clang_arg(format!("-I{}", path_like_to_str(incl)?))
        .layout_tests(false)
        .default_macro_constant_type(MacroTypeVariation::Signed)
        .use_core())
}

fn complete_builder(arch: &SupportedArch, gen: &GenSpec, mut builder: Builder) -> Result<Builder> {
    for header in gen.headers {
        builder = builder.header(enrich_header(header, arch.rust_name)?);
    }
    builder = gen.modifier.modify(builder);
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
    for gen in generated.arch {
        let name = format!("{}_{}", generated.bind_name, gen.arch.linux_name);
        let target = mod_dir.join(format!("{name}.rs"));
        let content = gen.bindings.to_string();
        tokio::fs::write(&target, content).await?;
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
