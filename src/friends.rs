pub mod shop;

use rand::Rng;


//#[derive(Clone)]
pub struct Friend{
    pub attack: i32,//need to implement temp added attack
    pub health: i32,//need to implement temp added health I think?
    pub id: i32,
    pub tier: i32,
    pub xp: i32,
}


impl Friend{
    pub fn faint(&self, friendly_friends: &mut Vec<Friend>, idx: i32) -> (){
        let lvl = (self.xp / 3) + 1;
        
        if self.id == shop::ANT{
            let rand_num = rand::thread_rng().gen_range(0..friendly_friends.len());
            println!("ant fainting, rand idx ={}", rand_num);
            friendly_friends[rand_num as usize].attack += 2 * lvl;
            friendly_friends[rand_num as usize].health += 1 * lvl;
        }

        if self.id == shop::CRICKET{
            let zombie_cricket = Friend{
                attack: 1 * lvl,
                health: 1 * lvl,
                id: 100,
                tier: 1,
                xp: 1,
            };
            friendly_friends.insert(idx as usize,zombie_cricket);
        }

        if self.id == shop::FLAMINGO{
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

    //we will get around issues related to index randomness right now by implementing buy effects before putting a pet into an index
    pub fn buy(&self, friendly_friends: &mut Vec<Friend>, _idx: i32) -> (){
        let lvl = (self.xp / 3) + 1;

        if self.id == shop::OTTER{
            if friendly_friends.len() <= 2{
                for i in 0..friendly_friends.len(){
                    friendly_friends[i].attack += 1 * lvl;
                    friendly_friends[i].health += 1 * lvl;
                }
            }
            else{
                //not sure this random for second index is working as intended
                let rnd_idx1 = rand::thread_rng().gen_range(0..friendly_friends.len());
                let rnd_idx2 = (0..friendly_friends.len()).filter(|x| *x != rnd_idx1).next().unwrap();
                friendly_friends[rnd_idx1 as usize].attack += 1 * lvl;
                friendly_friends[rnd_idx1 as usize].health += 1 * lvl;
                friendly_friends[rnd_idx2 as usize].attack += 1 * lvl;
                friendly_friends[rnd_idx2 as usize].health += 1 * lvl;
            }
        }


    }
}

pub fn friend_maker(id: i32) -> Friend{
    return Friend{
        attack: shop::PETS[id as usize][1],
        health: shop::PETS[id as usize][2],
        id: id,
        tier: 1, //need to make a tier calculator
        xp: 1,
    }
}

