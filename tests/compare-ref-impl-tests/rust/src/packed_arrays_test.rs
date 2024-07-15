use crate::deserialize_artifacts::read_from_python_and_compare;
use reference_module_lib::reference_modules::packed_arrays::packed_arrays::packed_array_wrapper::PackedArrayWrapper;

pub fn packed_arrays_test() {
    let mut test_obj = PackedArrayWrapper::default();
    read_from_python_and_compare("packed_arrays_test", &mut test_obj)
        .expect("can not compare with python");
}
