use std::{fmt::Display, str::FromStr};
use strum::FromRepr;

#[derive(Debug, FromRepr, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MJTileCategory {
    Man = 0,
    So,
    Pin,
    Ton,
    Nan,
    Shya,
    Pei,
    Tyu,
    Haku,
    Hatu
}

#[derive(Clone, Copy)]
pub struct MJTile {
    face_: u8,
    is_red_: bool
}

impl MJTile {
    pub fn new(cate: MJTileCategory, number: u8, isred: bool) -> Self {
        let face_: u8 = match cate {
            MJTileCategory::Man | MJTileCategory::So | MJTileCategory::Pin
                => (cate as u8) * 10 + number,
            _ => (cate as u8) * 10
        };
        MJTile { face_, is_red_: isred }
    }

    pub fn set_red(&mut self) { self.is_red_ = true; }
    pub fn clear_red(&mut self) { self.is_red_ = false; }

    pub fn category(self) -> MJTileCategory {
        unsafe { MJTileCategory::from_repr(self.face_ / 10).unwrap_unchecked() } 
    }
    pub fn number(self) -> u8 { self.face_ % 10 }
    pub fn is_red(self) -> bool { self.is_red_ }
    pub fn is_shiuhai(self) -> bool {
        match self.category() {
            MJTileCategory::Man | MJTileCategory::So | MJTileCategory::Pin => true,
            _ => false
        }
    }
    pub fn is_jihai(&self) -> bool { !self.is_shiuhai() }
    pub fn is_yaochuuhai(&self) -> bool { self.is_jihai() || (self.number() == 1 || self.number() == 9) }
    pub fn is_number_prev_to(self, next: MJTile) -> bool {
        self.category() == next.category() && self.is_shiuhai() && self.number()+1 == next.number()
    }

}

impl Display for MJTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(String::from(self).as_str())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MJTileParseError;

impl From<&MJTile> for String {
    fn from(value: &MJTile) -> Self {
        let mut b: Vec<u8> = vec![b'0',0,0,0];
        let c: u8 = match value.category() {
            MJTileCategory::Man => b'M',
            MJTileCategory::So => b'S',
            MJTileCategory::Pin => b'P',
            MJTileCategory::Ton => b'T',
            MJTileCategory::Nan => b'N',
            MJTileCategory::Shya => b'S',
            MJTileCategory::Pei => b'P',
            MJTileCategory::Tyu => b'Z',
            MJTileCategory::Haku => b'B',
            MJTileCategory::Hatu => b'F'
        };
        if value.is_jihai() {
            b[0] = c;
        } else {
            b[0] = b'0' + value.number();
            b[1] = c;
            b[2] = if value.is_red_ { b'r' } else { 0 }
        };
        unsafe { String::from_utf8_unchecked(b) }
    }
}

impl FromStr for MJTile {
    type Err = MJTileParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() >= 1 && s.len() <= 3 {
            let bytes = s.as_bytes();
            if s.len() == 1 {
                return match bytes[0] {
                    b'T' => Ok(MJTile::new(MJTileCategory::Ton, 0, false)),
                    b'N' => Ok(MJTile::new(MJTileCategory::Nan, 0, false)),
                    b'S' => Ok(MJTile::new(MJTileCategory::Shya, 0, false)),
                    b'P' => Ok(MJTile::new(MJTileCategory::Pei, 0, false)),
                    b'Z' => Ok(MJTile::new(MJTileCategory::Tyu, 0, false)),
                    b'B' => Ok(MJTile::new(MJTileCategory::Haku, 0, false)),
                    b'F' => Ok(MJTile::new(MJTileCategory::Hatu, 0, false)),
                    _ => Err(MJTileParseError)
                };
            } else {
                let isred = bytes.len() == 3 && bytes[2] == b'r';
                return match bytes[1] {
                    b'M' => Ok(MJTile::new(MJTileCategory::Man, bytes[0]-b'0', isred)),
                    b'S' => Ok(MJTile::new(MJTileCategory::So, bytes[0]-b'0', isred)),
                    b'P' => Ok(MJTile::new(MJTileCategory::Pin, bytes[0]-b'0', isred)),
                    _ => Err(MJTileParseError)
                };
            }
        }
        Err(MJTileParseError)
    }
}

impl TryFrom<&str> for MJTile {
    type Error = MJTileParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        MJTile::from_str(value)
    }
}

impl From<MJTile> for u8 {
    fn from(value: MJTile) -> Self {
        value.face_
    }
}

impl PartialEq for MJTile {
    fn eq(&self, other: &Self) -> bool {
        self.face_ == other.face_
    }
}

impl Eq for MJTile {}

impl PartialOrd for MJTile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MJTile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.face_.cmp(&other.face_)
    }
}