use std::ops::Range;
pub struct LoaContents {
    contents: Vec<(u32, u32, u32)>,
}

impl LoaContents {

    // higher contents must have lower index
    pub fn new(contents: Vec<(u32, u32, u32)>) -> LoaContents {
        LoaContents {
            contents
        }
    }

    pub fn to_integer(&self, lv: &f32) -> u64 {
        let mut value: u64 = 0;
        for (s, (l, r, _)) in self.contents.iter().enumerate() {
            if (*l as f32..*r as f32).contains(&lv) {
                value |= 1 << s;
            }
        }
        println!("lv: {}, contents: {:016b}", lv, value);
        value
    }

    pub fn cal_gold(&self, integer: &u64) -> u32 {
        let mut integer = *integer;
        let mut gold = 0;
        let mut limit = 3;
        let mut is_brel = false;

        for idx in 0..64 {
            if integer & 1 == 1 && limit > 0{
                gold += self.contents[idx].2;
                if (2..8).contains(&idx) {
                    if !is_brel {
                        is_brel = true;
                        limit -= 1;
                    }
                } else {
                    limit -= 1;
                }
            }
            integer >>= 1;
        }
        gold
    }

}