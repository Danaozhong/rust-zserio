package reference_modules.alignment.alignment;

struct AlignmentStruct
{
    bool boValue1;

    align(4):
    bool boValue2;

    align(8):
    bool boValue3;

    
    align(8): // This alignment should only be set if boValue4 is set.
    optional bool boValue4;

    align(4): // This alignment should only be set if boValue4 is set.
    bool boValue5 if boValue3;


};
