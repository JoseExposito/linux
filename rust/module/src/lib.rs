// SPDX-License-Identifier: GPL-2.0

//! Implements the `module!` macro magic

extern crate proc_macro;

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
    if let None = it.next() {
    } else {
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
    assert!(byte_string.ends_with("\""));

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
        // Built-in modules prefix their modinfo strings by `module.`
        format!(
            "{module}.{field}={content}",
            module = module,
            field = field,
            content = content
        )
    } else {
        // Loadable modules' modinfo strings go as-is
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

/// Declares a kernel module.
///
/// The `type` argument should be a type which implements the [`KernelModule`] trait.
/// Also accepts various forms of kernel metadata.
///
/// Example:
/// ```rust,no_run
/// use kernel::prelude::*;
///
/// module!{
///     type: MyKernelModule,
///     name: b"my_kernel_module",
///     author: b"Rust for Linux Contributors",
///     description: b"My very own kernel module!",
///     license: b"GPL v2",
///     params: {},
/// }
///
/// struct MyKernelModule;
///
/// impl KernelModule for MyKernelModule {
///     fn init() -> KernelResult<Self> {
///         Ok(MyKernelModule)
///     }
/// }
/// ```
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
        let param_default = if param_type == "bool" {
            get_ident(&mut param_it, "default")
        } else {
            get_literal(&mut param_it, "default")
        };
        let param_permissions = get_literal(&mut param_it, "permissions");
        let param_description = get_byte_string(&mut param_it, "description");
        expect_end(&mut param_it);

        // TODO: more primitive types
        // TODO: other kinds: arrays, unsafes, etc.
        let param_kernel_type = match param_type.as_ref() {
            "bool" => "bool",
            "i32" => "int",
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
        params_modinfo.push_str(
            &format!(
                "
                static mut __{name}_{param_name}_value: {param_type} = {param_default};

                struct __{name}_{param_name};

                impl __{name}_{param_name} {{
                    fn read(&self) -> {param_type} {{
                        unsafe {{ __{name}_{param_name}_value }}
                    }}
                }}

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
                    // TODO: `THIS_MODULE`
                    mod_: core::ptr::null_mut(),
                    ops: unsafe {{ &kernel::bindings::param_ops_{param_kernel_type} }} as *const kernel::bindings::kernel_param_ops,
                    perm: {permissions},
                    level: -1,
                    flags: 0,
                    __bindgen_anon_1: kernel::bindings::kernel_param__bindgen_ty_1 {{
                        arg: unsafe {{ &__{name}_{param_name}_value }} as *const _ as *mut kernel::c_types::c_void,
                    }},
                }});
                ",
                name = name,
                param_type = param_type,
                param_kernel_type = param_kernel_type,
                param_default = param_default,
                param_name = param_name,
                permissions = param_permissions,
            )
        );
    }

    let file = std::env::var("RUST_MODFILE").unwrap();

    format!(
        "
            static mut __MOD: Option<{type_}> = None;

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
