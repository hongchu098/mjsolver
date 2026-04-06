use super::mjcore;
use super::pattern::*;
use mjcore::mjtile::MJTile;
use mjcore::mjcomb::MJCombCategory;
use std::ops::{Index, IndexMut};
use std::cmp::max;

/*
    dp: [AgariPattern; [14, 15]]
    按排序后每种麻将牌的个数进行DP, dp[i][j]表示前i种牌(包括第i种)以状态j结尾组成的胡牌模式(AgariPattern)，
    j的取值含义为
        0       => 前i种牌组成了合法的胡牌模式，没有残留,
        1..=4   => 前i种牌合法胡牌模式末尾残留j张相同的单牌,
        5..=8   => 前i种牌合法胡牌模式末尾残留j-5张相同的单牌,和1组不同的牌(每组2张相邻数牌),
        9..=11  => 前i种牌合法胡牌模式末尾残留j-9张相同的单牌,和2组不同的牌(每组2张相邻数牌),
        12 | 13 => 前i种牌合法胡牌模式末尾残留j-12张相同的单牌,和3组不同的牌(每组2张相邻数牌),
        14      => 前i种牌合法胡牌模式末尾残留4组不同的牌(每组2张相邻数牌);
    最终结果在dp[len-1][0]处取得, 若dp[len-1][0].empty(), 则输入的牌不存在合法的胡牌模式
*/
struct SearchDPTable([[Option<AgariPatternSeq>; 15]; 14]);
impl SearchDPTable {
    fn new() -> SearchDPTable { SearchDPTable([[None; 15]; 14]) }
    fn get(&self, i: usize, j: usize) -> &Option<AgariPatternSeq> {
        unsafe { self.0.get_unchecked(i).get_unchecked(j) }
    }
    fn set(&mut self, i: usize, j: usize, value: AgariPatternSeq) {
        *self.index_mut((i,j)) = Some(value);
    }
    fn legalize_ij(&mut self, i: usize, j:usize) {
        let x = match *self.get(i,j) {
            Some(x) => x,
            _ => AgariPatternSeq::new()
        };
        self[(i,j)] = Some(x);
    }
    fn ij_push(&mut self, i: u8, j: u8, patt:AgariPattern) {
        let (i,j) =(i as usize, j as usize);
        let mut x = match *self.get(i,j) {
                Some(x) => x,
                _ => AgariPatternSeq::new()
            };
        x.push(patt);
        self[(i,j)] = Some(x);
    }
    fn foreach_at_least<Iter, F>(patt_iter: Iter, mut f: F) where
    Iter: Iterator<Item=AgariPattern>,
    F: FnMut(AgariPattern) {
        let mut empty = true;
        for patt in patt_iter {
            f(patt);
            empty = false;
        }
        if empty { f(AgariPattern::new()); }
    }
    fn foreach_clone_from<Iter>(&mut self, patt_iter: Iter, i: u8, j: u8)
    where Iter: Iterator<Item=AgariPattern> {
        Self::foreach_at_least(patt_iter, |patt| self.ij_push(i, j, patt));
    }
    fn foreach_push_n_with(
        &mut self, patt_seq: AgariPatternSeq, i: u8, j: u8,
        cate: MJCombCategory, repeat_n: u8
    ){
        if cate == MJCombCategory::Tuitu {
            Self::foreach_at_least(patt_seq.iter(), |mut patt| {
                // repeat_n will be ignored while push Tuitu
                if !patt.has_tuitu() && patt.len() < AgariPattern::MAX_COMB_COUNT {
                    patt.push(MJCombCategory::Tuitu);
                    self.ij_push(i, j, patt); }
            });
        } else {
            Self::foreach_at_least(patt_seq.iter(), |mut patt| {
                if patt.len() + (repeat_n as usize) <= AgariPattern::MAX_COMB_COUNT {
                    for _ in 0..max(1, repeat_n){
                        patt.push(cate); self.ij_push(i, j, patt);
                    }
                }
            });
        }
    }
    fn foreach_push_with(&mut self, patt_seq: AgariPatternSeq, i: u8, j: u8, cate: MJCombCategory) {
        self.foreach_push_n_with(patt_seq, i, j, cate, 1)
    }

    #[allow(non_upper_case_globals)]
    fn update_i_with_jlrn(&mut self, patt_seq: AgariPatternSeq, i: u8, j: u8, l: MJTile, r: MJTile, n: u8, lv: u8) {
        const Tuitu:  MJCombCategory = MJCombCategory::Tuitu;
        const Koutu:  MJCombCategory = MJCombCategory::Koutu;
        const Shuntu: MJCombCategory = MJCombCategory::Shuntu;
        let lv2b = |lv: u8| (11-lv)*lv / 2;
        match j {
            0 => {
                let b= lv2b(lv);
                self.foreach_clone_from(patt_seq.iter(), i, b+n);
                match n {
                    2 => self.foreach_push_with(patt_seq, i, b, Tuitu),
                    3 => {
                        self.foreach_push_with(patt_seq, i, b+1, Tuitu);
                        self.foreach_push_with(patt_seq, i, b, Koutu);
                    },
                    4 => {
                        self.foreach_push_with(patt_seq, i, b+2, Tuitu);
                        self.foreach_push_with(patt_seq, i, b+1, Koutu);
                    }
                    _ => ()
                };
            },
            1..=4 => if n >= j && l.is_number_prev_to(r) {
                    self.update_i_with_jlrn(patt_seq, i, 0, l, r, n-j, j);
            },
            5..=14 => {
                let lv_ = match j {
                    5..=8 => 1u8,
                    9..=11 => 2,
                    12 | 13 => 3,
                    14 => 4,
                    _  => 0
                };
                let lq = j-lv2b(lv_);
                if n >= lq+lv_ && l.is_number_prev_to(r) {
                    let patt_seq = {
                        let mut patt_seq_ = AgariPatternSeq::new();
                        Self::foreach_at_least(patt_seq.iter(), |mut patt| {
                            if patt.len() + (lv_ as usize) <= AgariPattern::MAX_COMB_COUNT {
                                for _ in 0..lv_ {
                                    patt.push(Shuntu);
                                }
                                patt_seq_.push(patt);
                            }
                        });
                        patt_seq_
                    };
                    self.update_i_with_jlrn(patt_seq, i, 0, l, r, n-(lq+lv_), lq);
                };
            },
            _ => ()
        }
    }
}
impl Index<usize> for SearchDPTable {
    type Output = [Option<AgariPatternSeq>; 15];
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.0.get_unchecked(index) }
    }
}
impl Index<(usize,usize)> for SearchDPTable {
    type Output = Option<AgariPatternSeq>;
    fn index(&self, index: (usize,usize)) -> &Self::Output {
        self.get(index.0, index.1)
    }
}
impl IndexMut<(usize, usize)> for SearchDPTable {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        unsafe { self.0.get_unchecked_mut(index.0).get_unchecked_mut(index.1) }
    }
}

pub fn search_pattern(tiles: &[MJTile]) -> Vec<AgariPattern> {
    let mut sorted = tiles.to_vec();
    sorted.sort_unstable();
    return search_pattern_sorted(&sorted);
}

#[allow(non_upper_case_globals)]
pub fn search_pattern_sorted(sorted_tiles: &[MJTile]) -> Vec<AgariPattern> {

    let tiles_len = sorted_tiles.len();
    debug_assert!(tiles_len > 0 && tiles_len <= 14);
    const Tuitu:  MJCombCategory = MJCombCategory::Tuitu;
    const Koutu:  MJCombCategory = MJCombCategory::Koutu;
    // const Shuntu: MJCombCategory = MJCombCategory::Shuntu;
    
    let tile_counts = {
        let mut counts: Vec<(MJTile,u8)> = Vec::with_capacity(tiles_len);
        for &tile in sorted_tiles {
            if let Some(last) = counts.last_mut() {
                if tile == last.0 { last.1 += 1; continue; }
            }
            counts.push((tile, 1));
        };
        counts
    };
    let mut dp = Box::new(SearchDPTable::new());
    {
        let empty_seq = AgariPatternSeq::new();
        let tuitu_seq: AgariPatternSeq = AgariPattern::from(Tuitu).into();
        let koutu_seq: AgariPatternSeq = AgariPattern::from(Koutu).into();
        let (_, n) = unsafe { *tile_counts.first().unwrap_unchecked() } ;
        dp[(0,n as usize)] = Some(empty_seq);
        match n {
            2 => dp[(0,0)] = Some(tuitu_seq),
            3 => { dp[(0,0)] = Some(koutu_seq); dp[(0,1)] = Some(tuitu_seq); }
            4 => { dp[(0,1)] = Some(koutu_seq); dp[(0,2)] = Some(tuitu_seq); }
            _ => ()
        };
    }
    for (i,dbl) in tile_counts.windows(2).enumerate() {
        let i = i + 1;
        let (l, _) = unsafe { *dbl.get_unchecked(0) };
        let (r, n) = unsafe { *dbl.get_unchecked(1) };
        for j in 0..14usize {
            match dp[i-1][j] {
                Some(patt_seq) =>
                    dp.update_i_with_jlrn(patt_seq, i as u8, j as u8, l, r, n, 0),
                _ => {}
            }
        }
    };

    if let Some(final_seq) = dp[tile_counts.len()-1][0] 
    {
        Vec::from_iter(final_seq.iter().rev().map(|patt| patt.reversed()))
    } else {
        Vec::new()
    }
}