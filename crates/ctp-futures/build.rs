#![allow(dead_code)]
extern crate bindgen;

use inflector::Inflector;

use std::env::var;
use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use clang::*;
fn get_full_name_of_entity(e: &Entity) -> String {
    let mut v = vec![e.get_name().expect("")];
    let mut xe = Box::new(e.clone());
    while let Some(e) = xe.get_lexical_parent() {
        if e.get_kind() == EntityKind::TranslationUnit || e.get_kind() == EntityKind::NotImplemented
        {
            break;
        }
        v.push(e.get_name().expect(""));
        xe = Box::new(e);
    }
    v.reverse();
    v.join("_")
}

fn parse_api(tu: &TranslationUnit, api_name: &str) -> String {
    let mut v_code = vec![];
    tu.get_entity().visit_children(|e, _parent| {
        let name = e.get_display_name();
        if let Some(name) = name {
            if name == api_name {
                let full_api_name = get_full_name_of_entity(&e);
                v_code.push(format!("impl {} {{", full_api_name));
                for e in e.get_children().iter().filter(|e| e.get_name().is_some()) {
                    let snake_fn_name = Inflector::to_snake_case(&e.get_name().unwrap());
                    let full_fn_name = get_full_name_of_entity(e);
                    if e.get_kind() == EntityKind::Destructor || e.is_static_method() {
                        continue;
                    }
                    let mut rust_code = format!("");
                    let mut c_code = format!("");
                    let arguments = e.get_arguments();
                    if let Some(arguments) = arguments {
                        for p in &arguments {
                            let tp = p.get_type().unwrap();
                            let (mut rust_type, c_type) = match tp.get_kind() {
                                TypeKind::Pointer => {
                                    let tp = tp.get_pointee_type().unwrap();
                                    let d = tp.get_declaration();
                                    if let Some(d) = d {
                                        (
                                            "&mut ".to_string() + &get_full_name_of_entity(&d),
                                            " as * mut ".to_string() + &get_full_name_of_entity(&d),
                                        )
                                    } else {
                                        match tp.get_kind() {
                                            // 这个是char*, register_front() 会有这样的参数
                                            TypeKind::CharS => (
                                                "std::ffi::CString".to_string(),
                                                ".into_raw()".to_string(),
                                            ),
                                            TypeKind::Pointer => {
                                                // 这是char**
                                                // mdapi.subscribe() 有这样的参数
                                                let tp = tp.get_pointee_type().unwrap();
                                                match tp.get_kind() {
                                                    TypeKind::CharS => (
                                                        "Vec<std::ffi::CString>".to_string(),
                                                        ".to_char_pp()".to_string(),
                                                    ),
                                                    _ => panic!(""),
                                                }
                                            }
                                            _ => (tp.get_display_name(), "".to_string()),
                                        }
                                    }
                                }
                                TypeKind::Typedef => {
                                    match tp
                                        .get_declaration()
                                        .unwrap()
                                        .get_typedef_underlying_type()
                                        .unwrap()
                                        .get_kind()
                                    {
                                        TypeKind::CharS => {
                                            ("std::os::raw::c_char".to_string(), "".to_string())
                                        }
                                        _ => {
                                            // (tp.get_display_name(), "".to_string())
                                            println!("tp={:?}", tp);
                                            panic!("");
                                        }
                                    }
                                }
                                TypeKind::Int => {
                                    ("std::os::raw::c_int".to_string(), "".to_string())
                                }
                                TypeKind::Enum => {
                                    let d = tp.get_declaration().unwrap();
                                    (get_full_name_of_entity(&d), "".to_string())
                                }
                                TypeKind::IncompleteArray => {
                                    ("Vec<std::ffi::CString>".to_string(), ".iter().map(|cs| cs.as_ptr()).collect::<Vec<_>>().as_mut_ptr() as *mut *mut i8".to_string())
                                }
                                _ => {
                                    // (tp.get_display_name(), "".to_string())
                                    println!("tp={:?}", tp);
                                    panic!("");
                                }
                            };
                            if rust_type == "int" {
                                // 或者要转为 std::os::raw::c_int
                                rust_type = "std::os::raw::c_int".to_string();
                            }
                            rust_code.push_str(&format!(
                                ", {}: {}",
                                Inflector::to_snake_case(&p.get_name().unwrap()),
                                rust_type
                            ));
                            c_code.push_str(&format!(
                                r#",
                                             {}{}"#,
                                Inflector::to_snake_case(&p.get_name().unwrap()),
                                c_type
                            ))
                        }
                    }
                    let result_type = e.get_result_type().unwrap().get_display_name();
                    let result_type = match result_type.as_str() {
                        "void" => "()",
                        "int" => "std::os::raw::c_int",
                        "const char *" => "*const std::os::raw::c_char",
                        _ => panic!("没处理的result_type={}", result_type),
                    };
                    let mut code = format!(
                        r#"
                            pub fn {}(&mut self{}) -> {} {{
                                    unsafe {{
                                           ((*(*self).vtable_).{})(self as *mut {}{})
                                        }}
                            }}"#,
                        snake_fn_name, rust_code, result_type, full_fn_name, full_api_name, c_code
                    );
                    if snake_fn_name == "register_spi" {
                        let static_vtable_var_name = Inflector::to_snake_case(&full_api_name)
                            .trim_end_matches("api")
                            .to_uppercase()
                            + "SPI_VTABLE";
                        code = format!(
                            r#"
                                pub fn {}(&mut self{}) -> {} {{
                                    let p_spi = Box::into_raw(Box::new(( &{}, p_spi)));            
                                        unsafe {{
                                               ((*(*self).vtable_).{})(self as *mut {}{})
                                            }}
                                }}"#,
                            snake_fn_name,
                            rust_code.replace("&mut", "*const dyn") + "_trait",
                            result_type,
                            static_vtable_var_name,
                            full_fn_name,
                            full_api_name,
                            c_code
                        );
                    }
                    v_code.push(code);
                }
                v_code.push(format!(
                    r#"}} 
                unsafe impl Send for {full_api_name} {{}}"#
                ));
                return EntityVisitResult::Break;
            }
        }
        EntityVisitResult::Recurse
    });
    v_code.join("")
}

fn parse_spi(tu: &TranslationUnit, spi_name: &str) -> String {
    let mut trait_lines = vec![];
    let mut vtable_lines = vec![];
    let mut static_table_lines = vec![];
    let mut c_fn_lines = vec![];
    let mut spi_output_enum_lines = vec![];
    let mut spi_output_enum_struct_lines = vec![];
    let mut impl_spi_fn_line = vec![];

    let mut fat_spi_code = "".to_string();
    let mut spi_stream_code = "".to_string();
    let vf = |e: Entity, _parent: Entity| {
        let name = e.get_name();
        if name.is_none() {
            return EntityVisitResult::Recurse;
        }
        if name.unwrap() != spi_name {
            return EntityVisitResult::Recurse;
        }
        let full_spi_name = get_full_name_of_entity(&e);
        let vtable_struct_name = format!("{full_spi_name}VTable");
        let full_trait_name = format!("{full_spi_name}_trait",);
        let full_spi_output_enum_name = format!("{full_spi_name}Output");
        let full_static_vtable_var_name =
            Inflector::to_snake_case(&full_spi_name).to_uppercase() + "_VTABLE";
        trait_lines.push(format!(r#"pub trait {full_trait_name}: Send {{"#,));
        vtable_lines.push(format!(
            r#"
        #[repr(C)]
        #[derive(Debug)]
        struct {vtable_struct_name} {{
        "#
        ));
        static_table_lines.push(format!(
            r#"static {full_static_vtable_var_name}: {vtable_struct_name} = {vtable_struct_name} {{
                "#
        ));
        spi_output_enum_lines.push(format!(
            r#"
        #[derive(Clone, Debug)]
        pub enum {full_spi_output_enum_name} {{"#
        ));
        impl_spi_fn_line.push(format!(
            r#"impl {full_trait_name} for {full_spi_name}Stream {{"#,
        ));
        for e in e.get_children().iter().filter(|e| e.get_name().is_some()) {
            let snake_fn_name = Inflector::to_snake_case(&e.get_name().unwrap());
            let fn_name = e.get_name().unwrap();
            let _full_fn_name = get_full_name_of_entity(e);
            if e.get_kind() == EntityKind::Destructor
                || e.get_kind() == EntityKind::Constructor
                || e.is_static_method()
            {
                continue;
            }
            let mut arg_list = vec![];
            if let Some(arguments) = e.get_arguments() {
                arg_list = arguments
                    .iter()
                    .map(|p| {
                        let arg_name = Inflector::to_snake_case(&p.get_name().unwrap());
                        let tp = p.get_type().unwrap();
                        match tp.get_kind() {
                            TypeKind::Pointer => {
                                let tp = tp.get_pointee_type().unwrap();
                                let d = tp.get_declaration();
                                if let Some(d) = d {
                                    (
                                        format!(
                                            "{arg_name} : Option<&{}",
                                            get_full_name_of_entity(&d) + ">"
                                        ),
                                        format!(
                                            "{arg_name} : * const {}",
                                            get_full_name_of_entity(&d)
                                        ),
                                        format!("{arg_name}.as_ref()"),
                                        format!("{arg_name}:{arg_name}.cloned()"),
                                    )
                                } else {
                                    panic!("");
                                }
                            }
                            TypeKind::Typedef => {
                                match tp
                                    .get_declaration()
                                    .unwrap()
                                    .get_typedef_underlying_type()
                                    .unwrap()
                                    .get_kind()
                                {
                                    TypeKind::CharS => (
                                        format!("{} : std::os::raw::c_char", arg_name),
                                        format!("{} : * const std::os::raw::c_char", arg_name,),
                                        arg_name.clone(),
                                        format!("{arg_name}:{arg_name}"),
                                    ),
                                    _ => panic!(""),
                                }
                            }
                            TypeKind::Int => (
                                format!("{} : std::os::raw::c_int", arg_name),
                                format!("{} : std::os::raw::c_int", arg_name,),
                                arg_name.clone(),
                                format!("{arg_name}:{arg_name}"),
                            ),
                            TypeKind::Bool => (
                                format!("{} : bool", arg_name),
                                format!("{} : bool", arg_name,),
                                arg_name.clone(),
                                format!("{arg_name}:{arg_name}"),
                            ),
                            _ => {
                                println!("kind={:?}", tp.get_kind());
                                panic!("");
                            }
                        }
                    })
                    .collect::<Vec<_>>();
            }
            let trait_line = format!(
                "fn {snake_fn_name}(&mut self, {}) {{}}\n",
                arg_list
                    .iter()
                    .map(|arg| { arg.0.clone() })
                    .collect::<Vec<_>>()
                    .join(",")
            );
            trait_lines.push(trait_line.clone());
            vtable_lines.push(format!(
                r#"{snake_fn_name}: extern "C" fn(spi: *mut {full_spi_name}Fat, {} ),
                "#,
                arg_list
                    .iter()
                    .map(|arg| { arg.1.clone() })
                    .collect::<Vec<_>>()
                    .join(",")
            ));
            static_table_lines.push(format!(
                r#"{snake_fn_name}: spi_{snake_fn_name},
            "#
            ));
            spi_output_enum_lines.push(format!(r#"{fn_name}({full_spi_name}{fn_name}Packet),"#,));
            spi_output_enum_struct_lines.push(format!(
                r#"
            #[derive(Clone, Debug)]
            pub struct {full_spi_name}{fn_name}Packet {{
                {}
            }}"#,
                arg_list
                    .iter()
                    .map(|arg| { format!("pub {},", arg.0.replace("&", "").clone()) })
                    .collect::<Vec<_>>()
                    .join("")
            ));
            let trait_line_front = trait_line.replace("{}", "");
            impl_spi_fn_line.push(format!(
                r#"{trait_line_front} {{
            self.inner.lock().unwrap().push({})
                }}
            "#,
                format!(r#"{full_spi_output_enum_name}::{fn_name}( {full_spi_name}{fn_name}Packet {{ {} }} )"#, 
                arg_list.iter().map(|arg|{arg.3.clone()}).collect::<Vec<_>>().join(","))
            ));

            c_fn_lines.push(format!(
                r#"extern "C" fn spi_{}(spi: *mut {}Fat, {}) {{
                    unsafe {{
                        (*(*spi).md_spi_ptr).{}({})
                    }}
                }}"#,
                snake_fn_name,
                full_spi_name,
                arg_list
                    .iter()
                    .map(|arg| { arg.1.clone() })
                    .collect::<Vec<_>>()
                    .join(","),
                snake_fn_name,
                arg_list
                    .iter()
                    .map(|arg| { arg.2.clone() })
                    .collect::<Vec<_>>()
                    .join(","),
            ));
        }

        fat_spi_code = format!(
            r#"
        #[repr(C)]
        pub struct {full_spi_name}Fat {{
            vtable: *const {vtable_struct_name},
            pub md_spi_ptr: *mut dyn {full_trait_name},
        }}
        "#
        );
        spi_stream_code = format!(
            r#"
        use futures::stream::Stream;
        use std::{{
            pin::Pin,
            sync::{{Arc, Mutex}},
            task::Waker,
        }};
        
        struct {full_spi_name}Inner {{
            buf: std::collections::VecDeque<{full_spi_output_enum_name}>,
            waker: Option<Waker>,
        }}
        
        impl {full_spi_name}Inner {{
            fn push(&mut self, msg: {full_spi_output_enum_name}) {{
                self.buf.push_back(msg);
                if let Some(ref waker) = &self.waker {{
                    waker.clone().wake()
                }}
            }}
        }}
        
        pub struct {full_spi_name}Stream {{
            inner: Arc<Mutex<{full_spi_name}Inner>>,
        }}
        
        impl Stream for {full_spi_name}Stream {{
            type Item = {full_spi_output_enum_name};
        
            fn poll_next(
                self: Pin<&mut Self>,
                cx: &mut futures::task::Context<'_>,
            ) -> futures::task::Poll<Option<Self::Item>> {{
                use futures::task::Poll;
                let mut inner = self.inner.lock().unwrap();
                if let Some(i) = inner.buf.pop_front() {{
                    Poll::Ready(Some(i))
                }} else {{
                    inner.waker = Some(cx.waker().clone());
                    Poll::Pending
                }}
            }}
        
            fn size_hint(&self) -> (usize, Option<usize>) {{
                (0, None)
            }}
        }}
        
        pub fn create_spi() -> (Box<{full_spi_name}Stream>, *mut {full_spi_name}Stream) {{
            let i = {full_spi_name}Inner {{
                buf: std::collections::VecDeque::new(),
                waker: None,
            }};
            let xspi = {full_spi_name}Stream {{
                inner: Arc::new(Mutex::new(i)),
            }};
            let myspi = Box::new(xspi);
            let pp = Box::into_raw(myspi);
            let pp2 = pp.clone();
            (unsafe {{ Box::from_raw(pp2) }}, pp)
        }}
        "#
        );
        return EntityVisitResult::Break;
    };
    tu.get_entity().visit_children(vf);

    format!(
        r#"{} }}
{} }} 
{} }} 
{}  
{} }};
{}
{}
{}
{} }}
"#,
        trait_lines.join(""),
        vtable_lines.join(""),
        spi_output_enum_lines.join(""),
        spi_output_enum_struct_lines.join(""),
        static_table_lines.join(""),
        c_fn_lines.join(""),
        fat_spi_code,
        spi_stream_code,
        impl_spi_fn_line.join("")
    )
}

use bindgen::callbacks::{DeriveInfo, ParseCallbacks};

#[derive(Debug)]
struct MyCallback {}

impl ParseCallbacks for MyCallback {
    // Test the "custom derives" capability by adding `PartialEq` to the `Test` struct.
    fn add_derives(&self, info: &DeriveInfo<'_>) -> Vec<String> {
        if info.name.starts_with("CThost")
            && !info.name.contains("Api")
            && !info.name.contains("Spi")
        {
            vec!["Decode".into(), "Encode".into()]
        } else {
            vec![]
        }
    }
}

fn main() {
    clang_sys::load().expect("");
    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, false, false);
    let tu = index.parser("wrapper.hpp").parse().unwrap();

    // 开着这个的话会导致每次重新编译
    let mut f = File::create("./src/md_impl.rs").expect("unable to create file");
    let code = parse_api(&tu, "CThostFtdcMdApi");
    f.write(code.as_bytes()).unwrap();
    let code = parse_spi(&tu, "CThostFtdcMdSpi");
    f.write(code.as_bytes()).unwrap();

    let mut f = File::create("./src/trade_impl.rs").expect("unable to create file");
    let code = parse_api(&tu, "CThostFtdcTraderApi");
    f.write(code.as_bytes()).unwrap();
    let code = parse_spi(&tu, "CThostFtdcTraderSpi");
    f.write(code.as_bytes()).unwrap();

    println!("cargo:rustc-link-search=./crates/ctp-futures/v_current");
    let dir = var("CARGO_MANIFEST_DIR").unwrap();
    let library_path = Path::new(&dir).join("v_current");
    println!("cargo:rustc-link-search=native={}", library_path.display());
    if cfg!(windows) {
        let output = var("OUT_DIR").unwrap();
        std::fs::copy(
            library_path.join("thostmduserapi_se.dll"),
            Path::new(&output)
                .join("..")
                .join("..")
                .join("..")
                .join("thostmduserapi_se.dll"),
        )
        .unwrap();
        std::fs::copy(
            library_path.join("thosttraderapi_se.dll"),
            Path::new(&output)
                .join("..")
                .join("..")
                .join("..")
                .join("thosttraderapi_se.dll"),
        )
        .unwrap();
    } else if cfg!(unix) {
        let output = var("OUT_DIR").unwrap();
        std::fs::copy(
            library_path.join("libthostmduserapi_se.so"),
            Path::new(&output)
                .join("..")
                .join("..")
                .join("..")
                .join("libthostmduserapi_se.so"),
        )
        .unwrap();
        std::fs::copy(
            library_path.join("libthosttraderapi_se.so"),
            Path::new(&output)
                .join("..")
                .join("..")
                .join("..")
                .join("libthosttraderapi_se.so"),
        )
        .unwrap();
    } else {
        println!("cargo:rustc-env=LD_LIBRARY_PATH={}", library_path.display());
    }

    println!("cargo:rustc-link-lib=thosttraderapi_se");
    println!("cargo:rustc-link-lib=thostmduserapi_se");

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .clang_arg("-x")
        .clang_arg("c++")
        .derive_default(true)
        .derive_debug(true)
        .vtable_generation(true)
        .generate_comments(false) // 不需形成doc ,默认true
        .layout_tests(false) //不需要test,默认true
        .generate_comments(false) //不需注释,默认true
        .derive_copy(true)
        .derive_hash(false) //不要实现hash
        .parse_callbacks(Box::new(MyCallback {}))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let binding_file = out_path.join("bindings.rs");
    bindings
        .write_to_file(&binding_file)
        .expect("Couldn't write bindings!");
}
