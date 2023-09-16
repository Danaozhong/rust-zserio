pub mod packed_arrays_test;
pub mod deserialize_artifacts;
use crate::packed_arrays_test::reference_implementation_test;

fn main() {
    reference_implementation_test();
}
