pub struct Field {
    pub name: String,
    pub type_: String,
    pub length: usize,

}

impl Field {
    pub fn new(name: String, type_: String, length: usize) -> Field {
        Field {
            name,
            type_,
            length
        }
    }

    // TODO: method for modifying field properties

    // TODO: method for validating field values (should prob just be in new?)
}
