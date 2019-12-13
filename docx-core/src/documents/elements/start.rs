use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Start {
    val: usize,
}

impl Start {
    pub fn new(val: usize) -> Start {
        Start { val }
    }
}

impl BuildXML for Start {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.start(self.val).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_start() {
        let c = Start::new(1);
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:start w:val="1" />"#);
    }
}