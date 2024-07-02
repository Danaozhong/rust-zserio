from generate_artifacts import generate_artifacts
from pygen.reference_modules.offsets.offsets.offsets import (
    Offsets,
)


def test_offsets() -> None:
    testdata = Offsets(
        u32_offset_=9,
        vi32_array_=[22, 56, 635],
        vi16_offset_array_=[11, 23, 42, 744],
        u32_array_offset_=[33, 35, 37, 39],
        vi64_index_offset_array_=[100, 200, 300, 400],
        u8_check_=241,
        u16_final_check_=14,
        has_flag_=False,
        u16_offset_=233,
        u32_value_=14,
        u16_yet_final_check_=42,
    )
    generate_artifacts(__name__, testdata)
