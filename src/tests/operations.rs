use std::num::NonZeroU32;

use crate::engine::error_types::OperationError;
use crate::engine::parser::VecValue;
use crate::engine::parser_ledger::parse_ledger;
use crate::engine::operations::{ArithmeticOp, FieldRef, Operation};
use crate::engine::types::{Table, Value, ValueType, Policy};
use crate::tests::table::{default_init_table, default_add_values, DEF_NAMES};

#[test]
fn arith_amount_sub() {
    let table_target = &mut default_init_table(None);
    default_add_values(table_target, 2u64, None);

    for n in &DEF_NAMES.to_vec()[1..3] {
        let operation = Operation::Arith { target: FieldRef { hash: 1 , name: n }, op: ArithmeticOp::Sub, amount: table_target.get_value(&2u64, n).unwrap() };

        assert_eq!(table_target.apply(operation), Ok(()));
    }
}

#[test]
fn arith_amount_add() {
    let table_target = &mut default_init_table(None);
    default_add_values(table_target, 2u64, None);

    for n in &DEF_NAMES.to_vec()[1..3] {
        let operation = Operation::Arith { target: FieldRef { hash: 1 , name: n }, op: ArithmeticOp::Add, amount: table_target.get_value(&2u64, n).unwrap() };

        assert_eq!(table_target.apply(operation), Ok(()));
    }
}

#[test]
fn arith_amount_div() {
    let table_target = &mut default_init_table(None);
    default_add_values(table_target, 2u64, None);

    let operation = Operation::Arith { target: FieldRef { hash: 1 , name: "_float" }, op: ArithmeticOp::Div, amount: table_target.get_value(&2u64, "_float").unwrap() };

    assert_eq!(table_target.apply(operation), Ok(()));
}

#[test]
fn arith_amount_mul() {
    let table_target = &mut default_init_table(None);
    default_add_values(table_target, 2u64, None);

    for n in &DEF_NAMES.to_vec()[1..3] {
        let operation = Operation::Arith { target: FieldRef { hash: 1 , name: n }, op: ArithmeticOp::Mul, amount: table_target.get_value(&2u64, n).unwrap() };

        assert_eq!(table_target.apply(operation), Ok(()));
    }
}
