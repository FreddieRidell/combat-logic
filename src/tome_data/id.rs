#[derive(Eq, PartialEq, Hash)]
pub struct Id(String);

impl Id {
    pub fn get_string(&self) -> &str {
        &self.0
    }
}
