// SPDX-License-Identifier: GPL-2.0

//! The custom target specification file generator for `rustc`.
//!
//! To configure a target from scratch, a JSON-encoded file has to be passed
//! to `rustc` (introduced in [RFC 131]). These options and the file itself are
//! unstable. Eventually, `rustc` should provide a way to do this in a stable
//! manner. For instance, via command-line arguments.
//!
//! [RFC 131]: https://rust-lang.github.io/rfcs/0131-target-specification.html

use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    io::BufRead,
};

enum Value {
    Boolean(bool),
    Number(i32),
    String(String),
    Array(Array),
    Object(Object),
}

type Array = Vec<Value>;
type Object = Vec<(String, Value)>;

/// Minimal "almost JSON" generator (e.g. no `null`s, no escaping), enough
/// for this purpose.
impl Display for Value {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Value::Boolean(boolean) => write!(formatter, "{}", boolean),
            Value::Number(number) => write!(formatter, "{}", number),
            Value::String(string) => write!(formatter, "\"{}\"", string),
            Value::Array(array) => {
                formatter.write_str("[")?;
                if let [ref rest @ .., ref last] = array[..] {
                    for value in rest {
                        write!(formatter, "{},", value)?;
                    }
                    write!(formatter, "{}", last)?;
                }
                formatter.write_str("]")
            }
            Value::Object(object) => {
                formatter.write_str("{")?;
                if let [ref rest @ .., ref last] = object[..] {
                    for (key, value) in rest {
                        write!(formatter, "\"{}\":{},", key, value)?;
                    }
                    write!(formatter, "\"{}\":{}", last.0, last.1)?;
                }
                formatter.write_str("}")
            }
        }
    }
}

struct TargetSpec(Object);

impl TargetSpec {
    fn new() -> TargetSpec {
        TargetSpec(Vec::new())
    }
}

trait Push<T> {
    fn push(&mut self, key: &str, value: T);
}

impl Push<bool> for TargetSpec {
    fn push(&mut self, key: &str, value: bool) {
        self.0.push((key.to_string(), Value::Boolean(value)));
    }
}

impl Push<i32> for TargetSpec {
    fn push(&mut self, key: &str, value: i32) {
        self.0.push((key.to_string(), Value::Number(value)));
    }
}

impl Push<String> for TargetSpec {
    fn push(&mut self, key: &str, value: String) {
        self.0.push((key.to_string(), Value::String(value)));
    }
}

impl Push<&str> for TargetSpec {
    fn push(&mut self, key: &str, value: &str) {
        self.push(key, value.to_string());
    }
}

impl Push<Object> for TargetSpec {
    fn push(&mut self, key: &str, value: Object) {
        self.0.push((key.to_string(), Value::Object(value)));
    }
}

impl Display for TargetSpec {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        // We add some newlines for clarity.
        formatter.write_str("{\n")?;
        if let [ref rest @ .., ref last] = self.0[..] {
            for (key, value) in rest {
                write!(formatter, "\"{}\":{},\n", key, value)?;
            }
            write!(formatter, "\"{}\":{}\n", last.0, last.1)?;
        }
        formatter.write_str("}")
    }
}

struct KernelConfig(HashMap<String, String>);

impl KernelConfig {
    /// Parses `include/config/auto.conf` from `stdin`.
    fn from_stdin() -> KernelConfig {
        let mut result = HashMap::new();

        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let mut line = String::new();

        loop {
            line.clear();

            if handle.read_line(&mut line).unwrap() == 0 {
                break;
            }

            if line.starts_with('#') {
                continue;
            }

            let (key, value) = line.split_once('=').expect("Missing `=` in line.");
            result.insert(key.to_string(), value.trim_end_matches('\n').to_string());
        }

        KernelConfig(result)
    }

    /// Does the option exist in the configuration (any value)?
    ///
    /// The argument must be passed without the `CONFIG_` prefix.
    /// This avoids repetition and it also avoids `fixdep` making us
    /// depending on it.
    fn has(&self, option: &str) -> bool {
        let option = "CONFIG_".to_owned() + option;
        self.0.contains_key(&option)
    }
}

fn main() {
    let cfg = KernelConfig::from_stdin();
    let mut ts = TargetSpec::new();

    let pre_link_args = vec![(
        "gcc".to_string(),
        Value::Array(vec![
            Value::String("-Wl,--as-needed".to_string()),
            Value::String("-Wl,-z,noexecstack".to_string()),
        ]),
    )];

    let pre_link_args_32 = vec![(
        "gcc".to_string(),
        Value::Array(vec![
            Value::String("-Wl,--as-needed".to_string()),
            Value::String("-Wl,-z,noexecstack".to_string()),
            Value::String("-m32".to_string()),
        ]),
    )];

    let pre_link_args_64 = vec![(
        "gcc".to_string(),
        Value::Array(vec![
            Value::String("-Wl,--as-needed".to_string()),
            Value::String("-Wl,-z,noexecstack".to_string()),
            Value::String("-m64".to_string()),
        ]),
    )];

    let stack_probes = vec![("kind".to_string(), Value::String("none".to_string()))];

    if cfg.has("ARM") {
        ts.push("arch", "arm");
        ts.push(
            "data-layout",
            "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64",
        );
        ts.push("dynamic-linking", true);
        ts.push("crt-static-respected", true);
        ts.push("executables", true);
        ts.push("features", "+strict-align,+v6");
        ts.push("has-elf-tls", true);
        ts.push("has-rpath", true);
        ts.push("llvm-target", "arm-unknown-linux-gnueabi");
        ts.push("max-atomic-width", 64);
        ts.push("pre-link-args", pre_link_args);
        ts.push("target-family", "unix");
        ts.push("target-mcount", "\\u0001__gnu_mcount_nc");
        ts.push("target-pointer-width", "32");
    } else if cfg.has("ARM64") {
        ts.push("arch", "aarch64");
        ts.push(
            "data-layout",
            "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128",
        );
        ts.push("disable-redzone", true);
        ts.push("emit-debug-gdb-scripts", false);
        ts.push("features", "+strict-align,+neon,+fp-armv8");
        ts.push("frame-pointer", "always");
        ts.push("llvm-target", "aarch64-unknown-none");
        ts.push("max-atomic-width", 128);
        ts.push("needs-plt", true);
        ts.push("pre-link-args", pre_link_args_64);
        ts.push("stack-probes", stack_probes);
        ts.push("target-c-int-width", "32");
        ts.push("target-pointer-width", "64");
        ts.push("vendor", "");
    } else if cfg.has("PPC") {
        ts.push("arch", "powerpc64");
        ts.push("code-model", "large");
        ts.push("cpu", "ppc64le");
        ts.push("data-layout", "e-m:e-i64:64-n32:64");
        ts.push("features", "-altivec,-vsx,-hard-float");
        ts.push("llvm-target", "powerpc64le-elf");
        ts.push("max-atomic-width", 64);
        ts.push("pre-link-args", pre_link_args_64);
        ts.push("target-family", "unix");
        ts.push("target-mcount", "_mcount");
        ts.push("target-pointer-width", "64");
    } else if cfg.has("RISCV") {
        if cfg.has("64BIT") {
            ts.push("arch", "riscv64");
            ts.push("cpu", "generic-rv64");
            ts.push("data-layout", "e-m:e-p:64:64-i64:64-i128:128-n64-S128");
            ts.push("llvm-target", "riscv64");
            ts.push("max-atomic-width", 64);
            ts.push("pre-link-args", pre_link_args_64);
            ts.push("target-pointer-width", "64");
        } else {
            ts.push("arch", "riscv32");
            ts.push("cpu", "generic-rv32");
            ts.push("data-layout", "e-m:e-p:32:32-i64:64-n32-S128");
            ts.push("llvm-target", "riscv32");
            ts.push("max-atomic-width", 32);
            ts.push("pre-link-args", pre_link_args_32);
            ts.push("target-pointer-width", "32");
        }
        ts.push("code-model", "medium");
        ts.push("disable-redzone", true);
        ts.push("emit-debug-gdb-scripts", false);
        let mut features = "+m,+a".to_string();
        if cfg.has("RISCV_ISA_C") {
            features += ",+c";
        }
        ts.push("features", features);
        ts.push("frame-pointer", "always");
        ts.push("needs-plt", true);
        ts.push("target-c-int-width", "32");
        ts.push("stack-probes", stack_probes);
        ts.push("vendor", "");
    } else if cfg.has("X86") {
        ts.push("arch", "x86_64");
        ts.push("code-model", "kernel");
        ts.push("cpu", "x86-64");
        ts.push(
            "data-layout",
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
        );
        ts.push("disable-redzone", true);
        ts.push("emit-debug-gdb-scripts", false);
        ts.push(
            "features",
            "-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2,+soft-float",
        );
        ts.push("frame-pointer", "always");
        ts.push("llvm-target", "x86_64-elf");
        ts.push("max-atomic-width", 64);
        ts.push("needs-plt", true);
        ts.push("pre-link-args", pre_link_args_64);
        ts.push("stack-probes", stack_probes);
        ts.push("target-c-int-width", "32");
        ts.push("target-pointer-width", "64");
        ts.push("vendor", "unknown");
    } else {
        panic!("Unsupported architecture");
    }

    ts.push("env", "gnu");
    ts.push("function-sections", false);
    ts.push("linker-is-gnu", true);
    ts.push("os", if cfg.has("ARM") { "linux" } else { "none" });
    ts.push("position-independent-executables", true);
    ts.push("relocation-model", "static");

    if !cfg.has("ARM") {
        ts.push("linker-flavor", "gcc");
        ts.push("panic-strategy", "abort");
        ts.push("relro-level", "full");

        if cfg.has("CPU_BIG_ENDIAN") {
            ts.push("target-endian", "big");
        } else {
            // Everything else is LE, whether `CPU_LITTLE_ENDIAN`
            // is declared or not (e.g. x86).
            ts.push("target-endian", "little");
        }
    }

    println!("{}", ts);
}
