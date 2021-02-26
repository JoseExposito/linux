// SPDX-License-Identifier: GPL-2.0

//! Proc macro crate implementing the [`module!`] magic.
//!
//! C header: [`include/linux/moduleparam.h`](../../../include/linux/moduleparam.h)

#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::perf)]
#![deny(clippy::style)]

use proc_macro::{token_stream, Delimiter, Group, TokenStream, TokenTree};

fn expect_ident(it: &mut token_stream::IntoIter) -> String {
    if let TokenTree::Ident(ident) = it.next().unwrap() {
        ident.to_string()
    } else {
        panic!("Expected Ident");
    }
}

fn expect_punct(it: &mut token_stream::IntoIter) -> char {
    if let TokenTree::Punct(punct) = it.next().unwrap() {
        punct.as_char()
    } else {
        panic!("Expected Punct");
    }
}

fn expect_literal(it: &mut token_stream::IntoIter) -> String {
    if let TokenTree::Literal(literal) = it.next().unwrap() {
        literal.to_string()
    } else {
        panic!("Expected Literal");
    }
}

fn expect_group(it: &mut token_stream::IntoIter) -> Group {
    if let TokenTree::Group(group) = it.next().unwrap() {
        group
    } else {
        panic!("Expected Group");
    }
}

fn expect_end(it: &mut token_stream::IntoIter) {
    if it.next().is_some() {
        panic!("Expected end");
    }
}

fn get_ident(it: &mut token_stream::IntoIter, expected_name: &str) -> String {
    assert_eq!(expect_ident(it), expected_name);
    assert_eq!(expect_punct(it), ':');
    let ident = expect_ident(it);
    assert_eq!(expect_punct(it), ',');
    ident
}

fn get_literal(it: &mut token_stream::IntoIter, expected_name: &str) -> String {
    assert_eq!(expect_ident(it), expected_name);
    assert_eq!(expect_punct(it), ':');
    let literal = expect_literal(it);
    assert_eq!(expect_punct(it), ',');
    literal
}

fn get_group(it: &mut token_stream::IntoIter, expected_name: &str) -> Group {
    assert_eq!(expect_ident(it), expected_name);
    assert_eq!(expect_punct(it), ':');
    let group = expect_group(it);
    assert_eq!(expect_punct(it), ',');
    group
}

fn get_byte_string(it: &mut token_stream::IntoIter, expected_name: &str) -> String {
    let byte_string = get_literal(it, expected_name);

    assert!(byte_string.starts_with("b\""));
    assert!(byte_string.ends_with('\"'));

    byte_string[2..byte_string.len() - 1].to_string()
}

fn __build_modinfo_string_base(
    module: &str,
    field: &str,
    content: &str,
    variable: &str,
    builtin: bool,
) -> String {
    let string = if builtin {
        // Built-in modules prefix their modinfo strings by `module.`.
        format!(
            "{module}.{field}={content}",
            module = module,
            field = field,
            content = content
        )
    } else {
        // Loadable modules' modinfo strings go as-is.
        format!("{field}={content}", field = field, content = content)
    };

    format!(
        "
            {cfg}
            #[link_section = \".modinfo\"]
            #[used]
            pub static {variable}: [u8; {length}] = *b\"{string}\\0\";
        ",
        cfg = if builtin {
            "#[cfg(not(MODULE))]"
        } else {
            "#[cfg(MODULE)]"
        },
        variable = variable,
        length = string.len() + 1,
        string = string,
    )
}

fn __build_modinfo_string_variable(module: &str, field: &str) -> String {
    format!("__{module}_{field}", module = module, field = field)
}

fn build_modinfo_string_only_builtin(module: &str, field: &str, content: &str) -> String {
    __build_modinfo_string_base(
        module,
        field,
        content,
        &__build_modinfo_string_variable(module, field),
        true,
    )
}

fn build_modinfo_string_only_loadable(module: &str, field: &str, content: &str) -> String {
    __build_modinfo_string_base(
        module,
        field,
        content,
        &__build_modinfo_string_variable(module, field),
        false,
    )
}

fn build_modinfo_string(module: &str, field: &str, content: &str) -> String {
    build_modinfo_string_only_builtin(module, field, content)
        + &build_modinfo_string_only_loadable(module, field, content)
}

fn build_modinfo_string_param(module: &str, field: &str, param: &str, content: &str) -> String {
    let variable = format!(
        "__{module}_{field}_{param}",
        module = module,
        field = field,
        param = param
    );
    let content = format!("{param}:{content}", param = param, content = content);
    __build_modinfo_string_base(module, field, &content, &variable, true)
        + &__build_modinfo_string_base(module, field, &content, &variable, false)
}

fn permissions_are_readonly(perms: &str) -> bool {
    let (radix, digits) = if let Some(n) = perms.strip_prefix("0x") {
        (16, n)
    } else if let Some(n) = perms.strip_prefix("0o") {
        (8, n)
    } else if let Some(n) = perms.strip_prefix("0b") {
        (2, n)
    } else {
        (10, perms)
    };
    match u32::from_str_radix(digits, radix) {
        Ok(perms) => perms & 0o222 == 0,
        Err(_) => false,
    }
}

/// Declares a kernel module.
///
/// The `type` argument should be a type which implements the [`KernelModule`]
/// trait. Also accepts various forms of kernel metadata.
///
/// [`KernelModule`]: ../kernel/trait.KernelModule.html
///
/// # Examples
///
/// ```rust,no_run
/// use kernel::prelude::*;
///
/// module!{
///     type: MyKernelModule,
///     name: b"my_kernel_module",
///     author: b"Rust for Linux Contributors",
///     description: b"My very own kernel module!",
///     license: b"GPL v2",
///     params: {
///        my_i32: i32 {
///            default: 42,
///            permissions: 0o000,
///            description: b"Example of i32",
///        },
///        writeable_i32: i32 {
///            default: 42,
///            permissions: 0o644,
///            description: b"Example of i32",
///        },
///    },
/// }
///
/// struct MyKernelModule;
///
/// impl KernelModule for MyKernelModule {
///     fn init() -> KernelResult<Self> {
///         // If the parameter is writeable, then the kparam lock must be
///         // taken to read the parameter:
///         {
///             let lock = THIS_MODULE.kernel_param_lock();
///             println!("i32 param is:  {}", writeable_i32.read(&lock));
///         }
///         // If the parameter is read only, it can be read without locking
///         // the kernel parameters:
///         println!("i32 param is:  {}", my_i32.read());
///         Ok(MyKernelModule)
///     }
/// }
/// ```
///
/// # Supported parameter types
///
///   - `bool`: Corresponds to C `bool` param type.
///   - `u8`: Corresponds to C `char` param type.
///   - `i16`: Corresponds to C `short` param type.
///   - `u16`: Corresponds to C `ushort` param type.
///   - `i32`: Corresponds to C `int` param type.
///   - `u32`: Corresponds to C `uint` param type.
///   - `u64`: Corresponds to C `ullong` param type.
///   - `str`: Corresponds to C `charp` param type. Reading returns a byte
///     slice.
///
/// `invbool` is unsupported: it was only ever used in a few modules.
/// Consider using a `bool` and inverting the logic instead.
#[proc_macro]
pub fn module(ts: TokenStream) -> TokenStream {
    let mut it = ts.into_iter();

    let type_ = get_ident(&mut it, "type");
    let name = get_byte_string(&mut it, "name");
    let author = get_byte_string(&mut it, "author");
    let description = get_byte_string(&mut it, "description");
    let license = get_byte_string(&mut it, "license");
    let params = get_group(&mut it, "params");

    expect_end(&mut it);

    assert_eq!(params.delimiter(), Delimiter::Brace);

    let mut it = params.stream().into_iter();

    let mut params_modinfo = String::new();

    loop {
        let param_name = match it.next() {
            Some(TokenTree::Ident(ident)) => ident.to_string(),
            Some(_) => panic!("Expected Ident or end"),
            None => break,
        };

        assert_eq!(expect_punct(&mut it), ':');
        let param_type = expect_ident(&mut it);
        let group = expect_group(&mut it);
        assert_eq!(expect_punct(&mut it), ',');

        assert_eq!(group.delimiter(), Delimiter::Brace);

        let mut param_it = group.stream().into_iter();
        let param_default = match param_type.as_ref() {
            "bool" => get_ident(&mut param_it, "default"),
            "str" => get_byte_string(&mut param_it, "default"),
            _ => get_literal(&mut param_it, "default"),
        };
        let param_permissions = get_literal(&mut param_it, "permissions");
        let param_description = get_byte_string(&mut param_it, "description");
        expect_end(&mut param_it);

        // TODO: more primitive types
        // TODO: other kinds: arrays, unsafes, etc.
        let param_kernel_type = match param_type.as_ref() {
            "bool" => "bool",
            "u8" => "char",
            "i16" => "short",
            "u16" => "ushort",
            "i32" => "int",
            "u32" => "uint",
            "u64" => "ullong",
            "str" => "charp",
            t => panic!("Unrecognized type {}", t),
        };

        params_modinfo.push_str(&build_modinfo_string_param(
            &name,
            "parmtype",
            &param_name,
            &param_kernel_type,
        ));
        params_modinfo.push_str(&build_modinfo_string_param(
            &name,
            "parm",
            &param_name,
            &param_description,
        ));
        let param_type_internal = match param_type.as_ref() {
            "str" => "*mut kernel::c_types::c_char",
            _ => &param_type,
        };
        let param_default = match param_type.as_ref() {
            "str" => format!(
                "b\"{}\0\" as *const _ as *mut kernel::c_types::c_char",
                param_default
            ),
            _ => param_default,
        };
        let read_func = match (param_type.as_ref(), permissions_are_readonly(&param_permissions)) {
            ("str", false) => format!(
                "
                    fn read<'lck>(&self, lock: &'lck kernel::KParamGuard) -> &'lck [u8] {{
                        // SAFETY: The pointer is provided either in `param_default` when building the module,
                        // or by the kernel through `param_set_charp`. Both will be valid C strings.
                        // Parameters are locked by `KParamGuard`.
                        unsafe {{
                            kernel::c_types::c_string_bytes(__{name}_{param_name}_value)
                        }}
                    }}
                ",
                name = name,
                param_name = param_name,
            ),
            ("str", true) => format!(
                "
                    fn read(&self) -> &[u8] {{
                        // SAFETY: The pointer is provided either in `param_default` when building the module,
                        // or by the kernel through `param_set_charp`. Both will be valid C strings.
                        // Parameters do not need to be locked because they are read only or sysfs is not enabled.
                        unsafe {{
                            kernel::c_types::c_string_bytes(__{name}_{param_name}_value)
                        }}
                    }}
                ",
                name = name,
                param_name = param_name,
            ),
            (_, false) => format!(
                "
                    // SAFETY: Parameters are locked by `KParamGuard`.
                    fn read<'lck>(&self, lock: &'lck kernel::KParamGuard) -> &'lck {param_type_internal} {{
                        unsafe {{ &__{name}_{param_name}_value }}
                    }}
                ",
                name = name,
                param_name = param_name,
                param_type_internal = param_type_internal,
            ),
            (_, true) => format!(
                "
                    // SAFETY: Parameters do not need to be locked because they are read only or sysfs is not enabled.
                    fn read(&self) -> &{param_type_internal} {{
                        unsafe {{ &__{name}_{param_name}_value }}
                    }}
                ",
                name = name,
                param_name = param_name,
                param_type_internal = param_type_internal,
            ),
        };
        let kparam = format!(
            "
                kernel::bindings::kernel_param__bindgen_ty_1 {{
                    arg: unsafe {{ &__{name}_{param_name}_value }} as *const _ as *mut kernel::c_types::c_void,
                }},
            ",
            name = name,
            param_name = param_name,
        );
        params_modinfo.push_str(
            &format!(
                "
                static mut __{name}_{param_name}_value: {param_type_internal} = {param_default};

                struct __{name}_{param_name};

                impl __{name}_{param_name} {{ {read_func} }}

                const {param_name}: __{name}_{param_name} = __{name}_{param_name};

                // Note: the C macro that generates the static structs for the `__param` section
                // asks for them to be `aligned(sizeof(void *))`. However, that was put in place
                // in 2003 in commit 38d5b085d2 (\"[PATCH] Fix over-alignment problem on x86-64\")
                // to undo GCC over-alignment of static structs of >32 bytes. It seems that is
                // not the case anymore, so we simplify to a transparent representation here
                // in the expectation that it is not needed anymore.
                // TODO: revisit this to confirm the above comment and remove it if it happened
                #[repr(transparent)]
                struct __{name}_{param_name}_RacyKernelParam(kernel::bindings::kernel_param);

                unsafe impl Sync for __{name}_{param_name}_RacyKernelParam {{
                }}

                #[cfg(not(MODULE))]
                const __{name}_{param_name}_name: *const kernel::c_types::c_char = b\"{name}.{param_name}\\0\" as *const _ as *const kernel::c_types::c_char;

                #[cfg(MODULE)]
                const __{name}_{param_name}_name: *const kernel::c_types::c_char = b\"{param_name}\\0\" as *const _ as *const kernel::c_types::c_char;

                #[link_section = \"__param\"]
                #[used]
                static __{name}_{param_name}_struct: __{name}_{param_name}_RacyKernelParam = __{name}_{param_name}_RacyKernelParam(kernel::bindings::kernel_param {{
                    name: __{name}_{param_name}_name,
                    // SAFETY: `__this_module` is constructed by the kernel at load time and will not be freed until the module is unloaded.
                    #[cfg(MODULE)]
                    mod_: unsafe {{ &kernel::bindings::__this_module as *const _ as *mut _ }},
                    #[cfg(not(MODULE))]
                    mod_: core::ptr::null_mut(),
                    ops: unsafe {{ &kernel::bindings::param_ops_{param_kernel_type} }} as *const kernel::bindings::kernel_param_ops,
                    perm: {permissions},
                    level: -1,
                    flags: 0,
                    __bindgen_anon_1: {kparam}
                }});
                ",
                name = name,
                param_type_internal = param_type_internal,
                read_func = read_func,
                param_kernel_type = param_kernel_type,
                param_default = param_default,
                param_name = param_name,
                permissions = param_permissions,
                kparam = kparam,
            )
        );
    }

    let file = std::env::var("RUST_MODFILE").unwrap();

    format!(
        "
            static mut __MOD: Option<{type_}> = None;

            // SAFETY: `__this_module` is constructed by the kernel at load time and will not be freed until the module is unloaded.
            #[cfg(MODULE)]
            static THIS_MODULE: kernel::ThisModule = unsafe {{ kernel::ThisModule::from_ptr(&kernel::bindings::__this_module as *const _ as *mut _) }};
            #[cfg(not(MODULE))]
            static THIS_MODULE: kernel::ThisModule = unsafe {{ kernel::ThisModule::from_ptr(core::ptr::null_mut()) }};

            // Loadable modules need to export the `{{init,cleanup}}_module` identifiers
            #[cfg(MODULE)]
            #[no_mangle]
            pub extern \"C\" fn init_module() -> kernel::c_types::c_int {{
                __init()
            }}

            #[cfg(MODULE)]
            #[no_mangle]
            pub extern \"C\" fn cleanup_module() {{
                __exit()
            }}

            // Built-in modules are initialized through an initcall pointer
            // and the identifiers need to be unique
            #[cfg(not(MODULE))]
            #[cfg(not(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS))]
            #[link_section = \"{initcall_section}\"]
            #[used]
            pub static __{name}_initcall: extern \"C\" fn() -> kernel::c_types::c_int = __{name}_init;

            #[cfg(not(MODULE))]
            #[cfg(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS)]
            global_asm!(
                r#\".section \"{initcall_section}\", \"a\"
                __{name}_initcall:
                    .long   __{name}_init - .
                    .previous
                \"#
            );

            #[cfg(not(MODULE))]
            #[no_mangle]
            pub extern \"C\" fn __{name}_init() -> kernel::c_types::c_int {{
                __init()
            }}

            #[cfg(not(MODULE))]
            #[no_mangle]
            pub extern \"C\" fn __{name}_exit() {{
                __exit()
            }}

            fn __init() -> kernel::c_types::c_int {{
                match <{type_} as KernelModule>::init() {{
                    Ok(m) => {{
                        unsafe {{
                            __MOD = Some(m);
                        }}
                        return 0;
                    }}
                    Err(e) => {{
                        return e.to_kernel_errno();
                    }}
                }}
            }}

            fn __exit() {{
                unsafe {{
                    // Invokes `drop()` on `__MOD`, which should be used for cleanup.
                    __MOD = None;
                }}
            }}

            {author}
            {description}
            {license}

            // Built-in modules also export the `file` modinfo string
            {file}

            {params_modinfo}
        ",
        type_ = type_,
        name = name,
        author = &build_modinfo_string(&name, "author", &author),
        description = &build_modinfo_string(&name, "description", &description),
        license = &build_modinfo_string(&name, "license", &license),
        file = &build_modinfo_string_only_builtin(&name, "file", &file),
        params_modinfo = params_modinfo,
        initcall_section = ".initcall6.init"
    ).parse().unwrap()
}
