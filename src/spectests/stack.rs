// Rust test file autogenerated with cargo build (src/build_spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/stack.wast
#![allow(
    warnings,
    dead_code
)]
use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, VmCtx, Export};
use super::_common::{
    spectest_importobject,
    NaNCheck,
};
use wabt::wat2wasm;


// Line 1
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (param i64) (result i64)))
      (func (;0;) (type 0) (param i64) (result i64)
        (local i64 i64)
        get_local 0
        set_local 1
        i64.const 1
        set_local 2
        block  ;; label = @1
          loop  ;; label = @2
            get_local 1
            i64.const 0
            i64.eq
            if  ;; label = @3
              br 2 (;@1;)
            else
              get_local 1
              get_local 2
              i64.mul
              set_local 2
              get_local 1
              i64.const 1
              i64.sub
              set_local 1
            end
            br 0 (;@2;)
          end
        end
        get_local 2)
      (func (;1;) (type 0) (param i64) (result i64)
        (local i64 i64)
        get_local 0
        set_local 1
        i64.const 1
        set_local 2
        block  ;; label = @1
          loop  ;; label = @2
            get_local 1
            i64.const 0
            i64.eq
            if  ;; label = @3
              br 2 (;@1;)
            else
              get_local 1
              get_local 2
              i64.mul
              set_local 2
              get_local 1
              i64.const 1
              i64.sub
              set_local 1
            end
            br 0 (;@2;)
          end
        end
        get_local 2)
      (func (;2;) (type 0) (param i64) (result i64)
        (local i64 i64)
        get_local 0
        set_local 1
        i64.const 1
        set_local 2
        block  ;; label = @1
          loop  ;; label = @2
            get_local 1
            i64.const 0
            i64.eq
            if  ;; label = @3
              br 2 (;@1;)
            else
              get_local 1
              get_local 2
              i64.mul
              set_local 2
              get_local 1
              i64.const 1
              i64.sub
              set_local 1
            end
            br 0 (;@2;)
          end
        end
        get_local 2)
      (func (;3;) (type 0) (param i64) (result i64)
        (local i64 i64)
        get_local 0
        set_local 1
        i64.const 1
        set_local 2
        block  ;; label = @1
          loop  ;; label = @2
            get_local 1
            i64.const 0
            i64.eq
            if  ;; label = @3
              br 2 (;@1;)
            else
              get_local 1
              get_local 2
              i64.mul
              set_local 2
              get_local 1
              i64.const 1
              i64.sub
              set_local 1
            end
            br 0 (;@2;)
          end
        end
        get_local 2)
      (func (;4;) (type 0) (param i64) (result i64)
        (local i64 i64)
        get_local 0
        set_local 1
        i64.const 1
        set_local 2
        block  ;; label = @1
          loop  ;; label = @2
            get_local 1
            i64.const 0
            i64.eq
            if  ;; label = @3
              br 2 (;@1;)
            else
              get_local 1
              get_local 2
              i64.mul
              set_local 2
              get_local 1
              i64.const 1
              i64.sub
              set_local 1
            end
            br 0 (;@2;)
          end
        end
        get_local 2)
      (export \"fac-expr\" (func 0))
      (export \"fac-stack\" (func 1))
      (export \"fac-stack-raw\" (func 2))
      (export \"fac-mixed\" (func 3))
      (export \"fac-mixed-raw\" (func 4)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_1(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 130
fn c1_l130_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c1_l130_action_invoke");
    let func_index = match result_object.module.info.exports.get("fac-expr") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(25 as i64, &vm_context);
    assert_eq!(result, 7034535277573963776 as i64);
}

// Line 131
fn c2_l131_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c2_l131_action_invoke");
    let func_index = match result_object.module.info.exports.get("fac-stack") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(25 as i64, &vm_context);
    assert_eq!(result, 7034535277573963776 as i64);
}

// Line 132
fn c3_l132_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c3_l132_action_invoke");
    let func_index = match result_object.module.info.exports.get("fac-mixed") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(25 as i64, &vm_context);
    assert_eq!(result, 7034535277573963776 as i64);
}

// Line 137

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_1(&result_object, &vm_context);
    c1_l130_action_invoke(&result_object, &vm_context);
    c2_l131_action_invoke(&result_object, &vm_context);
    c3_l132_action_invoke(&result_object, &vm_context);
}
fn create_module_2() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (type (;1;) (func (param i32)))
      (type (;2;) (func (result i32)))
      (func (;0;) (type 0)
        block  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        loop  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        else
          i32.const 0
          call_indirect (type 0)
        end
        block  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        loop  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        else
          i32.const 0
          call_indirect (type 0)
        end
        block  ;; label = @1
          i32.const 0
          i32.const 0
          call_indirect (type 1)
        end
        loop  ;; label = @1
          i32.const 0
          i32.const 0
          call_indirect (type 1)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          i32.const 0
          call_indirect (type 1)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          i32.const 0
          call_indirect (type 1)
        else
          i32.const 0
          i32.const 0
          call_indirect (type 1)
        end
        block (result i32)  ;; label = @1
          i32.const 0
          call_indirect (type 2)
        end
        drop
        loop (result i32)  ;; label = @1
          i32.const 0
          call_indirect (type 2)
        end
        drop
        i32.const 0
        if (result i32)  ;; label = @1
          i32.const 0
          call_indirect (type 2)
        else
          i32.const 0
          call_indirect (type 2)
        end
        drop
        block  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        loop  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        else
          i32.const 0
          call_indirect (type 0)
        end
        block  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        loop  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        else
          i32.const 0
          call_indirect (type 0)
        end
        block  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        loop  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        else
          i32.const 0
          call_indirect (type 0)
        end
        block  ;; label = @1
          i32.const 0
          i32.const 0
          call_indirect (type 1)
        end
        loop  ;; label = @1
          i32.const 0
          i32.const 0
          call_indirect (type 1)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          i32.const 0
          call_indirect (type 1)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          i32.const 0
          call_indirect (type 1)
        else
          i32.const 0
          i32.const 0
          call_indirect (type 1)
        end
        block (result i32)  ;; label = @1
          i32.const 0
          call_indirect (type 2)
        end
        drop
        loop (result i32)  ;; label = @1
          i32.const 0
          call_indirect (type 2)
        end
        drop
        i32.const 0
        if (result i32)  ;; label = @1
          i32.const 0
          call_indirect (type 2)
        else
          i32.const 0
          call_indirect (type 2)
        end
        drop
        block  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        loop  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        end
        i32.const 0
        if  ;; label = @1
          i32.const 0
          call_indirect (type 0)
        else
          i32.const 0
          call_indirect (type 0)
        end
        i32.const 0
        call_indirect (type 0))
      (table (;0;) 1 anyfunc))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_2(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

#[test]
fn test_module_2() {
    let result_object = create_module_2();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_2(&result_object, &vm_context);
}
