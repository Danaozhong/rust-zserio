package reference_modules.bitmask_isset.bitmask_isset;

bitmask uint8 SomeBitMask
{
    FLAG_A,
    FLAG_B = 0x04,
    FLAG_C,
    FLAG_D,
};

struct BitmaskTest {
    SomeBitMask value;

    function bool hasA()
    {
        return isset(value, SomeBitMask.FLAG_A);
    }

    function bool hasB()
    {
        return isset(value, SomeBitMask.FLAG_B);
    }
    
    function bool hasC()
    {
        return isset(value, SomeBitMask.FLAG_C);
    }

    function bool hasAOrC()
    {
        return isset(value, SomeBitMask.FLAG_A | SomeBitMask.FLAG_C);
    }
};