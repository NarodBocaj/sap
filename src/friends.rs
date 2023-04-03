pub mod shop;

use std::ops::Add;
use rand::{thread_rng, Rng};
//use rand::distributions::Uniform;
use std::cmp;
use crate::Game;


#[derive(Clone, Copy)]
pub struct Friend{
    pub attack: i32,//need to implement temp added attack
    pub health: i32,//need to implement temp added health I think? Other option is to only apply temp buffs to clone
    pub id: i32,
    pub tier: i32,
    pub xp: i32,
    pub food_id: i32,//maybe should make food based buff attributes
}


impl Friend{
    pub fn faint(&self, friendly_friends: &mut Vec<Friend>, idx: i32) -> (){
        let lvl = (self.xp / 3) + 1;
        
        if self.id == shop::ANT{
            if friendly_friends.len() > 0{
                let rand_num = rand::thread_rng().gen_range(0..friendly_friends.len());
                friendly_friends[rand_num as usize].attack += 2 * lvl;
                friendly_friends[rand_num as usize].health += 1 * lvl;
            }
        }

        if self.id == shop::CRICKET && friendly_friends.len() < 5{
            let zombie_cricket = Friend{
                attack: 1 * lvl,
                health: 1 * lvl,
                id: 58,
                tier: 1,
                xp: 1,
                food_id: -1,
            };
            friendly_friends.insert(idx as usize,zombie_cricket);
        }

        if self.id == shop::FLAMINGO{
            if friendly_friends.len() > 0{
                friendly_friends[idx as usize].attack += 1 * lvl;
                friendly_friends[idx as usize].health += 1 * lvl;
            }
            if friendly_friends.len() > 1{
                friendly_friends[(idx + 1) as usize].attack += 1 * lvl;
                friendly_friends[(idx + 1) as usize].health += 1 * lvl;
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
                let mut rng = thread_rng();
                let rand_nums = rand::seq::index::sample(&mut rng, friendly_friends.len(), 2).into_vec();
                friendly_friends[rand_nums[0] as usize].attack += 1 * lvl;
                friendly_friends[rand_nums[0] as usize].health += 1 * lvl;
                friendly_friends[rand_nums[1] as usize].attack += 1 * lvl;
                friendly_friends[rand_nums[1] as usize].health += 1 * lvl;
            }
        }

    }

    pub fn on_sell(&self, friendly_friends: &mut Vec<Friend>, game: &mut Game, _idx: i32) -> (){
        let lvl = (self.xp / 3) + 1;

        if self.id == shop::BEAVER{
            if friendly_friends.len() < 3{
                for i in 0..friendly_friends.len(){
                    friendly_friends[i].health += 1 * lvl;
                }
            }
            else{
                let mut rng = thread_rng();
                let rand_nums = rand::seq::index::sample(&mut rng, friendly_friends.len(), 2).into_vec();
                friendly_friends[rand_nums[0] as usize].health += 1 * lvl;
                friendly_friends[rand_nums[1] as usize].health += 1 * lvl;
            }
        }

        if self.id == shop::DUCK{
            for i in 0..game.shop.for_sale.len(){
                game.shop.for_sale[i].health += 1 * lvl;
            }
        }

        if self.id == shop::PIG{
            game.money += 1 * lvl;
        }

    }

    pub fn start_of_battle(&mut self, friendly_friends: &mut Vec<Friend>, opp_friends: &mut Vec<Friend>, idx: usize) -> (){
        let lvl = (self.xp / 3) + 1;

        if self.id == shop::MOSQUITO{
            let mut rng = thread_rng();
            let min_range = cmp::min(lvl, opp_friends.len() as i32);
            let rand_nums = rand::seq::index::sample(&mut rng, min_range as usize, lvl as usize).into_vec();
            for idx in rand_nums{
                do_dmg(opp_friends, 1, idx as usize);
            }
        }

        if self.id == shop::CRAB{
            let percent_hp = match lvl{
                1 => 0.5,
                2 => 1.0,
                3 => 1.5,
                _ => 1.0,
            };
            if friendly_friends.len() > 1{
                let max_hp = friendly_friends
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != idx) //exclude crab from copying it's own HP
                    .max_by_key(|(_, f)| f.health)
                    .unwrap()
                    .1
                    .health;
                let new_hp: i32 = cmp::max((max_hp as f32 * percent_hp) as i32, 1);
                self.health = new_hp;
             }
        }

        if self.id == shop::DODO{
            if idx > 0{//check that pet is infront of dod
                let percent_attack = match lvl{
                    1 => 0.33,
                    2 => 0.66,
                    3 => 1.00,
                    _ => 1.0,
                };
                let bonus_attack = (percent_attack * self.attack as f32) as i32;
                friendly_friends[idx - 1].attack += bonus_attack;
            }
        }

    }

    pub fn before_attack(&mut self, friendly_friends: &mut Vec<Friend>, idx: usize) -> (){
        let lvl = (self.xp / 3) + 1;

        if self.id == shop::ELEPHANT{
            let mut attack_count = 0;
            while idx + 1 < friendly_friends.len() && attack_count < lvl{
                do_dmg(friendly_friends, 1, idx + 1);
                attack_count += 1;
            }
        }

    }

}

impl Add for Friend{        //should function be banned for pets with diff ids?? how to handle when xp is greater than lvl 3??
    type Output = Self;
    //should add check here within add that see's if a lvl up occurs and then calls lvl up on shop
    fn add(self, other: Self) -> Self {
        Self{
            attack: cmp::max(self.attack, other.attack) + 1,
            health: cmp::max(self.health, other.health) + 1,
            id: self.id,
            tier: self.tier,
            xp: self.xp + other.xp,
            food_id: self.food_id,
        }
    }
}

pub fn friend_maker(id: i32, canned_food_cnt: i32) -> Friend{
    return Friend{
        attack: shop::PETS[id as usize].1 + canned_food_cnt,
        health: shop::PETS[id as usize].2 + canned_food_cnt,
        id: id,
        tier: tier_calc(id),
        xp: 1,
        food_id: -1,
    }
}

pub fn tier_calc(id: i32) -> i32{
    if id < 9{
        return 1
    }
    else if id < 19{
        return 2
    }
    else if id < 30{
        return 3
    }
    else if id < 41{
        return 4
    }
    else if id < 49{
        return 5
    }
    else{
        return 6
    }
}

#[derive(Clone, Copy)]
pub struct Food{
    pub id: i32,
}

pub fn do_dmg(team: &mut Vec<Friend>, dmg: i32, idx: usize) -> (){
    team[idx].health -= dmg;
    if team[idx].health < 1{
        let fainted_pet = team.remove(idx);
        fainted_pet.faint(team, 0);
    }
    else{
        //run the hurt ability
        //also need to make sure fainting is complete before running the hurt ability
        //maybe return an alive indicator so the hurt ability is run after
    }
}