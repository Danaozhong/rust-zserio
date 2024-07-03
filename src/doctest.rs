//! This module contains dumy code that is for doctests. It should
//! never be used.
//!
//! This is a workaround until https://github.com/rust-lang/rust/issues/67295
//! is fixed.
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use crate::{Result, ZserioPackableObject};
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

#[derive(Debug)]
pub struct DrinkOrder {
    #[allow(dead_code)]
    pub customer_name: String,
}

impl ZserioPackableObject for DrinkOrder {
    fn new() -> Self {
        Self {
            customer_name: String::new(),
        }
    }

    fn zserio_read(&mut self, _: &mut BitReader) -> Result<()> {
        Ok(())
    }

    fn zserio_write(&self, _: &mut BitWriter) -> Result<()> {
        Ok(())
    }

    fn zserio_read_packed(&mut self, _: &mut PackingContextNode, _: &mut BitReader) -> Result<()> {
        Ok(())
    }

    fn zserio_write_packed(&self, _: &mut PackingContextNode, _: &mut BitWriter) -> Result<()> {
        Ok(())
    }

    fn zserio_bitsize(&self, _: u64) -> Result<u64> {
        Ok(0)
    }

    fn zserio_bitsize_packed(&self, _: &mut PackingContextNode, _: u64) -> Result<u64> {
        Ok(0)
    }

    fn zserio_create_packing_context(_: &mut PackingContextNode) {}

    fn zserio_init_packing_context(&self, _: &mut PackingContextNode) {}
}
