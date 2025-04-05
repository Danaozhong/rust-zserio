# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "zserio~=2.16.0",
# ]
# ///
from packed_arrays_test import test_packed_arrays
from offsets_test import test_offsets


def run_all_tests() -> None:
    test_packed_arrays()
    test_offsets()


if __name__ == "__main__":
    run_all_tests()
