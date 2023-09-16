pub mod deserialize_artifacts;
pub mod packed_arrays_test;
use crate::packed_arrays_test::reference_implementation_test;

fn main() {
    reference_implementation_test();
}
