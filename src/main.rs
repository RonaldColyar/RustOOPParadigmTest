

struct User{
    username : String ,
    password : String,
    display_name: String,
    phone_number : String,
    age : String
    
}
impl User{
    // constructor
    pub fn new(user:&str , 
            password:&str , 
            display_name:&str , 
            pn:&str , 
            age :&str)-> User{

         return User{
            username:user.to_string(),
            password:password.to_string(),
            display_name:display_name.to_string(),
            phone_number:pn.to_string(),
            age:age.to_string()
        };
    }
    pub fn set_name(&mut self ,new_name : &str){
        self.display_name = new_name.to_string();
    }
    pub fn change_password(&mut self , new_password : &str){
        self.password = new_password.to_string();

    }
    pub fn change_username(&mut self , new_username : &str){
        self.username = new_username.to_string();
    }
    pub fn change_phone_number(&mut self , new_num :&str){
        self.phone_number = new_num.to_string();
    }
}

fn main() {
   let mut data :User = User::new(
       "ron",
       "000",
       "theking",
       "1223234332",
       "45"
   );
}
