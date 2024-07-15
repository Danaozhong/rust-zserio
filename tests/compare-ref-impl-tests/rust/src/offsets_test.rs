use crate::deserialize_artifacts::read_from_python_and_compare;
use reference_module_lib::reference_modules::offsets::offsets::offsets::Offsets;

pub fn offsets_test() {
    let mut test_obj = Offsets::default();
    read_from_python_and_compare("offsets_test", &mut test_obj)
        .expect("can not compare with python");
}
