pub struct TalentColumn {
    name: String,
    val: u32,
    talents: Vec<Talent>
}

pub impl TalentColumn {
    pub fn new(name: String, talents: Vec<Talent>) -> Self {
        TalentColumn {name: name, val: calc_col_value(&talents), talents: talents};

    }

    fn calc_col_value(talents: &Vec<Talent>) -> u32 {
        let mut val: u32 = 0;
        for i in 0..talents.len() {
            val += talents[i].value;
        }
        val
    }

    pub fn create_empty(name: String) -> Self {
        TalentColumn {name: name, val: 0, talents: vec![]}
    }
}