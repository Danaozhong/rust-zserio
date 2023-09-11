package reference_modules.subtyped_dot_expression.test;

import reference_modules.subtyped_dot_expression.subtyped_enum.*;

struct TestStruct {
    SubtypedEnum value1;

    int32 optValue1 if value1 == SubtypedEnum.TEST_VALUE_A;
    int32 optValue2 if value1 == SubtypedEnum.TEST_VALUE_B;
    
};