pub trait summary {
    fn summarize (&self)->String;
    fn summarize_author(&self){
        format!("Read more from {}...",self.summarize());
    }// here we have also transferred the behaviour of summarize_author to 
    // newsarticle and social post without even inserting the funciton directly

}
pub trait ToString{}
pub trait display{}
pub struct NewsArticle{
    pub headline: String,
    pub location:String,
    pub author: String,
    pub content: String,
}

impl summary for NewsArticle{
    fn summarize(&self)-> String{
        format!("{}, by {} ({})",self.headline, self.author,self.location)
    }
}

pub struct SocialPost{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}


impl summary for SocialPost{
    fn summarize(&self)->String{
        format!("{}: {}", self.username, self.content)
    }
}

// here we have implemented a summary as a type in a parameter
pub fn notify(item: &impl summary) {
    format!("Breaking news! {}", item.summarize());
}// here we can call any thing that is in the summary trait
// here we also have ......................trait bound syntax
pub fn notifyyyy<T:summary>(item: &T){
    println!("Breaking new! {}", item.summarize());
}
// here it works some but its more complex 
// 
pub fn notifyy(item1: &impl summary, item2: &impl summary) {}
// here we have implemented summary here item 1 and item 2 can have different types
// but if you want to force them to have same type then
pub fn notifyyy<T:summary>(item1: &T, item2:&T){}
// supposing if we want the funciton to use more than one trait you can do that 
// with + sign
pub fn notiffy(item: &(impl summary + display)) {}
pub fn notify1<T: summary + display>(item: &T) {}
//some type using so many trait becomes hard for coder to read so
// instead of fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// this we can 
fn some_function<T,U>(t:&T,u:&U)->i32
where
        T: display +ToString,
        U: summary+display,
{

8
}
// Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations 
impl<T:display>ToString for T{

}// here if any function has trait display it can use the trait ToString 







fn main(){
    let post = SocialPost{
        username: String::from("horse_ebooks"),
        content:String::from("of course, as you 
        probably already know, people "),
        reply:false,
        repost:false,
    };
    println!("1 new post : {}", post.summarize());
    post.summarize_author();
}


