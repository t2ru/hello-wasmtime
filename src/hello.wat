(module
    (func $print_number (import "" "print_number") (param i32))

    (func (export "add") (param $lhs i32) (param $rhs i32) (result i32)
        local.get $lhs
        local.get $rhs
        i32.add
        return
    )

    (func (export "add_print") (param $lhs i32) (param $rhs i32)
        local.get $lhs
        local.get $rhs
        i32.add
        call $print_number
    )
)