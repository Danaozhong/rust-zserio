pub struct Array<T, AT> {
    pub array_trait: AT,
    pub raw_array: Vec<T>,

    pub is_auto: bool,
    pub is_packed: bool,
    pub fixed_size: u32,
}