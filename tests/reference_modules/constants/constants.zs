package reference_modules.constants.constants;

subtype string CustomString;

const string CONSTANT_STRING = "Test";
const CustomString CUSTOM_CONSTANT_STRING = "Test";
const uint32 CONSTANT_U32 = 44423;



struct ConstantTestStruct {
    string strValue;
    CustomString otherStrValue;

    function string testStringConstants()
    {
        // Test mixing different string types, and make
        // sure the generated code handles these correctly.
        return strValue + CONSTANT_STRING + "DummyString" + otherStrValue + CUSTOM_CONSTANT_STRING + strValue;
    }
};