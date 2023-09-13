#![allow(clippy::all)]
#![allow(warnings)]
pub mod reference_modules {
    pub mod optional_values {
        pub mod optional_values;
    }
    pub mod core {
        pub mod instantiations;
        pub mod templates;
        pub mod types;
    }
    pub mod constants {
        pub mod constants;
    }
    pub mod alignment {
        pub mod alignment;
    }
    pub mod parameter_passing {
        pub mod index_operator;
        pub mod parameter_passing;
    }
    pub mod template_instantiation {
        pub mod template_instantiation;
    }
    pub mod user {
        pub mod testobject;
    }
    pub mod ambiguous_types {
        pub mod main;
        pub mod other;
    }
    pub mod parameterized_array_length {
        pub mod parameterized_array_length;
    }
    pub mod type_lookup_test {
        pub mod other_ztype;
        pub mod unrelated_ztype;
        pub mod ztype;
    }
    pub mod integer_types {
        pub mod integer_types;
    }
    pub mod packed_arrays {
        pub mod packed_arrays;
    }
    pub mod all;
    pub mod type_casts {
        pub mod type_casts;
    }
    pub mod bitmask_test {
        pub mod bitmask_test;
    }
}