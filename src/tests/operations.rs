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

    for un in &DEF_NAMES.to_vec()[3..7] {
        let operation = Operation::Arith { target: FieldRef { hash: 1 , name: un }, op: ArithmeticOp::Sub, amount: table_target.get_value(&2u64, un).unwrap() };

        assert_eq!(table_target.apply(operation), Err(OperationError::UnresolvedValues { value_type: table_target.get_value(&1u64, un).unwrap().value_type(), operation: ArithmeticOp::Sub }));
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

    for un in &DEF_NAMES.to_vec()[3..7] {
        let operation = Operation::Arith { target: FieldRef { hash: 1 , name: un }, op: ArithmeticOp::Add, amount: table_target.get_value(&2u64, un).unwrap() };

        assert_eq!(table_target.apply(operation), Err(OperationError::UnresolvedValues { value_type: table_target.get_value(&1u64, un).unwrap().value_type(), operation: ArithmeticOp::Add }));
    }
}

#[test]
fn arith_amount_div() {
    let table_target = &mut default_init_table(None);
    default_add_values(table_target, 2u64, None);

    table_target.set_value(&2u64, "_float", Value::Float(1.0)).unwrap();

    let operation = Operation::Arith { target: FieldRef { hash: 1 , name: "_float" }, op: ArithmeticOp::Div, amount: table_target.get_value(&2u64, "_float").unwrap() };

    assert_eq!(table_target.apply(operation), Ok(()));

    for (i, un) in DEF_NAMES.iter().enumerate().filter(|&(index, _)| index != 2) {
        let operation = Operation::Arith { target: FieldRef { hash: 1 , name: un }, op: ArithmeticOp::Div, amount: table_target.get_value(&2u64, un).unwrap() };

        assert_eq!(table_target.apply(operation), Err(OperationError::UnresolvedValues { value_type: table_target.get_value(&1u64, un).unwrap().value_type(), operation: ArithmeticOp::Div }));
    }

    let operation_zero = Operation::Arith { target: FieldRef { hash: 2 , name: "_float" }, op: ArithmeticOp::Div, amount: table_target.get_value(&1u64, "_float").unwrap() };

    assert_eq!(table_target.apply(operation_zero), Err(OperationError::DivideByZero));
}

#[test]
fn arith_amount_mul() {
    let table_target = &mut default_init_table(None);
    default_add_values(table_target, 2u64, None);

    for n in &DEF_NAMES.to_vec()[0..3] {
        let operation = Operation::Arith { target: FieldRef { hash: 1 , name: n }, op: ArithmeticOp::Mul, amount: table_target.get_value(&2u64, n).unwrap() };

        assert_eq!(table_target.apply(operation), Ok(()));
    }

    for un in &DEF_NAMES.to_vec()[3..7] {
        let operation = Operation::Arith { target: FieldRef { hash: 1 , name: un }, op: ArithmeticOp::Mul, amount: table_target.get_value(&2u64, un).unwrap() };

        assert_eq!(table_target.apply(operation), Err(OperationError::UnresolvedValues { value_type: table_target.get_value(&1u64, un).unwrap().value_type(), operation: ArithmeticOp::Mul }));
    }
}

#[test]
fn arith_amount_with_ledger() {
    let table_target = &mut default_init_table(Some(parse_ledger("1").unwrap()));
    default_add_values(table_target, 2u64, None);

    for n in &DEF_NAMES.to_vec()[1..3] {
        let operation = Operation::Arith { target: FieldRef { hash: 1 , name: n }, op: ArithmeticOp::Mul, amount: table_target.get_value(&2u64, n).unwrap() };

        assert_eq!(table_target.apply(operation), Err(OperationError::SetValueWithLedger { hash: 1 }));
    }
}

#[test]
fn arith_amount_type_mismatch() {
    let table_target = &mut default_init_table(None);
    default_add_values(table_target, 2u64, None);

    let operation = Operation::Arith { target: FieldRef { hash: 1 , name: "_int" }, op: ArithmeticOp::Mul, amount: table_target.get_value(&2u64, "_uint").unwrap() };

    assert_eq!(table_target.apply(operation), Err(OperationError::TargetAmountTypeMismatch { expected: ValueType::Int, received: ValueType::UInt }));
}
