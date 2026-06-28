use trpl::Html;
use std::time::Duration;

async fn page_title(url: &str)-> Option<String>{
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text).select_first("title").map(|title| title.inner_html())

    
}
fn page_title_1(url: &str)-> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text).select_first("title").map(|title| title.inner_html())
    }
}
fn main(){//fn main cannot be a async function itself as it is not a runtime itself

//     let args: Vec<String> = std::env::args().collect();
//     trpl::block_on(async {
//     let url = &args[1];
//     match page_title(url).await {
//         Some(title) => println!("the title for {url} was {title}"),
//         None => println!("{url} had no title"),}
// });

    trpl::block_on(async{
        //   let handle =  trpl::spawn_task(async{
                let fut1 = async {for i in 1..10{
                    println!("hi number {i} from the first task");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };
            // });
          let fut2 = async{ for i in 1..5{
                    println!("hi number {i} from the second task");
                    trpl::sleep(Duration::from_millis(500)).await;
            }};
            trpl::join(fut1,fut2).await;
            // handle.await.unwrap();
        
        });

}