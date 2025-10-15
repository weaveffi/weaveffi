use weaveffi_ir::ir::{Api, Module, Param, TypeRef};

fn c_type_for_param(ty: &TypeRef, name: &str) -> String {
    match ty {
        TypeRef::I32 => format!("int32_t {}", name),
        TypeRef::U32 => format!("uint32_t {}", name),
        TypeRef::I64 => format!("int64_t {}", name),
        TypeRef::F64 => format!("double {}", name),
        TypeRef::Bool => format!("bool {}", name),
        TypeRef::StringUtf8 => format!("const uint8_t* {}_ptr, size_t {}_len", name, name),
        TypeRef::Bytes => format!("const uint8_t* {}_ptr, size_t {}_len", name, name),
        TypeRef::Handle => format!("weaveffi_handle_t {}", name),
    }
}

fn c_ret_type_for(ty: &TypeRef) -> (&'static str, bool) {
    match ty {
        TypeRef::I32 => ("int32_t", false),
        TypeRef::U32 => ("uint32_t", false),
        TypeRef::I64 => ("int64_t", false),
        TypeRef::F64 => ("double", false),
        TypeRef::Bool => ("bool", false),
        TypeRef::StringUtf8 => ("const char*", false),
        TypeRef::Bytes => ("const uint8_t*", true), // requires out_len param
        TypeRef::Handle => ("weaveffi_handle_t", false),
    }
}

fn c_symbol_name(module: &str, func: &str) -> String {
    format!("weaveffi_{}_{}", module, func)
}

fn c_params_sig(params: &[Param]) -> Vec<String> {
    params
        .iter()
        .map(|p| c_type_for_param(&p.ty, &p.name))
        .collect()
}

pub fn render_c_header(api: &Api) -> String {
    let mut out = String::new();
    out.push_str("#ifndef WEAVEFFI_H\n");
    out.push_str("#define WEAVEFFI_H\n\n");
    out.push_str("#include <stdint.h>\n");
    out.push_str("#include <stddef.h>\n");
    out.push_str("#include <stdbool.h>\n\n");
    out.push_str("#ifdef __cplusplus\nextern \"C\" {\n#endif\n\n");

    out.push_str("typedef uint64_t weaveffi_handle_t;\n\n");
    out.push_str("typedef struct weaveffi_error { int32_t code; const char* message; } weaveffi_error;\n\n");
    out.push_str("void weaveffi_error_clear(weaveffi_error* err);\n");
    out.push_str("void weaveffi_free_string(const char* ptr);\n");
    out.push_str("void weaveffi_free_bytes(uint8_t* ptr, size_t len);\n\n");

    for m in &api.modules {
        render_module_header(&mut out, m);
    }

    out.push_str("\n#ifdef __cplusplus\n}\n#endif\n\n");
    out.push_str("#endif // WEAVEFFI_H\n");
    out
}

fn render_module_header(out: &mut String, module: &Module) {
    out.push_str(&format!("// Module: {}\n", module.name));
    for f in &module.functions {
        let mut params_sig = c_params_sig(&f.params);
        // If return bytes, append out_len before out_err
        let ret_sig = if let Some(ret) = &f.returns {
            let (ret_ty, needs_len) = c_ret_type_for(ret);
            if needs_len { params_sig.push("size_t* out_len".to_string()); }
            ret_ty.to_string()
        } else {
            "void".to_string()
        };
        params_sig.push("weaveffi_error* out_err".to_string());
        let fn_name = c_symbol_name(&module.name, &f.name);
        out.push_str(&format!("{} {}({});\n", ret_sig, fn_name, params_sig.join(", ")));
    }
    out.push_str("\n");
}

pub fn render_c_convenience_c() -> String {
    let mut out = String::new();
    out.push_str("#include \"weaveffi.h\"\n\n");
    out.push_str("// Optional convenience wrappers can be added here in future versions.\n");
    out
}

pub fn render_wasm_readme() -> String {
    let mut out = String::new();
    out.push_str("# WeaveFFI WASM (experimental)\n\n");
    out.push_str("This folder contains a minimal stub to help you load a `wasm32-unknown-unknown` build of your WeaveFFI library.\n\n");
    out.push_str("Build (example):\n\n");
    out.push_str("```") ; out.push_str("bash\n");
    out.push_str("cargo build --target wasm32-unknown-unknown --release\n");
    out.push_str("``" ); out.push_str("\n\n");
    out.push_str("Then serve the `.wasm` and use `weaveffi_wasm.js` to load it.\n");
    out
}

pub fn render_wasm_js_stub() -> String {
    let mut out = String::new();
    out.push_str("// Minimal JS loader for WeaveFFI WASM\n");
    out.push_str("export async function loadWeaveFFI(url) {\n");
    out.push_str("  const response = await fetch(url);\n");
    out.push_str("  const bytes = await response.arrayBuffer();\n");
    out.push_str("  const { instance } = await WebAssembly.instantiate(bytes, {});\n");
    out.push_str("  return instance.exports;\n");
    out.push_str("}\n");
    out
}

fn swift_type_for(t: &TypeRef) -> &'static str {
    match t {
        TypeRef::I32 => "Int32",
        TypeRef::U32 => "UInt32",
        TypeRef::I64 => "Int64",
        TypeRef::F64 => "Double",
        TypeRef::Bool => "Bool",
        TypeRef::StringUtf8 => "String",
        TypeRef::Bytes => "Data",
        TypeRef::Handle => "UInt64",
    }
}

fn swift_call_args_for_params(params: &[Param]) -> String {
    let mut out: Vec<String> = Vec::new();
    for p in params {
        match &p.ty {
            TypeRef::StringUtf8 | TypeRef::Bytes => {
                // strings/bytes use pointer + len
                out.push(format!("{}_ptr", p.name));
                out.push(format!("{}_len", p.name));
            }
            _ => {
                out.push(p.name.clone());
            }
        }
    }
    out.join(", ")
}

fn swift_prep_params(params: &[Param]) -> String {
    let mut out = String::new();
    for p in params {
        match &p.ty {
            TypeRef::StringUtf8 => {
                out.push_str(&format!(
                    "        let {n}_bytes = Array({n}.utf8)\n        let {n}_ptr = UnsafePointer<UInt8>({n}_bytes)\n        let {n}_len = {n}_bytes.count\n",
                    n = p.name,
                ));
            }
            TypeRef::Bytes => {
                out.push_str(&format!(
                    "        let {n}_ptr = {n}.withUnsafeBytes {{ (raw: UnsafeRawBufferPointer) in\n            return raw.bindMemory(to: UInt8.self).baseAddress\n        }}\n        let {n}_len = {n}.count\n",
                    n = p.name,
                ));
            }
            _ => {}
        }
    }
    out
}

fn swift_return_postprocess(ret: Option<&TypeRef>) -> (String, String) {
    match ret {
        None => (String::from(""), String::from("")),
        Some(TypeRef::StringUtf8) => (
            String::from("        defer { weaveffi_free_string(rv) }\n        guard let rv = rv else { throw WeaveFFIError.error(code: -1, message: \"null string\") }\n        return String(cString: rv)\n"),
            String::from("let rv")
        ),
        Some(TypeRef::Bytes) => (
            String::from("        // Returned bytes not yet wrapped to Swift Data in this template\n        return ()\n"),
            String::from("let _")
        ),
        Some(_) => (
            String::from("        return rv\n"),
            String::from("let rv")
        ),
    }
}

pub fn render_swift_wrapper(api: &Api) -> String {
    let mut out = String::new();
    out.push_str("import WeaveFFI\n\n");
    out.push_str("public enum WeaveFFIError: Error, CustomStringConvertible {\n    case error(code: Int32, message: String)\n    public var description: String {\n        switch self { case let .error(code, message): return \"(\\(code)) \\ (message)\" }\n    }\n}\n\n");
    out.push_str("@inline(__always)\nfunc check(_ err: inout weaveffi_error) throws {\n    if err.code != 0 {\n        let message = err.message.flatMap { String(cString: $0) } ?? \"\"\n        weaveffi_error_clear(&err)\n        throw WeaveFFIError.error(code: err.code, message: message)\n    }\n}\n\n");
    for m in &api.modules {
        let type_name = to_camel(&m.name);
        out.push_str(&format!("public enum {} {{\n", type_name));
        for f in &m.functions {
            // signature
            let mut params_sig: Vec<String> = Vec::new();
            for p in &f.params {
                params_sig.push(format!("{} {}", p.name, swift_type_for(&p.ty)));
            }
            let ret_swift = f.returns.as_ref().map(|t| swift_type_for(t)).unwrap_or("Void");
            out.push_str(&format!("    public static func {}({}) throws -> {} {{\n", f.name, params_sig.join(", "), ret_swift));
            out.push_str("        var err = weaveffi_error(code: 0, message: nil)\n");
            out.push_str(&swift_prep_params(&f.params));
            let call_args = swift_call_args_for_params(&f.params);
            // bytes return not implemented ergonomically yet; keep placeholder
            let (ret_post, let_rv) = swift_return_postprocess(f.returns.as_ref());
            match f.returns.as_ref() {
                None => {
                    out.push_str(&format!("        {}( {}, &err )\n", c_symbol_name(&m.name, &f.name), call_args));
                    out.push_str("        try check(&err)\n");
                    out.push_str("    }\n");
                }
                Some(_) => {
                    out.push_str(&format!("        {} = {}( {}, &err )\n", let_rv, c_symbol_name(&m.name, &f.name), call_args));
                    out.push_str("        try check(&err)\n");
                    out.push_str(&ret_post);
                    out.push_str("    }\n");
                }
            }
        }
        out.push_str("}\n\n");
    }
    out
}

fn to_camel(s: &str) -> String {
    let mut it = s.split('_');
    let mut out = String::new();
    if let Some(first) = it.next() { out.push_str(&first[..1].to_uppercase()); out.push_str(&first[1..]); }
    for part in it { out.push_str(&part[..1].to_uppercase()); out.push_str(&part[1..]); }
    out
}

fn ffi_napi_type_for(t: &TypeRef) -> &'static str {
    match t {
        TypeRef::I32 => "int",
        TypeRef::U32 => "uint",
        TypeRef::I64 => "int64",
        TypeRef::F64 => "double",
        TypeRef::Bool => "bool",
        TypeRef::StringUtf8 => "CString", // const char*
        TypeRef::Bytes => "pointer",      // const uint8_t*
        TypeRef::Handle => "uint64",
    }
}

pub fn render_node_index_ts(api: &Api) -> String {
    let mut out = String::new();
    out.push_str("import ffi from 'ffi-napi'\n");
    out.push_str("import ref from 'ref-napi'\n\n");
    out.push_str("const libPath = process.env.WEAVEFFI_LIB || './libweaveffi.dylib'\n");
    out.push_str("const CString = ref.types.CString as any\n");
    out.push_str("const bool = ref.types.bool as any\n");
    out.push_str("const uint = ref.types.uint as any\n");
    out.push_str("const int = ref.types.int as any\n");
    out.push_str("const int64 = ref.types.int64 as any\n");
    out.push_str("const uint64 = ref.types.uint64 as any\n");
    out.push_str("const double = ref.types.double as any\n");
    out.push_str("const size_t = ref.types.size_t as any\n");
    out.push_str("const pointer = ref.refType(ref.types.void) as any\n\n");
    out.push_str("export const lib = ffi.Library(libPath, {\n");
    // memory helpers
    out.push_str("  'weaveffi_free_string': ['void', [CString]],\n");
    out.push_str("  'weaveffi_free_bytes': ['void', [pointer, size_t]],\n");
    out.push_str("  'weaveffi_error_clear': ['void', [pointer]],\n");
    for m in &api.modules {
        for f in &m.functions {
            let sym = c_symbol_name(&m.name, &f.name);
            let (ret_ty, needs_len) = f.returns.as_ref().map(|t| c_ret_type_for(t)).unwrap_or(("void", false));
            let ts_ret = match ret_ty { "const char*" => "CString", "const uint8_t*" => "pointer", "void" => "void", other if other == "bool" => "bool", other => other };
            let mut args: Vec<String> = Vec::new();
            for p in &f.params {
                match p.ty {
                    TypeRef::StringUtf8 | TypeRef::Bytes => { args.push("pointer".into()); args.push("size_t".into()); }
                    _ => { args.push(ffi_napi_type_for(&p.ty).into()); }
                }
            }
            if needs_len { args.push("pointer".into()); }
            args.push("pointer".into()); // out_err
            // Return type: use variable (e.g., CString, int, pointer) except for 'void' which is allowed as a string
            if ts_ret == "void" {
                out.push_str(&format!("  '{}': ['void', [{}]],\n", sym, args.join(", ")));
            } else {
                out.push_str(&format!("  '{}': [{}, [{}]],\n", sym, ts_ret, args.join(", ")));
            }
        }
    }
    out.push_str("})\n\n");
    out.push_str("export default lib\n");
    out
}

pub fn render_node_dts(api: &Api) -> String {
    let mut out = String::from("// Generated types for WeaveFFI functions\n");
    for m in &api.modules {
        out.push_str(&format!("// module {}\n", m.name));
        for f in &m.functions {
            let mut params: Vec<String> = Vec::new();
            for p in &f.params {
                let ts = match p.ty {
                    TypeRef::I32 | TypeRef::U32 | TypeRef::I64 | TypeRef::F64 => "number",
                    TypeRef::Bool => "boolean",
                    TypeRef::StringUtf8 => "string",
                    TypeRef::Bytes => "Buffer",
                    TypeRef::Handle => "number",
                };
                params.push(format!("{}: {}", p.name, ts));
            }
            let ret = match f.returns.as_ref() {
                None => "void",
                Some(TypeRef::I32 | TypeRef::U32 | TypeRef::I64 | TypeRef::F64) => "number",
                Some(TypeRef::Bool) => "boolean",
                Some(TypeRef::StringUtf8) => "string",
                Some(TypeRef::Bytes) => "Buffer",
                Some(TypeRef::Handle) => "number",
            };
            out.push_str(&format!("export function {}({}): {}\n", f.name, params.join(", "), ret));
        }
    }
    out
}
