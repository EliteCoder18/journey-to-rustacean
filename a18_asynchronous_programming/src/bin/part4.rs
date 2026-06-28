use std::time::Duration;
fn main(){
    trpl::block_on(async {


        let (tx,mut rx) = trpl::channel();
            let tx1= tx.clone();

        let tx_fut = async move{//here move moves the ownership fof tx to the block

                let vals = vec![
                    String::from("hello"),
                    String::from("hi"),
                    String::from("by"),
                    String::from("take care"),
                ];

                for val in vals{
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

        let rx_fut = async{


            while let Some(value)= rx.recv().await{// in rust there is no way to use for for asynchronously produced series of messages so while let continues till the patern matches the some value
            println!("the message is : {}", value);
            }
        };

        let tx1_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };
        
        trpl::join!(tx_fut, rx_fut, tx1_fut); //trpl::join is used for two futures and trpl::join! macro is used when u r 
        // using arbitary no of futtures known at compile time

       });
    //The synchronous Receiver::recv method in std::mpsc::channel blocks until it receives a message.
    //The trpl::Receiver::recv method does not, because it is async. Instead of blocking, it hands 
    // control back to the runtime until either a message is received or the send side of the channel closes
}
