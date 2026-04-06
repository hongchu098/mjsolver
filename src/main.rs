
mod mjcore;
mod agari;

use std::str::FromStr;
use mjcore::{mjtile, mjcomb};
use mjtile::{MJTile, MJTileCategory};
use mjcomb::{MJComb, MJCombCategory, MJCombSeq, MJCombRedFlags};
use agari::pattern::AgariPatternSeq;
use agari::pattern_search::search_pattern;

use crate::agari::pattern::AgariPattern;


fn printAgariPatterns(patterns: &[AgariPattern]) {
    patterns.iter().for_each(|patt|{
        print!("  ");
        patt.iter().for_each(|comb_cate|{
            let c = match comb_cate {
                MJCombCategory::Shuntu => 'S',
                MJCombCategory::Koutu => 'K',
                MJCombCategory::Kantu => 'G',
                MJCombCategory::Tuitu => 'T'
            };
            print!("{} ", c);
        });
        println!("");
    });
}

fn main() {
    let tile = MJTile::from_str("9Sr").unwrap();
    let comb = MJComb::new(MJCombCategory::Shuntu, "5M".try_into().unwrap(), (1,0,0,0).into());
    let mut combseq = MJCombSeq::new();
    combseq.push(comb.clone());
    combseq.push(MJComb::new(MJCombCategory::Tuitu, "6Sr".try_into().unwrap(), MJCombRedFlags::NONE));
    println!("Hello, world!");
    println!("{tile}: is_yaochuuhai({}) is_red({})", tile.is_yaochuuhai(), tile.is_red());
    println!("comb: {comb}");
    println!("combseq: [{} {}]", combseq[0], combseq[1]);
    println!("Option<AgariPatternSeq> Size: {} bytes", size_of::<Option<AgariPatternSeq>>());

    // std::vector<MJTile> seq1{ "1S"_T, "2S"_T, "3S"_T, "5S"_T, "6S"_T, "7S"_T, "1M"_T, "2M"_T, "3M"_T, "5M"_T, "6M"_T, "7M"_T, "S"_T, "S"_T };
    let seq1 = {
        let mut seq1_ = Vec::<MJTile>::new();
        for s in ["1S", "2S", "3S", "5S", "6S", "7S", "1M", "2M", "3M", "5M", "6M", "7M", "S", "S"] {
            seq1_.push(MJTile::try_from(s).unwrap());
        }
        seq1_
    };
    let patts1 = search_pattern(&seq1);
    println!("patts1:"); printAgariPatterns(&patts1);
    
    // std::vector<MJTile> seq3{ "1S"_T, "1S"_T, "1S"_T, "2S"_T, "2S"_T, "2S"_T, "2S"_T, "3S"_T, "3S"_T, "3S"_T, "3S"_T, "4S"_T, "4S"_T, "4S"_T };
    let seq2 = {
        let mut seq2_ = Vec::<MJTile>::new();
        for s in ["1S", "1S", "1S", "2S", "2S", "2S", "2S", "3S", "3S", "3S", "3S", "4S", "4S", "4S"] {
            seq2_.push(MJTile::try_from(s).unwrap());
        }
        seq2_
    };
    let patts2 = search_pattern(&seq2);
    println!("patts2:"); printAgariPatterns(&patts2);
    // std::vector<MJTile> seq1{ "2M"_T, "2M"_T, "3M"_T, "3M"_T, "3M"_T, "4M"_T, "4M"_T, "4M"_T, "5M"_T, "5M"_T, "5M"_T, "6M"_T, "6M"_T, "6M"_T };
    let seq3 = {
        let mut seq3_ = Vec::<MJTile>::new();
        for s in ["2M", "2M", "3M", "3M", "3M", "4M", "4M", "4M", "5M", "5M", "5M", "6M", "6M", "6M"] {
            seq3_.push(MJTile::try_from(s).unwrap());
        }
        seq3_
    };
    let patts3 = search_pattern(&seq3);
    println!("patts3:"); printAgariPatterns(&patts3);
}
