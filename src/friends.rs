#[derive(Clone)]
pub struct Friend{
    pub attack: i32,
    pub health: i32,
    pub name: String,
    pub tier: i8,
    pub xp: i8,
}


impl Friend{
    pub fn faint(&self, friendly_friends: &mut Vec<Friend>) -> Vec<Friend>{
        if self.name == "ant".to_string(){
            println!("ant fainting");
            friendly_friends[0].health += 1;
            return friendly_friends.to_vec()
        }
        return friendly_friends.to_vec()
    }
}