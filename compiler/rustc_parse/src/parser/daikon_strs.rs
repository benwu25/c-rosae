// FIXME: add one test for each String.

// Proper primitive types
// i8, i16, i32, i64, i128 and isize
// u8, u16, u32, u64, u128 and usize
// f32, f64
// char
// bool
// () -- why is this possible for parameters :/

pub static I8: &str = "i8";
pub static I16: &str = "i16";
pub static I32: &str = "i32";
pub static I64: &str = "i64";
pub static I128: &str = "i128";
pub static ISIZE: &str = "isize";

pub static U8: &str = "u8";
pub static U16: &str = "u16";
pub static U32: &str = "u32";
pub static U64: &str = "u64";
pub static U128: &str = "u128";
pub static USIZE: &str = "usize";

pub static F32: &str = "f32";
pub static F64: &str = "f64";

pub static CHAR: &str = "char";
pub static BOOL: &str = "bool";
pub static UNIT: &str = "()";
pub static STR: &str = "str";
pub static STRING: &str = "String";
pub static VEC: &str = "Vec";

// Initialize a function-local nonce counter.
// (no arguments)
pub(crate) static INIT_NONCE: &str = "fn main() { let mut __daikon_nonce = 0;
let mut __unwrap_nonce = NONCE_COUNTER.lock().unwrap();
__daikon_nonce = *__unwrap_nonce;
*__unwrap_nonce += 1;
drop(__unwrap_nonce);
}";

// Build dtrace entry call for each ppt-enter.
// $1: ppt-name (e.g. MAIN, FOO)
pub(crate) static DTRACE_ENTRY: &str = "
fn main() {
    dtrace_entry(\"${ppt_name}:::ENTER\", __daikon_nonce);
}";

// Build dtrace exit call for each ppt-exit.
// $1: ppt-name.
// $2: number corresponding to this exit ppt, unique for each within the function.
pub(crate) static DTRACE_EXIT: &str = "
fn main() {
    dtrace_exit(\"${ppt_name}:::EXIT${exit_num}\", __daikon_nonce);
}";

// Build a call to log a primitive value.
// $1: Primitive type of the variable.
// $2: var identifier.
pub(crate) static DTRACE_PRIM: &str = "
fn main() {
    dtrace_print_prim::<${type}>(${var_name}, String::from(\"${name}\"));
}";

// Build a log statement for a primitive return value.
// $1: Primitive type of the return value.
pub(crate) static DTRACE_PRIM_RET: &str = "
fn main() {
    dtrace_print_prim::<${type}>(__daikon_ret, String::from(\"return\"));
}";

// Build log statement for a variable with primitive reference type.
// $1: Primitive type of the variable.
// $2: Identifier of the variable.
// $3: Identifier of the variable (but different).
pub(crate) static DTRACE_PRIM_REF: &str = "
fn main() {
    dtrace_print_prim::<${type}>(${type}::from_str(&${var_name}.to_string()).expect(\"Ok\"),
                                 String::from(\"${var_name_2}\"));
}";

// Build log statement for variable of String-like type (&str, etc.).
// $1: Identifier for variable.
// $2: Identifier for variable (but different).
pub(crate) static DTRACE_PRIM_TOSTRING: &str = "
fn main() {
    dtrace_print_string(${var_name}.to_string(), String::from(\"${var_name_2}\"));
}";

// Build log stmt for a String field.
// $1: Field name.
pub(crate) static DTRACE_PRIM_FIELD_TOSTRING: &str =
    "dtrace_print_string(self.${field_name}.to_string(),
                         format!(\"{}{}\", prefix, \".${field_name}\"));";

// Build log stmt for primitive field (non-string type).
// $1: Primitive type of field.
// $2: Field name.
pub(crate) static DTRACE_PRIM_STRUCT: &str = "dtrace_print_prim::<${type}>(self.${field_name}, format!(\"{}{}\", prefix, \".${field_name}\"));";

// FIXME: if you have Vec<&'a &'b i32>, you will probably have to make a new Vec<i32> like this
//       to satisfy dtrace_print_prim_vec<T>(v: &Vec<T>).

// Build log stmt for fields with primitive reference type.
// $1: Underlying primitive type of field.
// $2: Field name.
pub(crate) static DTRACE_PRIM_REF_STRUCT: &str = "dtrace_print_prim::<${type}>(${type}::from_str(&self.${field_name}.to_string()).expect(\"Ok\"), format!(\"{}{}\", prefix, \".${field_name}\"));";

// Build log stmt for a struct variable.
// $1: Variable name.
// $2: Depth counter.
pub(crate) static DTRACE_USERDEF: &str = "fn main() {
    dtrace_print_pointer(${var_name} as *const _ as usize, String::from(\"${var_name}\"));
    ${var_name}.dtrace_print_fields(${depth}, String::from(\"${var_name}\"));
}";

// Build log stmt for struct variable, syntax difference.
// Args: See DTRACE_USERDEF.
pub(crate) static DTRACE_USERDEF_AMPERSAND: &str = "fn main() {
    dtrace_print_pointer(&${var_name} as *const _ as usize, String::from(\"${var_name}\"));
    ${var_name}.dtrace_print_fields(${depth}, String::from(\"${var_name}\"));
}";

pub(crate) static DTRACE_USERDEF_RET: &str = "
fn __skip() {
    dtrace_print_pointer(__daikon_ret as *const _ as usize, String::from(\"return\"));
    __daikon_ret.dtrace_print_fields(${depth}, String::from(\"return\"));
}";

pub(crate) static DTRACE_USERDEF_RET_AMPERSAND: &str = "
fn __skip() {
    dtrace_print_pointer(&__daikon_ret as *const _ as usize, String::from(\"return\"));
    __daikon_ret.dtrace_print_fields(${depth}, String::from(\"return\"));
}";

pub(crate) static DTRACE_USERDEF_STRUCT: &str =
    "dtrace_print_pointer(self.${field_name} as *const _ as usize, format!(\"{}{}\", prefix, \".${field_name}\"));
self.${field_name}.dtrace_print_fields(depth - 1, format!(\"{}{}\", prefix, \".${field_name}\"));";

pub(crate) static DTRACE_USERDEF_STRUCT_AMPERSAND: &str =
    "dtrace_print_pointer(&self.${field_name} as *const _ as usize, format!(\"{}{}\", prefix, \".${field_name}\"));
self.${field_name}.dtrace_print_fields(depth - 1, format!(\"{}{}\", prefix, \".${field_name}\"));";

// always with ampersand, we will always make a copy.
pub(crate) static DTRACE_USERDEF_VEC_FIELDS: &str = "${plain_struct}::dtrace_print_fields_vec(&${tmp_vec_name}, depth - 1, format!(\"{}{}\", prefix, \".${field_name}\"));";

pub(crate) static DTRACE_PRINT_XFIELD_VEC: &str = "${plain_struct}::dtrace_print_${field_name}_vec(&${var_name}, format!(\"{}{}\", prefix, \".${field_name}\"));";

pub(crate) static DTRACE_PRINT_POINTER_VEC_USERDEF: &str = "dtrace_print_pointer_vec::<${plain_struct}>(&${tmp_vec_name}, format!(\"{}{}\", prefix, \".${field_name}\"));";

// we're expecting a tmp vec loop before this.
pub(crate) static DTRACE_VEC_POINTER: &str =
    "dtrace_print_pointer(${var_name}.as_ptr() as usize, String::from(\"${var_name}\"));";

pub(crate) static DTRACE_VEC_POINTER_RET: &str =
    "dtrace_print_pointer(__daikon_ret.as_ptr() as usize, String::from(\"return\"));";

pub(crate) static DTRACE_PRINT_POINTER_VEC: &str = "dtrace_print_pointer_vec::<${type}>(&__daikon_tmp${tmp_uuid}, format!(\"{}{}\", String::from(\"${var_name}\"), \"[..]\"));";

pub(crate) static DTRACE_VEC_FIELDS: &str =
    // we always mash with a tmp loop, so just close __skip.
    "
    ${plain_struct}::dtrace_print_fields_vec(&${tmp_vec_name},
    3,
    format!(\"{}{}\", String::from(\"${var_name}\"), \"[..]\"));
}";

pub(crate) static DAIKON_TMP_VEC_USERDEF: &str =
    "let mut __daikon_tmp${first_tmp}: Vec<&${type}> = Vec::new();
let mut __daikon_tmp${next_tmp} = 0;
while __daikon_tmp${next_tmp} < v.len() {
    __daikon_tmp${first_tmp}.push(v[__daikon_tmp${next_tmp}]);
    __daikon_tmp${next_tmp} += 1;
}";

pub(crate) static DAIKON_TMP_VEC_USERDEF_FIELD: &str =
    "let mut __daikon_tmp${first_tmp}: Vec<&${type}> = Vec::new();
let mut __daikon_tmp${next_tmp} = 0;
while __daikon_tmp${next_tmp} < v.len() {
    __daikon_tmp${first_tmp}.push(v[__daikon_tmp${next_tmp}].${next_tmp});
    __daikon_tmp${next_tmp} += 1;
}";

pub(crate) static DAIKON_TMP_VEC_USERDEF_FIELD_AMPERSAND: &str =
    "let mut __daikon_tmp${first_tmp}: Vec<&${type}> = Vec::new();
let mut __daikon_tmp${next_tmp} = 0;
while __daikon_tmp${next_tmp} < v.len() {
    __daikon_tmp${first_tmp}.push(&v[__daikon_tmp${next_tmp}].${field_name});
    __daikon_tmp${next_tmp} += 1;
}";

// this will always be mashed with some subsequent call, so don't close __skip yet.
// the thing you mash it with must close __skip.
pub(crate) static DAIKON_TMP_VEC: &str = "
fn __skip() {
    let mut __daikon_tmp${first_tmp}: Vec<&${type}> = Vec::new();
    let mut __daikon_tmp${next_tmp} = 0;
    while __daikon_tmp${next_tmp} < ${var_name}.len() {
        __daikon_tmp${first_tmp}.push(${var_name}[__daikon_tmp${next_tmp}]);
        __daikon_tmp${next_tmp} += 1;
    }";

// FIXME: use this for params/returns where you have Vec<Type>.
pub(crate) static DAIKON_TMP_VEC_AMPERSAND: &str = "
fn __skip() {
    let mut __daikon_tmp${first_tmp}: Vec<&${type}> = Vec::new();
    let mut __daikon_tmp${first_tmp} = 0;
    while __daikon_tmp${next_tmp} < ${var_name}.len() {
        __daikon_tmp${first_tmp}.push(&${var_name}[__daikon_tmp${next_tmp}]);
        __daikon_tmp${next_tmp} += 1;
    }";

pub(crate) static DAIKON_TMP_VEC_PRIM: &str =
    "
fn __skip() {
    let mut __daikon_tmp${first_tmp}: Vec${type} = Vec::new();
    let mut __daikon_tmp${next_tmp} = 0;
    while __daikon_tmp${next_tmp} < ${var_name}.len() {
        __daikon_tmp${first_tmp}.push(${first_tmp}::from_str(&${var_name}[__daikon_tmp${next_tmp}].to_string()).expect(\"Ok\"));
        __daikon_tmp${next_tmp} += 1;
    }"; // don't close __skip() because we will mash

pub(crate) static DTRACE_TMP_VEC_FOR_FIELD: &str = "
    let mut __daikon_tmp${first_tmp}: Vec<&${type}> = Vec::new();
    let mut __daikon_tmp${next_tmp} = 0;
    while __daikon_tmp${next_tmp} < self.${field_name}.len() {
        __daikon_tmp${first_tmp}.push(self.${field_name}[__daikon_tmp${next_tmp}]);
        __daikon_tmp${next_tmp} += 1
    }";

// FIXME: use this for fields which are f: Vec<Type> or f: &Vec<Type>, need to use &.
pub(crate) static DTRACE_TMP_VEC_FOR_FIELD_AMPERSAND: &str = "
    let mut __daikon_tmp${first_tmp}: Vec<&${type}> = Vec::new();
    let mut __daikon_tmp${next_tmp} = 0;
    while __daikon_tmp${next_tmp} < self.${field_name}.len() {
        __daikon_tmp${first_tmp}.push(&self.${field_name}[__daikon_tmp${next_tmp}]);
        __daikon_tmp${next_tmp} += 1
}";

pub(crate) static DTRACE_POINTER_VEC_USERDEF: &str = "dtrace_print_pointer(self.${field_name}.as_ptr() as usize, format!(\"{}{}\", prefix, \".${field_name}\"));";

pub(crate) static DTRACE_POINTER_ARR_USERDEF: &str = "
    dtrace_print_pointer(self.${field_name} as *const _ as *const () as usize,
                         format!(\"{}{}\", prefix, \".${field_name}\"));";

pub(crate) static DTRACE_POINTERS_VEC_USERDEF: &str = "
dtrace_print_pointer_vec::<${type}>(&__daikon_tmp${tmp_uuid},
                                    format!(\"{}{}[..]\", prefix, \".${field_name}\"));";

pub(crate) static DTRACE_PRINT_FIELDS_FOR_FIELD: &str = "
    ${type}::dtrace_print_fields_vec(&__daikon_tmp${tmp_uuid},
                                     depth - 1,
                                     format!(\"{}{}[..]\", prefix, \".${field_name}\"));";

pub(crate) static DTRACE_PRINT_XFIELD_FOR_FIELD_PROLOGUE: &str = "
pub fn dtrace_print_${field_name}(&self, depth: i32, prefix: String) {";

pub(crate) static DTRACE_PRINT_XFIELD_FOR_FIELD_MID: &str = "
    if depth == 0 {
        return;
    }";

pub(crate) static DTRACE_PRINT_XFIELD_FOR_FIELD_EPILOGUE: &str = "
}";

pub(crate) static DTRACE_TMP_PRIM_VEC_FOR_FIELD: &str =
    "
    let mut __daikon_tmp${first_tmp}: Vec${type} = Vec::new();
    let mut __daikon_tmp${next_tmp} = 0;
    while __daikon_tmp${next_tmp} < self.${field_name}.len() {
        __daikon_tmp${first_tmp}.push(${type}::from_str(&self.${field_name}[__daikon_tmp${next_tmp}].to_string()).expect(\"Ok\"));
        __daikon_tmp${next_tmp} += 1;
    }";

pub(crate) static DTRACE_PRINT_PRIM_VEC_FOR_FIELD: &str = "dtrace_print_prim_vec::<${type}>(&__daikon_tmp${tmp_uuid}, format!(\"{}{}\", prefix, \".${field_name}\"));";

pub(crate) static DTRACE_PRINT_STRING_VEC_FOR_FIELD: &str = "dtrace_print_string_vec(&__daikon_tmp${tmp_uuid}, format!(\"{}{}\", prefix, \".${field_name}\"));";

pub(crate) static DTRACE_CALL_PRINT_FIELD: &str =
    "self.dtrace_print_${field_name}(depth, prefix.clone());";

#[allow(dead_code)]
pub(crate) static DTRACE_PRINT_XFIELDS_VEC: &str = "
pub fn dtrace_print_${field_name}_vec(v: &Vec<&${type}>, var_name: String) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(&mut traces, \"{}\", var_name).ok();
    let mut arr = String::from(\"[\");
    let mut i = 0;
    while i+1 < v.len() {
        arr.push_str(&format!(\"0x{:x} \", v[i].${field_name}.as_ptr() as usize));
        i += 1;
    }
    if v.len() > 0 {
        arr.push_str(&format!(\"0x{:x}\", v[i].${field_name}.as_ptr() as usize));
    }
    arr.push_str(\"]\");
    writeln!(&mut traces, \"{}\", arr).ok();
    writeln!(traces, \"0\").ok(); }";

// pub(crate) static LET_RET: &str = "
// fn __skip() {
//     let __daikon_ret: ${type} = ${expr};
// }";

pub(crate) static RET: &str = "
fn __skip() {
    return __daikon_ret;
}";

// you have to delete this?
// make this an array with DTRACE_PRINT_FIELDS_EPILOGUE...
pub(crate) static DTRACE_PRINT_FIELDS_PROLOGUE: &str = "
impl __skip {
    pub fn dtrace_print_fields(&self, depth: i32, prefix: String) {
        if depth == 0 {
            return;
        } ";

pub(crate) static DTRACE_PRINT_FIELDS_EPILOGUE: &str = "
    }
}
struct __skip{}"; // maybe can avoid deleting it, but still bad

pub(crate) static DTRACE_PRINT_FIELDS_VEC_PROLOGUE: &str = "
impl __skip {
    pub fn dtrace_print_fields_vec(v: &Vec<&${spliced_struct}>, depth: i32, prefix: String) {
        if depth == 0 {
            return;
        }";

pub(crate) static DTRACE_PRINT_FIELDS_VEC_EPILOGUE: &str = "} } struct __skip{}";

pub(crate) static DTRACE_PRINT_XFIELDS_VEC_PROLOGUE: &str = "impl __skip {";

pub(crate) static DTRACE_PRINT_XFIELDS_VEC_EPILOGUE: &str = " }";

pub(crate) static DTRACE_PRINT_XFIELDS: &str = "
pub fn dtrace_print_${field_name}_vec(v: &Vec<&${type}>, var_name: String) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
            Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
            Ok(traces) => traces,
        };
        writeln!(&mut traces, \"{}\", var_name).ok();
        let mut arr = String::from(\"[\");
        let mut i = 0;
        while i+1 < v.len() {
    arr.push_str(&format!(\"{} \", v[i].${field_name}));
        i += 1;
        }
        if v.len() > 0 {
    arr.push_str(&format!(\"{}\", v[i].${field_name})); }
        arr.push_str(\"]\");
        writeln!(&mut traces, \"{}\", arr).ok();
        writeln!(traces, \"0\").ok(); }";

pub(crate) static DTRACE_PRINT_XFIELDS_STRING: &str = "
pub fn dtrace_print_${field_name}_vec(v: &Vec<&${type}>, var_name: String) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(&mut traces, \"{}\", var_name).ok();
    let mut arr = String::from(\"[\");
    let mut i = 0;
    while i+1 < v.len() {
        arr.push_str(&format!(\"\\\"{}\\\" \", v[i].${field_name}));
        i += 1;
    }
    if v.len() > 0 {
        arr.push_str(&format!(\"\\\"{}\\\"\", v[i].${field_name}));
    }
    arr.push_str(\"]\");
    writeln!(&mut traces, \"{}\", arr).ok();
    writeln!(traces, \"0\").ok();
}";

pub(crate) static BUILD_POINTER_ARR: &str = "
    dtrace_print_pointer(${var_name} as *const _ as *const () as usize,
                         String::from(\"${var_name}\"));";

// does this work for references?
pub(crate) static BUILD_POINTER_ARR_RET: &str = "
    dtrace_print_pointer(__daikon_ret as *const _ as *const () as usize,
                         String::from(\"return\"));";

// only end fn __skip because we will smash a tmp vec loop on the front.
pub(crate) static DTRACE_PRINT_PRIM_VEC: &str = "
    dtrace_print_prim_vec::<${type}>(&__daikon_tmp${tmp_uuid},
                                     String::from(\"${var_name}\"));
}";

pub(crate) static DTRACE_PRINT_STRING_VEC: &str = "
    dtrace_print_string_vec(&__daikon_tmp${tmp_uuid},
                            String::from(\"${var_name}\"));
}";

pub(crate) static BUILD_A_IMPL_BLOCK: &str = "
impl __skip {}";

pub(crate) static VOID_RETURN: &str = "
fn __skip() {
    return;
}";

pub(crate) static DTRACE_NEWLINE: &str = "
fn __skip() {
    dtrace_newline();
}";

// this NONCE_COUNTER per-file is broken for multi-file non-concurrent programs. It has to be a single counter shared between all the files.
// Difficult in Rust as there is no easy extern escape like in C. Maybe unsafe.
pub(crate) static IMPORTS: &str = "use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};
use std::str::FromStr;
static NONCE_COUNTER: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));";

pub(crate) static DAIKON_LIB: &str =
    "pub fn dtrace_print_pointer_arr<T>(v: &[&T], var_name: String) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(&mut traces, \"{}\", var_name).ok();
    let mut arr = String::from(\"[\");
    let mut i = 0;
    while i+1 < v.len() {
        arr.push_str(&format!(\"0x{:x} \", v[i] as *const _ as usize));
        i += 1;
    }
    if v.len() > 0 {
        arr.push_str(&format!(\"0x{:x}\", v[i] as *const _ as usize));
    }
    arr.push_str(\"]\");
    writeln!(&mut traces, \"{}\", arr).ok();
    writeln!(&mut traces, \"0\").ok();
}
pub(crate) fn daikon_lib() -> String {
    let mut res = String::from(DAIKON_LIB[0]);
    for i in 1..DAIKON_LIB.len() {
        res.push_str(&*OUTPUT_PREFIX.lock().unwrap());
        res.push_str(DAIKON_LIB[i]);
    }
    res
}

pub fn dtrace_print_pointer_vec<T>(v: &Vec<&T>, var_name: String) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(&mut traces, \"{}\", var_name).ok();
    let mut arr = String::from(\"[\");
    let mut i = 0;
    while i+1 < v.len() {
        arr.push_str(&format!(\"0x{:x} \", v[i] as *const _ as usize));
        i += 1;
    }
    if v.len() > 0 {
        arr.push_str(&format!(\"0x{:x}\", v[i] as *const _ as usize));
    }
    arr.push_str(\"]\");
    writeln!(&mut traces, \"{}\", arr).ok();
    writeln!(&mut traces, \"0\").ok();
}

// T must implement Display trait
fn dtrace_print_prim_arr<T: std::fmt::Display>(v: &[T], prefix: String) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(&mut traces, \"{}\", format!(\"{}{}\", prefix, \"[..]\")).ok();
    let mut arr = String::from(\"[\");
    let mut i = 0;
    while i+1 < v.len() {
        arr.push_str(&format!(\"{} \", v[i]));
        i += 1;
    }
    if v.len() > 0 {
        arr.push_str(&format!(\"{}\", v[i]));
    }
    arr.push_str(\"]\");
    writeln!(&mut traces, \"{}\", arr).ok();
    writeln!(&mut traces, \"0\").ok();
}

fn dtrace_print_prim_vec<T: std::fmt::Display>(v: &Vec<T>, prefix: String) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(&mut traces, \"{}\", format!(\"{}{}\", prefix, \"[..]\")).ok();
    let mut arr = String::from(\"[\");
    let mut i = 0;
    while i+1 < v.len() {
        arr.push_str(&format!(\"{} \", v[i]));
        i += 1;
    }
    if v.len() > 0 {
        arr.push_str(&format!(\"{}\", v[i]));
    }
    arr.push_str(\"]\");
    writeln!(&mut traces, \"{}\", arr).ok();
    writeln!(&mut traces, \"0\").ok();
}

fn dtrace_print_str(v: &str, var_name: String) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(&mut traces, \"{}\", var_name).ok();
    writeln!(&mut traces, \"{}\", v).ok();
    writeln!(&mut traces, \"0\").ok();
}

// T must implement Display trait
fn dtrace_print_prim<T: std::fmt::Display>(v: T, var_name: String) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(&mut traces, \"{}\", var_name).ok();
    writeln!(&mut traces, \"{}\", v).ok();
    writeln!(&mut traces, \"0\").ok();
}

fn dtrace_print_string(v: String, var_name: String) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(&mut traces, \"{}\", var_name).ok();
    writeln!(&mut traces, \"\\\"{}\\\"\", v).ok();
    writeln!(&mut traces, \"0\").ok();
}

fn dtrace_print_string_vec(v: &Vec<String>, prefix: String) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(&mut traces, \"{}\", format!(\"{}{}\", prefix, \"[..]\")).ok();
    let mut arr = String::from(\"[\");
    let mut i = 0;
    while i+1 < v.len() {
        arr.push_str(&format!(\"\\\"{}\\\" \", v[i]));
        i += 1;
    }
    if v.len() > 0 {
        arr.push_str(&format!(\"\\\"{}\\\"\", v[i]));
    }
    arr.push_str(\"]\");
    writeln!(&mut traces, \"{}\", arr).ok();
    writeln!(&mut traces, \"0\").ok();
}

fn dtrace_print_pointer(v: usize, var_name: String) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(&mut traces, \"{}\", var_name).ok();
    writeln!(&mut traces, \"0x{:x}\", v).ok();
    writeln!(&mut traces, \"0\").ok();
}

fn dtrace_entry_no_nonce(ppt_name: &str) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(&mut traces, \"{}\", ppt_name).ok();
}

fn dtrace_exit_no_nonce(ppt_name: &str) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(&mut traces, \"{}\", ppt_name).ok();
}

fn dtrace_entry(ppt_name: &str, nonce: u32) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(&mut traces, \"{}\", ppt_name).ok();
    writeln!(&mut traces, \"this_invocation_nonce\").ok();
    writeln!(&mut traces, \"{}\", nonce).ok();
}

fn dtrace_exit(ppt_name: &str, nonce: u32) {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(traces, \"{}\", ppt_name).ok();
    writeln!(traces, \"this_invocation_nonce\").ok();
    writeln!(traces, \"{}\", nonce).ok();
}

fn dtrace_newline() {
    let mut traces = match File::options().append(true).open(\"${output_prefix}.dtrace\") {
        Err(why) => panic!(\"Daikon couldn't open file, {}\", why),
        Ok(traces) => traces,
    };
    writeln!(traces, \"\").ok();
}";
