use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(PartialEq)]
pub struct JabberWocky {
    face: String,
    body: char,
}

#[test]
fn test_equal() {
    assert!(JabberWocky::from(('🐸', '🐋')) == JabberWocky::from(('🐸', '🐋')));
    assert!(JabberWocky::from(('🐸', '🐋')) != JabberWocky::from(('🧟', '🫀')));
}

impl Display for JabberWocky {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}-{}=<", self.face, self.body)
    }
}

impl Add for JabberWocky {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let heads = self.face + &rhs.face;
        Self {
            face: heads,
            body: self.body,
        }
    }
}

impl From<(char, char)> for JabberWocky {
    fn from(tuple: (char, char)) -> Self {
        Self {
            face: tuple.0.to_string(),
            body: tuple.1,
        }
    }
}

impl Iterator for JabberWocky {
    type Item = u32;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        Some(42)
    }
}

#[test]
fn test_jabberwocky_from_tuple() {
    let hollis: JabberWocky = ('👽', '🦗').into();
    assert_eq!(format!("{}", hollis), "👽-🦗=<");
}

#[test]
fn test_jabberwocky_add() {
    let harry = JabberWocky {
        face: '👹'.to_string(),
        body: '🦎',
    };
    let frank = JabberWocky {
        face: '🦊'.to_string(),
        body: '🐋',
    };

    let double_jb = harry + frank;

    assert_eq!(format!("{}", double_jb), "👹🦊-🦎=<");
}

#[test]
fn test_jabberwocky_display() {
    let jb = JabberWocky {
        face: '👹'.to_string(),
        body: '🦎',
    };
    assert_eq!(format!("{}", jb), "👹-🦎=<");
}

#[test]
fn test_jabberwocky_iter() {
    let jb = JabberWocky {
        face: '👹'.to_string(),
        body: '🦎',
    };
    assert_eq!(jb.into_iter().take(5).next(), Some(42));
}
