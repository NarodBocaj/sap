use rand::Rng;

//#[derive(Clone)]
pub struct Friend{
    pub attack: i32,
    pub health: i32,
    pub name: String,
    pub tier: i32,
    pub xp: i32,
}


impl Friend{
    pub fn faint(&self, friendly_friends: &mut Vec<Friend>, idx: i32) -> (){
        let lvl = (self.xp / 3) + 1;
        
        if self.name == "ant".to_string(){
            let rand_num = rand::thread_rng().gen_range(0..friendly_friends.len());
            println!("ant fainting, rand inx ={}", rand_num);
            friendly_friends[rand_num as usize].attack += 2 * lvl;
            friendly_friends[rand_num as usize].health += 1 * lvl;
        }

        if self.name == "cricket".to_string(){
            let zombie_cricket = Friend{
                attack: 1 * lvl,
                health: 1 * lvl,
                name: "zombie_cricket".to_string(),
                tier: 1,
                xp: 1,
            };
            friendly_friends.insert(idx as usize,zombie_cricket);
        }

        if self.name == "flamingo".to_string(){
            if idx - 1 >= 0{
                friendly_friends[(idx - 1) as usize].attack += 1 * lvl;
                friendly_friends[(idx - 1) as usize].health += 1 * lvl;
            }
            if idx - 2 >= 0{
                friendly_friends[(idx - 2) as usize].attack += 1 * lvl;
                friendly_friends[(idx - 2) as usize].health += 1 * lvl;
            }
        }
    }
}