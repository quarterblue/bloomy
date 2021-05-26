pub struct Table<T> {
    fields: Vec<String>,
    rows: Vec<Vec<T>>,
}

impl Table<T> {
    pub fn new() -> Self {
        Table {
            fields: Vec::new(),
            rows: Vec::new(Vec::new()),
        }
    }

    pub fn add_fields() -> Result<(), std::io::Error> {}
    pub fn add_row() -> Result<(), std::io::Error> {}
    pub fn print_table() -> Result<(), std::io::Error> {}
    pub fn print_title() -> Result<(), std::io::Error> {}
}
