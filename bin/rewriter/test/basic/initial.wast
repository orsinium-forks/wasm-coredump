(module
  (memory 10)
  (func $entry (param i32 i32) (result i32)
    (get_local 0)
    (get_local 1)
    (call $addTwo)
  )
  (func $addTwo (param i32 i32) (result i32)
    (get_local 0)
    (get_local 1)
    (i32.add)
    (unreachable)
  )
  (export "addTwo" (func $entry))
  (export "memory" (memory 0))
)