from generate_artifacts import generate_artifacts
from pygen.reference_modules.packed_arrays.packed_arrays.bubble_tea_addons import (
    BubbleTeaAddons,
)
from pygen.reference_modules.packed_arrays.packed_arrays.bubble_tea_size import (
    BubbleTeaSize,
)
from pygen.reference_modules.packed_arrays.packed_arrays.data_struct import DataStruct
from pygen.reference_modules.packed_arrays.packed_arrays.packed_array_wrapper import (
    PackedArrayWrapper,
)


def test_packed_arrays() -> None:
    testdata = PackedArrayWrapper(
        packed_array_=[
            DataStruct(
                u32_packed_array_=[1, 2, 3, 4],
                u32_array_=[10, 11, 5523],
                size_=BubbleTeaSize.COLD_LARGE,
                tea_addons_=BubbleTeaAddons.Values.HAS_BLACK_SUGAR
                | BubbleTeaAddons.Values.HAS_CREAM_CHEESE,
                vi32_value1_=-353,
                bo_value2_=True,
                i16_value3_=433,
                str_value4_="Black Sugar Milk Tea",
            ),
            DataStruct(
                u32_packed_array_=[11, 12, 13],
                u32_array_=[5523, 5525, 5521],
                size_=BubbleTeaSize.HOT_MEDIUM,
                tea_addons_=BubbleTeaAddons.Values.HAS_CREAM_CHEESE,
                vi32_value1_=-4,
                bo_value2_=True,
                i16_value3_=1,
                str_value4_="Placeholder",
            ),
        ],
        standard_array_=[
            DataStruct(
                u32_packed_array_=[],
                u32_array_=[7765],
                size_=BubbleTeaSize.HOT_MEDIUM,
                tea_addons_=BubbleTeaAddons.Values.HAS_PEARLS,
            ),
        ],
    )

    generate_artifacts(__name__, testdata)
