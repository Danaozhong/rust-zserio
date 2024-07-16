use reference_module_lib::reference_modules::expr_numbits::expr_numbits::expression_numbits_test::ExpressionNumbitsTest;

/// Tests a zserio struct, which uses a function with the numbits() operator.
/// The function get_num_bits() is defined as `return numbits(u16Value) + numbits(8);`.
pub fn test_expr_numbits() {
    let mut test_struct = ExpressionNumbitsTest::default();
    assert_eq!(test_struct.get_num_bits(), 3);

    test_struct.u_16_value = 1;
    assert_eq!(test_struct.get_num_bits(), 4);

    test_struct.u_16_value = 7;
    assert_eq!(test_struct.get_num_bits(), 6);

    // as per
    // https://zserio.org/doc/ZserioLanguageOverview.html
    // numbits(8) + numbits(8) =
    // 3 + 3
    test_struct.u_16_value = 8;
    assert_eq!(test_struct.get_num_bits(), 6);

    test_struct.u_16_value = 9;
    assert_eq!(test_struct.get_num_bits(), 7);
}
