#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // add more states if needed
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}




fn main(){
    let config_max=Some(3u8);
    match config_max{
        Some(max)=>println!("the maxium is confgured to be {max}"),
        _ =>(), // here we need to write_=>() but we dont need such value
    }
    
//................................if else
    // instead we can do this by 
    if let Some(max)= config_max{
        println!("the maximum is configured to be {max}");
    }



}