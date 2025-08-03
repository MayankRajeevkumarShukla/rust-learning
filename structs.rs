struct User{
    activte:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}
fn main(){
    let mut user1 = User{
        activte:true,
        username:String::from("someusername123"),
        email:String::from("someone@example.com"),
        sign_in_count:1,

    };
    user1.email = String::from("Ag");
       let user2 = User {
        // active: user1.active,
        // username: user1.username,
        // email: String::from("another@example.com"),
        // sign_in_count: user1.sign_in_count,
        // this can be also retuen as 
        email:String::from("another@example.com"),
        ..user1
    };
}
fn build_user(email:String,username:String) ->User{
    // User{
    //     activte:true,
    //     username:username,
    //     email:email,
    //     sign_in_count:1,
    // };
    // with init shorthand bcoz the fun parameter is same as struct
    User{
        activte:true,
        username,
        email,
        sign_in_count:1,
    }
}
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn structTuple(){
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}
