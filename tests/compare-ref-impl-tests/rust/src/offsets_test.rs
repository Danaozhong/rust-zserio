use reference_module_lib::reference_modules::offsets::offsets::offsets::Offsets;

use crate::deserialize_artifacts::read_from_python_and_compare;
use rust_zserio::ztype::ZserioPackableObject;

pub fn offsets_test() {
    let mut test_obj = Offsets::new();
    read_from_python_and_compare("offsets_test", &mut test_obj)
        .expect("can not compare with python");
}
