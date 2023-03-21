// extern crate cpython;
// use cpython::{Python, PyResult};

use pyo3::prelude::*;


pub mod friends;


pub fn battle(my_friends: &mut Vec<friends::Friend>, opp_friends: &mut Vec<friends::Friend>, trophies: &mut i32, lives: &mut i32) -> (){
    //run start of battle ability for all pets

    while my_friends.len() > 0 && opp_friends.len() > 0{
        //make them battle
        //print_friends(my_friends);
        //print_enemies(opp_friends);
        println!("{}", print_battle_state(my_friends, opp_friends));

        let my_attack = my_friends[0].attack;
        let opp_attack = opp_friends[0].attack;
        
        //need to know food situation too 
        do_dmg(my_friends, opp_attack, 0);//my team recieving dmg | should call appropriate friend ahead fns
        do_dmg(opp_friends, my_attack, 0);//opps team recieving dmg | should call appropriate friend ahead fns
        //I believe some kind of hurt queue is prudent here
    }
    println!("Final Team State");
    println!("{}", print_battle_state(my_friends, opp_friends));

    if my_friends.len() > 0{
        *trophies += 1;
        println!("We won!");
    }
    else if my_friends.len() == 0 && opp_friends.len() == 0{
        *lives -= 1; //situation where both vecs have len == 0 is a tie and nothing happens
        println!("We tied!");
    }
    else{
        println!("You lost you fucking loser....what's wrongs with you, why can't you do anything right");
    }
}

fn do_dmg(team: &mut Vec<friends::Friend>, dmg: i32, idx: usize) -> (){
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

pub fn print_friends(friendly_friends: & Vec<friends::Friend>) -> (){
    println!("Printing current my team: id attack/health");
    for i in 0..friendly_friends.len(){
        //id attack/health
        println!("{} {}/{}", friendly_friends[i].id, friendly_friends[i].attack, friendly_friends[i].health);
    }
}

fn print_battle_state(my_friends: & Vec<friends::Friend>, opp_friends: & Vec<friends::Friend>) -> String{
    //println!("Printing Battle State");
    let mut battle_string: String = "".to_string();

    let my_friends_len = my_friends.len();
    let opp_friends_len = opp_friends.len();

    for i in 0..my_friends_len{
        let name = friends::shop::PETS[(my_friends[(my_friends_len - 1 - i) as usize].id) as usize].3;
        let attack = my_friends[(my_friends_len - 1 - i) as usize].attack;
        let health = my_friends[(my_friends_len - 1 - i) as usize].health;
        let mut comma = ", ";
        if i == my_friends_len - 1{
            comma = "";
        }
        battle_string += &format!("{}({}/{}){}", name, attack, health, comma);
    }

    battle_string += &" ||| ".to_string(); 
    
    for i in 0..opp_friends_len{
        let name = friends::shop::PETS[(opp_friends[i as usize].id) as usize].3;
        let attack = opp_friends[i as usize].attack;
        let health = opp_friends[i as usize].health;
        let mut comma = ", ";
        if i == opp_friends_len - 1{
            comma = "";
        }
        battle_string += &format!("{}({}/{}){}", name, attack, health, comma);
    }
    return battle_string
}

pub fn print_shop(shop: &friends::shop::Shop) -> (){
    let mut shop_string: String = "".to_string();
    for friend in &shop.for_sale{
        let name = friends::shop::PETS[(friend.id) as usize].3;
        let attack = friend.attack;
        let health = friend.health;
        shop_string += &format!("{}({}/{}) | ", name, attack, health);
    }
    println!("For Sale: {}", shop_string);
    
    let mut frozen_string: String = "".to_string();
    for friend in &shop.frozen{
        let name = friends::shop::PETS[(friend.id) as usize].3;
        let attack = friend.attack;
        let health = friend.health;
        frozen_string += &format!("{}({}/{}) | ", name, attack, health);
    }
    println!("Frozen: {}", frozen_string);
}

pub struct Game{
    pub wins: i32,
    pub lives: i32,
    pub turnnum: i32,
    pub money: i32,
    pub friendly_friends: Vec<friends::Friend>,
    pub shop: friends::shop::Shop,
    pub lost_lst_rnd: bool,
    pub actions_remaining: i32,
}

impl Game{
    //List of fns for Game
    //Function to Get game state, this function should only have a python output
    //Funciton to give all the possible actions to python
    //Function to do the action (roll shop, freeze shop, buy from shop, sell from pets, move pets, combine pets)
    //Function to go to battle
        //battle consists of the following
        //Getting team on same turn cloning games team and running the battle function
        //giving the result to python for reward for the RL
        //incrementing turn number
        //rolling the shop
        //runs fucntions 1 and 2 again
    pub fn new() -> Self{
        Game {
            wins: 0,
            lives: 5,
            turnnum: 1,
            money: 10,
            friendly_friends: Vec::new(),
            shop: friends::shop::Shop{
                turn_num: 1,//remove this when game fn works, all shop fucntions should use game.turnnum
                frozen: Vec::new(),
                for_sale: Vec::new(),
                lvl_up: Vec::new(),
                food: Vec::new(),
                frozen_food: Vec::new(),
                canned_food_cnt: 0,
            },
            lost_lst_rnd: false,
            actions_remaining: 50,
        }
    }

    pub fn game_state(&self) -> Vec<i32> {
        let mut state_vec: Vec<i32> = Vec::new();
        for i in 0..5{//getting all of the team info
            if i < self.friendly_friends.len(){
                let idx = i as usize;
                let temp_pet = self.friendly_friends[idx];
                state_vec.push(temp_pet.id);
                state_vec.push(temp_pet.attack);
                state_vec.push(temp_pet.health);
                state_vec.push(temp_pet.xp);
                state_vec.push(temp_pet.food_id);
            }
            else{
                for _ in 0..5{
                    state_vec.push(-64);
                }
            }
        }
        for friend in &self.shop.for_sale{//adding all the non-frozen pets
            state_vec.push(friend.id);
            state_vec.push(friend.attack);
            state_vec.push(friend.health);
            state_vec.push(friend.tier);
            state_vec.push(0);//frozen status
        }
        for friend in &self.shop.frozen{//adding all the frozen pets
            state_vec.push(friend.id);
            state_vec.push(friend.attack);
            state_vec.push(friend.health);
            state_vec.push(friend.tier);
            state_vec.push(1);//frozen status
        }
        for _ in (self.shop.frozen.len() + self.shop.for_sale.len())..6{//adding fillers for missing shop slots
            for x in 0..5{
                state_vec.push(1024);
            }
        }
        for fud in &self.shop.food{//adding non-frozen food
            state_vec.push(fud.id);
            state_vec.push(0);//frozen status
        }
        for fud in &self.shop.frozen_food{//adding frozen food
            state_vec.push(fud.id);
            state_vec.push(1);//frozen status
        }
        if self.lost_lst_rnd{
            state_vec.push(1);
        }
        else{
            state_vec.push(0);
        }
        state_vec.push(self.shop.canned_food_cnt);
        state_vec.push(self.actions_remaining);
        return state_vec
    }
    

    pub fn game_options(&self) -> Vec<Vec<i32>> {//***NOTE, need to add restriction if out of actions to only allow go to battle
        //need codes for all different options
        
        //format will be [code for option, idx 1 affected (-1 if none), idx 2 affected (-1 if none)]

        //-1 will be sell
        //1 will be buy to open slot
        //2 will be buy to combine with certain index
        //3 will be buy food to certain index
        //4 will be freeze shop pet at index
        //5 will be freeze shop food at index
        //6 will be roll
        //7 will be swap pets and will take two indices (it will only be allowed to swap adjacent pets)
        //8 will be combine team pets together
        //9 will be go to battle and will only be allowed when money is out and will be forced when remaining actions gets to 0
        let mut opts_vec = Vec::new();

        if self.actions_remaining > 0{
            for idx in 0..self.friendly_friends.len(){//things that can be sold -1
                opts_vec.push(vec![-1, idx as i32, -1]);
            }

            if self.money > 2{
                if self.friendly_friends.len() < 5{//things that can be bought to open slot 1
                    for idx in 0..self.shop.for_sale.len(){
                        opts_vec.push(vec![1, idx as i32, -1]);
                    }
                }

                for i in 0..self.shop.for_sale.len(){//things that can be bought to combine 2
                    for j in 0..self.friendly_friends.len(){
                        if self.shop.for_sale[i].id == self.friendly_friends[j].id{
                            opts_vec.push(vec![2, i as i32, j as i32]);
                        }
                    }
                }

                for i in 0..self.shop.food.len(){//food that can bought 3
                    for j in 0..self.friendly_friends.len(){
                        opts_vec.push(vec![3, j as i32, i as i32]);
                    }
                }
            }

            for idx in 0..self.shop.for_sale.len(){//pet that can be frozen 4
                opts_vec.push(vec![4, idx as i32, -1]);
            }

            for idx in 0..self.shop.food.len(){//food that can be frozen 5
                opts_vec.push(vec![5, idx as i32, -1]);
            }

            if self.money > 0{//roll the shop 6
                opts_vec.push(vec![6, -1 , -1]);
            }

            for idx in 0..self.friendly_friends.len() - 1{//swap pets 7
                opts_vec.push(vec![7, idx as i32, (idx + 1) as i32]);
            }

            for i in 0..self.friendly_friends.len() - 1{//combine pets on team 8
                for j in i + 1..self.friendly_friends.len(){
                    if self.friendly_friends[i].id == self.friendly_friends[j].id && self.friendly_friends[i].xp + self.friendly_friends[j].xp <= 6{//pets can only be combined if they add up to lvl 3 or less    
                        opts_vec.push(vec![8, i as i32, j as i32]);//combine i on j
                        opts_vec.push(vec![8, j as i32, i as i32]);//combine j on i
                    }
                }
            }
        }

        if self.money == 0 || self.actions_remaining == 0{//go to battle
            opts_vec.push(vec![9, -1 , -1]);
        }

        return opts_vec
    }

    //need function that takes action choice from python then executes it

}


//testing pyo3
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello, world!".to_string())
}

#[pymodule]
fn libsap(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;

    Ok(())
}