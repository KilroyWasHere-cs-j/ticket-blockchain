use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    f_name: String, // First name
    l_name: String, // Last name
    s_name: String, // Show name
    t_name: String, // Theater name
    s_let: char, // Seat letter
    s_num: i64, // Seat number
    s_acom: String, // Special accomidations
}

impl Ticket {
    pub fn new() -> Self {
        Ticket {
            f_name: "".to_string() ,
            l_name: "".to_string(),
            s_name: "".to_string(),
            t_name: "".to_string(),
            s_let: 'a',
            s_num: 0,
            s_acom: "No acoms needed".to_string(),
        }
    }

    pub fn generate_new(f_name: String, l_name: String, s_name: String, t_name: String, s_let: char, s_num: i64, s_acom: String) -> Ticket {
        Ticket {
            f_name,
            l_name,
            s_name,
            t_name,
            s_let,
            s_num,
            s_acom,
        }
    }
}
