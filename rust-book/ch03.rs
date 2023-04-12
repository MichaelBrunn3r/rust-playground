#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]

use std::num::Wrapping;

fn main() {
    shadowing();
    integer_types();
    integer_overflow();
    chars();
    param_vs_args(1, 2); // 1 & 2 are arguments
    statement_vs_expression();
    assign_to_if_expression(true);
    assign_to_uninitialized_immutable(true);
    assign_to_loop();
    loop_labels();
}

fn shadowing() {
    let change_type = "   ";
    let change_type = change_type.len();

    let assign_twice = 2;
    let assign_twice = assign_twice * 2;
    // assign_twice += 1; // error
}

fn integer_types() {
    let seperator = 1_000_000;
    let hex: u32 = 0xff0000; // Red
    let octal = 0o777;
    let binary = 0b1111_0000;
    let ascii: u8 = b'A'; // = 65
    let suffix: u16 = 2u16;
}

fn integer_overflow() {
    // assert_eq!(255u8 + 1u8, 0); // error

    assert_eq!((Wrapping(255u8) + Wrapping(1u8)).0, 0);

    assert_eq!(255u8.wrapping_add(1u8), 0);

    assert_eq!(255u8.checked_add(1u8), None);
    assert_eq!(0u8.checked_add(1u8), Some(1u8));

    assert_eq!(255u8.overflowing_add(1u8), (0, true));
    assert_eq!(0u8.overflowing_add(1u8), (1, false));

    assert_eq!(255u8.saturating_add(1u8), 255);
    assert_eq!(0u8.saturating_sub(1u8), 0);
}

fn chars() {
    let check = '✅';
}

fn param_vs_args(param1: u32, param2: u32) -> u32 {
    param1 + param2
}

fn statement_vs_expression() -> u32 {
    let x = 1; // statement, executes instruction and does not result in a value
    x + 1 // expression, results in the value 2
}

fn assign_to_if_expression(condition: bool) {
    let x = if condition { 1 } else { 2 };
}

fn assign_to_uninitialized_immutable(condition: bool) {
    let x;
    if condition {
        x = 1
    } else {
        x = 2
    };
}

fn assign_to_loop() {
    let mut counter = 0;
    let done = loop {
        if counter > 10 {
            break true;
        }
        counter += 1;
    };
}

fn loop_labels() {
    let mut a = 0;
    'outer: loop {
        let mut b = 10;
        loop {
            if a == 10 {
                break 'outer;
            }
            if b == 0 {
                break;
            }
            b -= 1;
        }
        a += 1;
    }
}
