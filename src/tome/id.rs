use std::path::PathBuf;

pub type PlatformId = PathBuf;

#[derive(Debug)]
pub struct Id<'a> {
    elements: Vec<&'a str>,
}

impl<'a> From<&'a str> for Id<'a> {
    fn from(s: &'a str) -> Id<'a> {
        let elements = s.split('/').filter(|x| x.len() > 0).collect();

        Id { elements }
    }
}

impl<'a> From<Id<'a>> for PathBuf {
    fn from(id: Id<'a>) -> PathBuf {
        let mut me = PathBuf::new();

        for x in id {
            me.push(x);
        }

        me
    }
}
