package reference_modules.expr_numbits.expr_numbits;

struct ExpressionNumbitsTest
{
    uint32 u16Value;

    function uint32 getNumBits() {
        return numbits(u16Value) + numbits(8);
    }
};
