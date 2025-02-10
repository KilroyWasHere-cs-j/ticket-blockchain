use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    fname: String, // First name
    lname: String, // Last name
    sname: String, // Show name
    tname: String, // Theater name
    sLet: char, // Seat letter
    sNum: i64, // Seat number
    sAcom: String, // Special accomidations
}

impl Ticket {
    pub fn new() -> Self {
        Ticket {
            fname: "".to_string() ,
            lname: "".to_string(),
            sname: "".to_string(),
            tname: "".to_string(),
            sLet: 'a',
            sNum: 0,
            sAcom: "No acoms needed".to_string(),
        }
    }

    pub fn generate_new(fname: String, lname: String, sname: String, tname: String, sLet: char, sNum: i64, sAcom: String) -> Ticket {
        Ticket {
            fname,
            lname,
            sname,
            tname,
            sLet,
            sNum,
            sAcom,
        }
    }
}
