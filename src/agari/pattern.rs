use super::mjcore;
use mjcore::mjcomb::MJCombCategory;

use std::ops::Index;
use std::num::NonZeroU64;


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AgariPattern(u16);

impl AgariPattern {
    pub const MAX_COMB_COUNT: usize = 5;

    fn encode_comb(cate: MJCombCategory) -> u8 {
        match cate {
            MJCombCategory::Shuntu => 0b01,
            MJCombCategory::Koutu | MJCombCategory::Kantu => 0b10,
            MJCombCategory::Tuitu => 0b11
        }
    }

    pub fn new() -> AgariPattern { AgariPattern(0) }
    pub fn empty(self) -> bool { self.0 == 0 }
    pub fn len(self) -> usize { 
        let mut sz: usize = 0;
        let mut code = self.0;
        while code > 0 {
            sz += 1; code >>= 2;
        };
        sz
     }
    pub fn iter(self) -> <Self as IntoIterator>::IntoIter {
        AgariPatternIter(0, self)
    }
    pub fn reversed(self) -> AgariPattern {
        let len = self.len() as u16;
        let mut revd = Self::new();
        for i in 0..len {
            revd.0 = (revd.0 << 2) | ((self.0 >> 2*i) & 0b11);
        }
        revd
    }

    pub fn has_tuitu(self) -> bool {
        (self.0 & 0b11) == 0b11 || ((self.0 >> 2) & 0b11) == 0b11 || ((self.0 >> 4) & 0b11) == 0b11
        || ((self.0 >> 6) & 0b11) == 0b11 || ((self.0 >> 8) & 0b11) == 0b11
    }

    pub fn push(&mut self, cate: MJCombCategory) {
        self.0 = (self.0 << 2) | Self::encode_comb(cate) as u16;
    }

    pub fn assign(&mut self, pos: u8, cate: MJCombCategory) {
        self.0 = (self.0 & !(0b11u16 << ((pos*2)))) | ((Self::encode_comb(cate) as u16) << (pos*2)); 
    }
}

impl Index<u8> for AgariPattern {
    type Output = Option<MJCombCategory>;
    fn index(&self, index: u8) -> &Self::Output {
        match (self.0 >> (index*2)) & 0b11 {
            0b01 => &Some(MJCombCategory::Shuntu),
            0b10 => &Some(MJCombCategory::Koutu),
            0b11 => &Some(MJCombCategory::Tuitu),
            _ => &None
        }
    }
}

impl From<MJCombCategory> for AgariPattern {
    fn from(value: MJCombCategory) -> Self {
        AgariPattern(Self::encode_comb(value) as u16)
    }
}

impl IntoIterator for AgariPattern {
    type Item = MJCombCategory;
    type IntoIter = AgariPatternIter;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct AgariPatternIter(u8, AgariPattern);
impl Iterator for AgariPatternIter {
    type Item = MJCombCategory;
    fn next(&mut self) -> Option<Self::Item> {
        if (self.0 as usize) < AgariPattern::MAX_COMB_COUNT {
            self.0 += 1;
            self.1[self.0 - 1]
        } else {
            None
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if (self.0 as usize) + n < AgariPattern::MAX_COMB_COUNT {
            self.0 += (n as u8)+1;
            self.1[self.0 - 1]
        } else {
            None
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AgariPatternSeq {
    flipped_code: NonZeroU64    // for memory-saved Option<Self>
}

impl AgariPatternSeq {
    pub const MAX_PATTERN_COUNT: usize = 6;

    fn code(self) -> u64 { !self.flipped_code.get() }
    pub fn new() -> AgariPatternSeq {
        AgariPatternSeq{ flipped_code: unsafe{ NonZeroU64::new_unchecked(u64::MAX) } }
    }
    pub fn empty(self) -> bool { self.code() == 0 }
    pub fn len(self) -> usize { 
        let mut sz: usize = 0;
        let mut code = self.code();
        while code > 0 {
            sz += 1; code >>= 10;
        };
        sz
    }
    pub fn get(self, index: u8) -> Option<AgariPattern> {
        match (self.code() >> (10*(index as u64))) & 0x3ff {
            0 => None,
            t => Some(AgariPattern(t as u16))
        }
    }
    pub fn iter(self) -> <Self as IntoIterator>::IntoIter {
        AgariPatternSeqIter(0, self.len() as u8, self)
    }

    pub fn set_if_existed(&mut self, index: u8, pattern: AgariPattern) {
        if (index as usize) < self.len() {
            self.flipped_code = unsafe { NonZeroU64::new_unchecked(
                !((self.code() & !(0x3ffu64 << (10*index))) | ((pattern.0 as u64) << (10*index)))
            )};
        }
    }

    pub fn push(&mut self, pattern: AgariPattern) {
        debug_assert!(self.len() <= Self::MAX_PATTERN_COUNT);
        self.flipped_code = unsafe { NonZeroU64::new_unchecked(!((self.code() << 10) | pattern.0 as u64)) };
    }
}

impl From<AgariPattern> for AgariPatternSeq {
    fn from(value: AgariPattern) -> Self {
        let mut seq = AgariPatternSeq::new();
        seq.push(value);
        seq
    }
}

impl IntoIterator for AgariPatternSeq {
    type Item = AgariPattern;
    type IntoIter = AgariPatternSeqIter;
    
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct AgariPatternSeqIter(u8, u8, AgariPatternSeq);

impl Iterator for AgariPatternSeqIter {
    type Item = AgariPattern;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 < self.1 {
            self.0 += 1;
            self.2.get(self.0 - 1)
        } else { None }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if (self.0 as usize) + n < (self.1 as usize) {
            self.0 += (n as u8)+1;
            self.2.get(self.0 - 1)
        } else { None }
    }
}

impl DoubleEndedIterator for AgariPatternSeqIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.0 < self.1 {
            self.1 -= 1;
            self.2.get(self.1)
        } else { None }
    }
}