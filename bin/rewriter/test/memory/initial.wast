(module
  (memory 1 1)
  (func $valid (result i32)
    (i32.store (i32.const 0) (i32.const 123))
    (i32.load (i32.const 0))
  )
  (func $invalid (result i32)
    (i32.load (i32.const 16392))
  )
  (func $invalid_with_locals (result i32) (local i32 i64)
    (i32.load (i32.const 16392))
  )
  (func $invalid_nested (result i32)
    (i32.load (i32.const 16392))
  )
  (func $invalid_nested1 (result i32)
    (call $invalid_nested)
  )
  (func $invalid_nested2
    (call $invalid_nested1)
    (drop)
  )
  (export "valid" (func $valid))
  (export "invalid" (func $invalid))
  (export "invalid_with_locals" (func $invalid_with_locals))
  (export "invalid_nested2" (func $invalid_nested2))
  (export "_start" (func $invalid_nested2))
  (export "memory" (memory 0))
)