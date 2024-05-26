package reference_modules.valueof_operator.valueof_operator;


enum int32 OptionEnum {
    HAS_A = 0,
    HAS_B = 1,
    HAS_C = 2,
};

enum uint16 OtherEnum {
    HAS_A = 0,
    HAS_B = 1,
    HAS_C = 2,
};

bitmask uint16 Color {
    RED,
    GREEN = 0x04,
    BLUE,
};

struct ValueOfTest
{
    Color  color;
    OptionEnum optionEnum;
    OtherEnum otherEnum;
    
    function uint16 getValueOfColor()
    {
        return valueof(color);
    }

    function int32 getValueOfOptionEnum()
    {
        return valueof(optionEnum);
    }

    function uint16 getValueOfOtherEnum()
    {
        return valueof(otherEnum);
    }

};
