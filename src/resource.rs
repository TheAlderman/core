use std::fmt;

pub struct Resource {
    pub id: String,
    pub price: i64
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resource {}, base price: {}", self.id, self.price)
    }
}
