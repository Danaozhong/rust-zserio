use reference_module_lib::reference_modules::expr_numbits::expr_numbits::expression_numbits_test::ExpressionNumbitsTest;
use rust_zserio::ztype::ZserioPackableObject;

/// Tests a zserio struct, which uses a function with the numbits() operator.
/// The function get_num_bits() is defined as `return numbits(u16Value) + numbits(8);`.
pub fn test_expr_numbits() {
    let mut test_struct = ExpressionNumbitsTest::new();
    assert_eq!(test_struct.get_num_bits(), 4);

    test_struct.u_16_value = 1;
    assert_eq!(test_struct.get_num_bits(), 5);

    test_struct.u_16_value = 7;
    assert_eq!(test_struct.get_num_bits(), 7);

    test_struct.u_16_value = 8;
    assert_eq!(test_struct.get_num_bits(), 8);
}
