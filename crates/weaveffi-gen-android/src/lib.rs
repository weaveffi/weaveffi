use anyhow::Result;
use camino::Utf8Path;
use tracing::info;
use weaveffi_core::codegen::Generator;
use weaveffi_ir::ir::{Api, TypeRef};
use std::fmt::Write as _;

pub struct AndroidGenerator;

impl Generator for AndroidGenerator {
    fn name(&self) -> &'static str { "android" }
    fn generate(&self, _api: &Api, out_dir: &Utf8Path) -> Result<()> {
        info!("generating Android JNI + Gradle template");
        let dir = out_dir.join("android");
        std::fs::create_dir_all(&dir)?;
        // settings.gradle
        let settings = "rootProject.name = 'weaveffi'\n";
        std::fs::write(dir.join("settings.gradle"), settings)?;
        // build.gradle (library)
        let build_gradle = r#"plugins {
    id 'com.android.library'
    id 'org.jetbrains.kotlin.android' version '1.9.22' apply false
}

android {
    namespace 'com.weaveffi'
    compileSdk 34
    defaultConfig { minSdk 24 }
}
"#;
        std::fs::write(dir.join("build.gradle"), build_gradle)?;
        // Kotlin wrapper stub
        let src_dir = dir.join("src/main/java/com/weaveffi");
        std::fs::create_dir_all(&src_dir)?;
        let mut kotlin = String::from("package com.weaveffi\n\nclass WeaveFFI {\n    companion object {\n        init { System.loadLibrary(\"weaveffi\") }\n\n");
        for m in &_api.modules {
            for f in &m.functions {
                let mut params_sig: Vec<String> = Vec::new();
                for p in &f.params {
                    params_sig.push(format!("{}: {}", p.name, kotlin_type(&p.ty)));
                }
                let ret = f.returns.as_ref().map(kotlin_type).unwrap_or("Unit");
                writeln!(kotlin, "        @JvmStatic external fun {}({}): {}", f.name, params_sig.join(", "), ret).ok();
            }
        }
        kotlin.push_str("    }\n}\n");
        std::fs::write(src_dir.join("WeaveFFI.kt"), kotlin)?;
        // C JNI shim sample and CMakeLists
        let jni_dir = dir.join("src/main/cpp");
        std::fs::create_dir_all(&jni_dir)?;
        let cmake = r#"cmake_minimum_required(VERSION 3.22)
project(weaveffi)
add_library(weaveffi SHARED weaveffi_jni.c)
target_include_directories(weaveffi PRIVATE ../../../../c)
"#;
        std::fs::write(jni_dir.join("CMakeLists.txt"), cmake)?;
        let mut jni_c = String::from("#include <jni.h>\n#include <stdbool.h>\n#include <stdint.h>\n#include <stddef.h>\n#include \"weaveffi.h\"\n\n");
        for m in &_api.modules {
            for f in &m.functions {
                // Signature
                let jret = jni_ret_type(f.returns.as_ref());
                let mut jparams: Vec<String> = Vec::new();
                jparams.push("JNIEnv* env".into());
                jparams.push("jclass clazz".into());
                for p in &f.params {
                    jparams.push(format!("{} {}", jni_param_type(&p.ty), p.name));
                }
                writeln!(jni_c, "JNIEXPORT {} JNICALL Java_com_weaveffi_WeaveFFI_{}({}) {{", jret, f.name, jparams.join(", ")).ok();
                // Prepare params
                writeln!(jni_c, "    weaveffi_error err = {{0, NULL}};").ok();
                for p in &f.params {
                    match p.ty {
                        TypeRef::StringUtf8 => {
                            writeln!(jni_c, "    const char* {n}_chars = (*env)->GetStringUTFChars(env, {n}, NULL);", n = p.name).ok();
                            writeln!(jni_c, "    jsize {n}_len = (*env)->GetStringUTFLength(env, {n});", n = p.name).ok();
                        }
                        TypeRef::Bytes => {
                            writeln!(jni_c, "    jboolean {n}_is_copy = 0;", n = p.name).ok();
                            writeln!(jni_c, "    jbyte* {n}_elems = (*env)->GetByteArrayElements(env, {n}, &{n}_is_copy);", n = p.name).ok();
                            writeln!(jni_c, "    jsize {n}_len = (*env)->GetArrayLength(env, {n});", n = p.name).ok();
                        }
                        _ => {}
                    }
                }
                // Call underlying C function
                let c_sym = format!("weaveffi_{}_{}", m.name, f.name);
                let mut call_args: Vec<String> = Vec::new();
                for p in &f.params {
                    match p.ty {
                        TypeRef::StringUtf8 => {
                            call_args.push(format!("(const uint8_t*){n}_chars", n = p.name));
                            call_args.push(format!("(size_t){n}_len", n = p.name));
                        }
                        TypeRef::Bytes => {
                            call_args.push(format!("(const uint8_t*){n}_elems", n = p.name));
                            call_args.push(format!("(size_t){n}_len", n = p.name));
                        }
                        TypeRef::Bool => call_args.push(format!("(bool)({} == JNI_TRUE)", p.name)),
                        TypeRef::I32 => call_args.push(format!("(int32_t){}", p.name)),
                        TypeRef::U32 => call_args.push(format!("(uint32_t){}", p.name)),
                        TypeRef::I64 => call_args.push(format!("(int64_t){}", p.name)),
                        TypeRef::F64 => call_args.push(format!("(double){}", p.name)),
                        TypeRef::Handle => call_args.push(format!("(weaveffi_handle_t){}", p.name)),
                    }
                }
                let needs_len = matches!(f.returns, Some(TypeRef::Bytes));
                if needs_len {
                    writeln!(jni_c, "    size_t out_len = 0;").ok();
                }
                if f.returns.is_none() {
                    writeln!(jni_c, "    {}( {}, &err );", c_sym, call_args.join(", ")).ok();
                    write_error_throw(&mut jni_c);
                    writeln!(jni_c, "    return;").ok();
                } else {
                    match f.returns.as_ref().unwrap() {
                        TypeRef::StringUtf8 => {
                            writeln!(jni_c, "    const char* rv = {}( {}, &err );", c_sym, call_args.join(", ")).ok();
                            write_error_throw(&mut jni_c);
                            writeln!(jni_c, "    jstring out = rv ? (*env)->NewStringUTF(env, rv) : (*env)->NewStringUTF(env, \"\");").ok();
                            writeln!(jni_c, "    weaveffi_free_string(rv);").ok();
                            writeln!(jni_c, "    return out;").ok();
                        }
                        TypeRef::Bytes => {
                            // Append out_len before err
                            let mut args = call_args.clone();
                            args.push("&out_len".into());
                            writeln!(jni_c, "    const uint8_t* rv = {}( {}, &err );", c_sym, args.join(", ")).ok();
                            write_error_throw(&mut jni_c);
                            writeln!(jni_c, "    jbyteArray out = (*env)->NewByteArray(env, (jsize)out_len);").ok();
                            writeln!(jni_c, "    if (out && rv) {{ (*env)->SetByteArrayRegion(env, out, 0, (jsize)out_len, (const jbyte*)rv); }}").ok();
                            writeln!(jni_c, "    weaveffi_free_bytes((uint8_t*)rv, (size_t)out_len);").ok();
                            writeln!(jni_c, "    return out;").ok();
                        }
                        TypeRef::Bool => {
                            writeln!(jni_c, "    bool rv = {}( {}, &err );", c_sym, call_args.join(", ")).ok();
                            write_error_throw(&mut jni_c);
                            writeln!(jni_c, "    return rv ? JNI_TRUE : JNI_FALSE;").ok();
                        }
                        TypeRef::I32 | TypeRef::U32 | TypeRef::I64 | TypeRef::F64 | TypeRef::Handle => {
                            let jcast = match f.returns.as_ref().unwrap() {
                                TypeRef::I32 | TypeRef::U32 => "(jint)",
                                TypeRef::I64 | TypeRef::Handle => "(jlong)",
                                TypeRef::F64 => "(jdouble)",
                                _ => "",
                            };
                            writeln!(jni_c, "    auto rv = {}( {}, &err );", c_sym, call_args.join(", ")).ok();
                            write_error_throw(&mut jni_c);
                            writeln!(jni_c, "    return {} rv;", jcast).ok();
                        }
                    }
                }
                // Release params
                for p in &f.params {
                    match p.ty {
                        TypeRef::StringUtf8 => {
                            writeln!(jni_c, "    (*env)->ReleaseStringUTFChars(env, {n}, {n}_chars);", n = p.name).ok();
                        }
                        TypeRef::Bytes => {
                            writeln!(jni_c, "    (*env)->ReleaseByteArrayElements(env, {n}, {n}_elems, 0);", n = p.name).ok();
                        }
                        _ => {}
                    }
                }
                writeln!(jni_c, "}}\n").ok();
            }
        }
        std::fs::write(jni_dir.join("weaveffi_jni.c"), jni_c)?;
        Ok(())
    }
}

fn kotlin_type(t: &TypeRef) -> &'static str {
    match t {
        TypeRef::I32 => "Int",
        TypeRef::U32 => "Int",
        TypeRef::I64 => "Long",
        TypeRef::F64 => "Double",
        TypeRef::Bool => "Boolean",
        TypeRef::StringUtf8 => "String",
        TypeRef::Bytes => "ByteArray",
        TypeRef::Handle => "Long",
    }
}

fn jni_param_type(t: &TypeRef) -> &'static str {
    match t {
        TypeRef::I32 | TypeRef::U32 => "jint",
        TypeRef::I64 | TypeRef::Handle => "jlong",
        TypeRef::F64 => "jdouble",
        TypeRef::Bool => "jboolean",
        TypeRef::StringUtf8 => "jstring",
        TypeRef::Bytes => "jbyteArray",
    }
}

fn jni_ret_type(t: Option<&TypeRef>) -> &'static str {
    match t {
        None => "void",
        Some(TypeRef::I32 | TypeRef::U32) => "jint",
        Some(TypeRef::I64 | TypeRef::Handle) => "jlong",
        Some(TypeRef::F64) => "jdouble",
        Some(TypeRef::Bool) => "jboolean",
        Some(TypeRef::StringUtf8) => "jstring",
        Some(TypeRef::Bytes) => "jbyteArray",
    }
}

fn write_error_throw(out: &mut String) {
    let _ = writeln!(out, "    if (err.code != 0) {{");
    let _ = writeln!(out, "        jclass exClass = (*env)->FindClass(env, \"java/lang/RuntimeException\");");
    let _ = writeln!(out, "        const char* msg = err.message ? err.message : \"WeaveFFI error\";");
    let _ = writeln!(out, "        (*env)->ThrowNew(env, exClass, msg);");
    let _ = writeln!(out, "        weaveffi_error_clear(&err);");
    let _ = writeln!(out, "    }}");
}
