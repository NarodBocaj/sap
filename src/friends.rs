pub mod shop;

use std::ops::Add;
use rand::{thread_rng, Rng};
//use rand::distributions::Uniform;
use std::cmp;

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
                println!("ant fainting, rand idx = {}", rand_num);
                friendly_friends[rand_num as usize].attack += 2 * lvl;
                friendly_friends[rand_num as usize].health += 1 * lvl;
            }
        }

        if self.id == shop::CRICKET{
            let zombie_cricket = Friend{
                attack: 1 * lvl,
                health: 1 * lvl,
                id: 100,
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

impl Food{
//food should have attribute like add to pet that firstly checks what food it is
//if something like pill, it will call faint on that pet
//if pear gives buff to that pet
//if garlic gives pet held food with that id
    pub fn give_food(&self, friendly_friends: &mut Vec<Friend>, shop: &mut shop::Shop, idx: usize) -> () {
        //check all the food ids to give appropriate affect
        if self.id == 0{//apple
            friendly_friends[idx].health += 1;
            friendly_friends[idx].attack += 1;
        }
        else if self.id == 1{//honey
            //figure out how to handle honey
        }
        else if self.id == 2{//pill
            //faint pet, I think I should write fill faint function
        }
        else if self.id == 3{//cupcake
            //temp buff with cupcake
        }
        else if self.id == 4{//meatbone
            friendly_friends[idx].food_id = self.id;
        }
        else if self.id == 5{//salad
            if friendly_friends.len() < 2{
                if friendly_friends.len() == 1{
                    friendly_friends[0].attack += 1;
                    friendly_friends[0].health += 1;
                }
                return
            }
            let mut rng = thread_rng();
            let rand_nums = rand::seq::index::sample(&mut rng, friendly_friends.len(), 2).into_vec();
            for i in rand_nums.iter().map(|&x| x as usize){
                friendly_friends[i].attack += 1;
                friendly_friends[i].health += 1;
            }
        }
        else if self.id == 6{//garlic
            friendly_friends[idx].food_id = self.id;
        }
        else if self.id == 7{//canned food
            shop.canned_food_cnt += 1;
            for mut friend in &mut shop.for_sale{
                friend.attack += 1;
                friend.health += 1;
            }
        }
        else if self.id == 8{//pear
            friendly_friends[idx].attack += 2;
            friendly_friends[idx].health += 2;
        }
        else if self.id == 9{//chili
            friendly_friends[idx].food_id = self.id;
        }
        else if self.id == 10{//chocolate
            friendly_friends[idx].xp += 1;
            //need to check if lvlup effect happens
        }
        else if self.id == 11{//sushi
            if friendly_friends.len() < 3{
                if friendly_friends.len() == 2{
                    friendly_friends[0].attack += 1;
                    friendly_friends[0].health += 1;
                    friendly_friends[1].attack += 1;
                    friendly_friends[1].health += 1;
                }
                else if friendly_friends.len() == 1{
                    friendly_friends[0].attack += 1;
                    friendly_friends[0].health += 1;
                }
                return
            }
            let mut rng = thread_rng();
            let rand_nums = rand::seq::index::sample(&mut rng, friendly_friends.len(), 3).into_vec();
            for i in rand_nums.iter().map(|&x| x as usize){
                friendly_friends[i].attack += 1;
                friendly_friends[i].health += 1;
            }
        }
        else if self.id == 12{//melon
            friendly_friends[idx].food_id = self.id;
        }
        else if self.id == 13{//mushroom
            friendly_friends[idx].food_id = self.id;
        }
        else if self.id == 14{//pizza
            if friendly_friends.len() < 2{
                if friendly_friends.len() == 1{
                    friendly_friends[0].attack += 2;
                    friendly_friends[0].health += 2;
                }
                return
            }
            let mut rng = thread_rng();
            let rand_nums = rand::seq::index::sample(&mut rng, friendly_friends.len(), 2).into_vec();
            for i in rand_nums.iter().map(|&x| x as usize){
                friendly_friends[i].attack += 2;
                friendly_friends[i].health += 2;
            }
        }
        else if self.id == 15{//steak
            friendly_friends[idx].food_id = self.id;
        }
    }
}