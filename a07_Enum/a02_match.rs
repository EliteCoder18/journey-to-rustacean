
#[derive(Debug)]

enum UsState{
    Alabama,
    Alaska,
}




enum Coin{
    penny,
    nickel,
    dime, 
    quarter(UsState),
}

fn value_in_cents(coin: Coin)->u8{
    match coin{
        Coin::penny=>{println!("the coin is penny");
            1} // here if you want to used multiple line then use curly brackets else 
            // comma
        Coin::nickel=>5,
        Coin::dime=>10,
        Coin::quarter(state)=>{
            println!("State quarter from {state:?}!");
            25},
    }
}


fn add_fancy_hat(){}
fn remove_fancy_hat(){}
fn move_player(space:u8){}
fn reroll(){}

fn main(){
   let coin1=Coin::penny;
    value_in_cents(coin1);
    let dice_roll=8;


match dice_roll{//here match can be also used for any other value
    3=>add_fancy_hat(),
    7=>remove_fancy_hat(),
    other=>move_player(other),// this arm catches all the value and it needs to be put at last 
    // because match runs in order and it wont let other conditions to be checked
}

match dice_roll1{
    3=>add_fancy_hat(),
    7=>remove_fancy_hat(),
    _=>reroll(),// this "_" runs and catches all value but does not bind to that value 
}
    value_in_cents(Coin::quarter(UsState::Alaska));//giving enum inside enum
}