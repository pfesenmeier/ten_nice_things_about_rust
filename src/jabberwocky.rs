use std::fmt::{Display, Formatter};
 use std::ops::Add;

pub struct JabberWocky {
  face: String,
  body: char,
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
        Self { face: heads, body: self.body }
    }
}

impl From<(char, char)> for JabberWocky {
    fn from(tuple: (char, char)) -> Self { 
        Self { face: tuple.0.to_string(), body: tuple.1 }
    }
}

#[test]
fn test_jabberwocky_from_tuple() {
    let hollis: JabberWocky = ('👽','🦗').into();
    assert_eq!(format!("{}", hollis), "👽-🦗=<");
}

#[test]
fn test_jabberwocky_add() {
  let harry = JabberWocky { face: '👹'.to_string(), body: '🦎'};
  let frank = JabberWocky { face: '🦊'.to_string(), body: '🐋'};

  let double_jb = harry + frank;

  assert_eq!(format!("{}", double_jb), "👹🦊-🦎=<");
}

#[test]
fn test_jabberwocky_display() {
    let jb = JabberWocky { face: '👹'.to_string(), body: '🦎'};
    assert_eq!(format!("{}", jb), "👹-🦎=<");
}
