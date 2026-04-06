
use super::mjtile::{MJTile};
use std::fmt::{Display, Write};
use bitvec::prelude::*;
use arrayvec::ArrayVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum MJCombCategory {
    Shuntu = 0,
    Koutu,
    Kantu,
    Tuitu
}

#[derive(Debug, Clone, Copy)]
pub struct MJCombRedFlags(BitArr!(for 4, in u8, Lsb0));

impl MJCombRedFlags {
    fn new(flags: u8) -> Self {
        MJCombRedFlags(BitArray::<[u8;1]>::from([flags]))
    }

    pub const NONE: Self = Self(BitArray::ZERO);
}

impl From<(bool,bool,bool,bool)> for MJCombRedFlags {
    fn from(value: (bool,bool,bool,bool)) -> Self {
        let (rb1,rb2,rb3,rb4) = value;
        MJCombRedFlags::new(((rb4 as u8) << 3) | ((rb3 as u8) << 2) | ((rb2 as u8) << 1) | (rb1 as u8))
    }
}

impl From<(u8,u8,u8,u8)> for MJCombRedFlags {
    fn from(value: (u8,u8,u8,u8)) -> Self {
        let (r1, r2, r3, r4) = value;
        From::<(bool,bool,bool,bool)>::from((r1>0, r2>0, r3>0, r4>0))
    }
}

#[derive(Clone)]
pub struct MJComb {
    category_: MJCombCategory,
    first_tile_: MJTile,
    red_flags_: MJCombRedFlags
}

impl MJComb {
    pub fn new(category: MJCombCategory, first_tile: MJTile, red_flags: MJCombRedFlags) -> Self {
        MJComb { category_: category, first_tile_: first_tile, red_flags_: red_flags }
    }

    pub fn category(&self) -> MJCombCategory { self.category_ }
    pub fn first_tile(&self) -> MJTile { self.first_tile_.clone() }
    pub fn red_flags(&self) -> MJCombRedFlags { self.red_flags_ }
    pub fn red_count(&self) -> u8 { self.red_flags_.0.count_ones() as u8 }
    pub fn is_tile_red(&self, pos: u8) -> bool { self.red_flags_.0[pos as usize] }
    pub fn is_general_koutu(&self) -> bool { self.category_ == MJCombCategory::Koutu || self.category_ == MJCombCategory::Kantu }
    pub fn is_chanta(&self) -> bool {
        self.first_tile_.is_yaochuuhai() || (self.category_ == MJCombCategory::Shuntu && self.first_tile_.number() == 7)
    }
}

impl PartialEq for MJComb {
    fn eq(&self, other: &Self) -> bool {
        self.category_ == other.category_ && self.first_tile_ == other.first_tile_
    }
}

impl Eq for MJComb {}

impl PartialOrd for MJComb {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for MJComb {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.first_tile_
            .cmp(&other.first_tile_)
            .then(self.category_.cmp(&other.category_))
    }
}

impl Display for MJComb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = match self.category() {
            MJCombCategory::Shuntu => 'S',
            MJCombCategory::Koutu => 'K',
            MJCombCategory::Kantu => 'G',
            MJCombCategory::Tuitu => 'T'
        };
        self.first_tile_.fmt(f)?;
        f.write_char(c)
    }
}


pub type MJCombSeq = ArrayVec<MJComb, 5>;