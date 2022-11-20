use lazy_static::lazy_static;

const END_LV: u64 = 99999;

pub struct LoaContents {
    pub contents: Vec<(u64, u64, u64)>,
}

lazy_static! {
    pub static ref LOA_CONTENTS: LoaContents = LoaContents { 
        contents: vec![
            (1600, END_LV, 6500), (1580, 1600, 5500),
            (1560, END_LV, 3000), (1550, END_LV, 2000), (1540, END_LV, 5500),
            (1520, 1560, 2500), (1500, 1550, 1500), (1490, 1540, 4500),
            (1475, END_LV, 4500), (1460, END_LV, 4500), (1445, END_LV, 4500),
            (1430, 1460, 2500), (1415, 1445, 2500)
        ]
    };
}

impl LoaContents {
    // higher contents must have lower index
    pub fn get_hw(&self, lv: &f64) -> u64 {
        let mut hw: u64 = 0;
        for (s, (l, r, _)) in self.contents.iter().enumerate() {
            if (*l as f64..*r as f64).contains(&lv) {
                hw |= 1 << s;
            }
        }
        hw
    }

    pub fn cal_gold(&self, integer: &u64) -> u64 {
        let mut integer = *integer;
        let mut gold = 0u64;
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