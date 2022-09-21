pub struct user_info {
    user_name: String,
    user_character: Vec<char_info>,
}

pub struct char_info {
    char_name: String,
    char_lv: u32,
}

impl user_info {
    pub fn new(user_name: String, user_character: Vec<char_info>) -> user_info {
        user_info { user_name, user_character }
    }
    
}

impl char_info {
    pub fn new(char_name: String, char_lv: u32) -> char_info {
        char_info { char_name, char_lv }
    }
}