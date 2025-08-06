pub trait Summary{
    fn summarize(&self) ->String;
}
// trait always have a return type string 
pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub content: String,
}
pub struct SocialPost {
    pub username: String,
    pub content: String,
}
impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("{} from {}", self.headline, self.location)
    }
}
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}
fn notify(item:&impl Summary){
      println!("Summary: {}", item.summarize());
}
fn main(){
    let article = NewsArticle{
        headline: String::from("Big News Today!"),
        location: String::from("Delhi"),
        content: String::from("Something important happened."),
    };
    let post =SocialPost{
        username:String::from("mayank"),
        content:String::from("Hello Rustaceans!"),
    };
    notify(&article); 
    notify(&post);
}