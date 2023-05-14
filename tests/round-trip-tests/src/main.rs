pub mod reference_modules {
    pub mod core {
       pub mod types;
    }
   }
   
   use crate::reference_modules::core::types::{
       ValueWrapper,
       Color,   
   };
   

fn main() {
    // This test generates a test structure, serializes it, deserializes it, and ensures
    // that the data is still the same.

    // Instantiate the data
    let value_wrapper = ValueWrapper::ValueWrapper{
        value:1,
        other_value:3,
        enum_value:Color::Color::BLACK,
        description:String::from("te2222st"),
        opt_int_32:Option::from(12),
    };
    
    // TODO serialize

    // TODO deserialize

    // TODO compare
}
