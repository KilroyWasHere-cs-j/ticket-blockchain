use gblock;


#[tokio::main]
async fn main() {
    println!("Welcome to Thea ticketing system node");
    println!("Initalizing...");
    match gblock::init() {
        Ok(()) => {
            println!("Initalizing successful!");
            println!("Attempting to spinup node...");
            match gblock::spinup().await  {
                Ok(()) => {
                    println!("Borscht good");
                },
                Err(e) => {
                    println!("Node failed to spinup saying {}", e);
                }
            }
        },
        Err(e) => { 
            println!("Node failed to Initalized saying {}", e);
        }
    }
}
