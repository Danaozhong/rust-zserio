package reference_modules.constants.constants;

const string CONSTANT_STRING = "Test";
const uint32 CONSTANT_U32 = 44423;

subtype string CustomString;

struct ConstantTestStruct {
    string strValue;
    CustomString otherStrValue;

    function string testStringConstants()
    {
        // Test mixing different string types, and make
        // sure the generated code handles these correctly.
        return strValue + CONSTANT_STRING + "DummyString" + otherStrValue + CONSTANT_STRING + strValue;
    }
};