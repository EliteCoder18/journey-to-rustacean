use std::time::Duration;


fn main(){
    trpl::block_on( async{

            
                    let slow = async{
                    trpl::sleep(Duration::from_millis(50)).await;
                    "finally finished"
            };

            match timeout(slow, Duration::from_secs(2)).await{
                Ok(message)=>println!("succeded with '{message}'"),
                Err(duration)=>{
                    println!("Failed after {} seconds"duration.as_secs())

                }

            }

        
    } );     
}