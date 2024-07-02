pub mod deserialize_artifacts;
pub mod offsets_test;
pub mod packed_arrays_test;
use crate::offsets_test::offsets_test;
use crate::packed_arrays_test::packed_arrays_test;

fn main() {
    offsets_test();
    packed_arrays_test();
}
