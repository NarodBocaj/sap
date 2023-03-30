// extern crate cpython;
// use cpython::{Python, PyResult};

use pyo3::prelude::*;
use pyo3::types::PyList;

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
        friends::do_dmg(my_friends, opp_attack, 0);//my team recieving dmg | should call appropriate friend ahead fns
        friends::do_dmg(opp_friends, my_attack, 0);//opps team recieving dmg | should call appropriate friend ahead fns
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

#[pyclass]
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

#[pymethods]
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
    #[new]
    pub fn new() -> Self{
        println!("Creating Game Instance");
        Game {
            wins: 0,
            lives: 5,
            turnnum: 1,
            money: 10,
            friendly_friends: Vec::new(),
            shop: friends::shop::Shop::new(),
            lost_lst_rnd: false,//This needs to only go back to true when a win happens
            actions_remaining: 50,
        }
    }

    pub fn game_state(&self) -> PyResult<Vec<i32>> {
        println!("Game State");
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
        for _ in (self.shop.frozen_food.len() + self.shop.food.len())..2{//adding fillers for missing food shop slots
            for x in 0..2{
                state_vec.push(1024);
            }
        }
        if self.lost_lst_rnd{
            state_vec.push(1);
        }
        else{
            state_vec.push(0);
        }
        state_vec.push(self.shop.canned_food_cnt);
        state_vec.push(self.actions_remaining);
        state_vec.push(self.money);
        state_vec.push(self.wins);
        state_vec.push(self.lives);
        state_vec.push(self.turnnum);
        Ok(state_vec)
    }
    

    pub fn game_options(&self) -> PyResult<Vec<Vec<i32>>> {//***NOTE, need to add restriction if out of actions to only allow go to battle
        //need codes for all different options
        
        //format will be [code for option, idx 1 affected (-1 if none), idx 2 affected (-1 if none)]

        //-1 will be sell (max 5 sell actions)
        //1 will be buy to open slot (max 6 actions here)
        //2 will be buy to combine with certain index (max 30 actions here)
        //3 will be buy food to certain index (max 10 actions)
        //4 will be freeze shop pet at index (max 6 actions)
        //5 will be freeze shop food at index (max 2 actions)
        //6 will be roll (1 action)
        //7 will be swap pets and will take two indices (it will only be allowed to swap adjacent pets, max 4 actions)
        //8 will be combine team pets together (8 + 6 + 4 + 2, max 20 actions)
        //9 will be go to battle and will only be allowed when money is out and will be forced when remaining actions gets to 0 (max 1 action)
        //total possible actions 85
        let mut opts_vec = Vec::new();
 
        if self.actions_remaining > 0{
            for idx in 0..self.friendly_friends.len(){//things that can be sold -1
                opts_vec.push(vec![-1, idx as i32, -1]);
            }

            for _ in self.friendly_friends.len()..5{//*empty option for pets that can't be sold
                opts_vec.push(vec![]);
            }
   
            if self.money > 2{
                if self.friendly_friends.len() < 5{//things that can be bought to open slot 1
                    for idx in 0..self.shop.for_sale.len(){
                        opts_vec.push(vec![1, idx as i32, -1]);
                    }

                    for _ in self.shop.for_sale.len()..6{//**empty options for buying to open slot when shop isnt of len 6
                        opts_vec.push(vec![]);
                    }

                }

                else{//**empty options for buying to open slot
                    for _ in 0..6{
                        opts_vec.push(vec![]);
                    }
                }

                // for i in 0..self.shop.for_sale.len(){//old method before constant indices in the opts_vec
                //     for j in 0..self.friendly_friends.len(){
                //         if self.shop.for_sale[i].id == self.friendly_friends[j].id{
                //             opts_vec.push(vec![2, i as i32, j as i32]);
                //         }
                //     }
                // }

                for i in 0..6{//things that can be bought to combine 2
                    for j in 0..5{
                        if i < self.shop.for_sale.len() && j < self.friendly_friends.len(){
                            if self.shop.for_sale[i].id == self.friendly_friends[j].id{
                                opts_vec.push(vec![2, i as i32, j as i32]);
                            }
                            else{//**empty option if ID doesn't match
                                opts_vec.push(vec![]);
                            }
                        }
                        else{//**empty option if outside of shop or team range
                            opts_vec.push(vec![]);
                        }
                    }
                }

                for i in 0..2{//food that can bought 3
                    for j in 0..5{
                        if i < self.shop.food.len() && j < self.friendly_friends.len(){
                            if self.shop.food[i].id == 10 && self.friendly_friends[j].xp >= 6{//**push empty if chocolate cannot be bought
                                opts_vec.push(vec![]);
                            }
                            else{
                                opts_vec.push(vec![3, j as i32, i as i32]);
                            }
                        }
                        else{//**empty options for out of food or team range
                            opts_vec.push(vec![]);
                        }
                    }
                }
            }

            else{//**empty options for when money is too low for options 1, 2, and 3
                for _ in 0..46{
                    opts_vec.push(vec![]);
                }
            }

            for idx in 0..6{//pet that can be frozen 4
                if idx < self.shop.for_sale.len(){
                    opts_vec.push(vec![4, idx as i32, -1]);
                }
                else{//**empty option if outside shop range
                    opts_vec.push(vec![]);
                }
            }

            for idx in 0..2{//food that can be frozen 5
                if idx < self.shop.food.len(){
                    opts_vec.push(vec![5, idx as i32, -1]);
                }
                else{//**empty option for out of food range
                    opts_vec.push(vec![]);
                }
            }

            if self.money > 0{//roll the shop 6
                opts_vec.push(vec![6, -1 , -1]);
            }

            else{//**empty option for when shop can't be rolled
                opts_vec.push(vec![]);
            }

            for idx in 0..4{//swap pets 7
                if idx + 1 < self.friendly_friends.len(){
                    opts_vec.push(vec![7, idx as i32, (idx + 1) as i32]);
                }
                else{//**empty option for when out of team range
                    opts_vec.push(vec![]);
                }
            }

            for i in 0..4{//combine pets on team 8
                for j in i + 1..5{
                    if j < self.friendly_friends.len() {
                        if self.friendly_friends[i].id == self.friendly_friends[j].id && self.friendly_friends[i].xp + self.friendly_friends[j].xp <= 6{//pets can only be combined if they add up to lvl 3 or less    
                            opts_vec.push(vec![8, i as i32, j as i32]);//combine i on j
                            opts_vec.push(vec![8, j as i32, i as i32]);//combine j on i
                        }
                        else{//empty options for when pet ids don't match
                            opts_vec.push(vec![]);
                            opts_vec.push(vec![]);
                        }
                    }
                    else{//empty options for when out of team range
                        opts_vec.push(vec![]);
                        opts_vec.push(vec![]);
                    }
                }
            }

        }

        else{//**push empty options
            for _ in 0..84{
                opts_vec.push(vec![]);
            }
        }

        if self.money == 0 || self.actions_remaining == 0{//go to battle
            opts_vec.push(vec![9, -1 , -1]);
        }

        else{//**push empty actions for no going to battle
            opts_vec.push(vec![]);
        }

        println!("Game Options");
        Ok(opts_vec)
    }

    pub fn do_action(&mut self, option: &PyList) -> PyResult<f32>{//function should return a reward for the RL
        let opt: Vec<i32> = option.into_iter().map(|item| item.extract::<i32>()).collect::<Result<Vec<i32>, _>>()?;
        let mut reward: f32 = 0.0;
        if opt[0] == -1{//selling
            self.friendly_friends.remove(opt[1] as usize);
            self.money += 1;
            reward -= 2.0;
        }
        
        else if opt[0] == 1{//buying to open slot
            self.shop.buy(&mut self.friendly_friends, opt[1] as usize);
            self.money -= 3;
            self.actions_remaining -= 1;
            reward += 5.0;
        }

        else if opt[0] == 2{//buying to combine
            let mut combined_friend = self.friendly_friends[opt[2] as usize] + self.shop.for_sale[opt[1] as usize];
            
            self.friendly_friends.remove(opt[2] as usize);
            self.shop.for_sale.remove(opt[1] as usize);
            self.friendly_friends.push(combined_friend);

            self.money -= 3;
            self.actions_remaining -= 1;
            reward += 2.0;
        }

        else if opt[0] == 3{//buying food
            self.shop.buy_food(&mut self.friendly_friends, opt[2] as usize, opt[1] as usize);//food idx then pet idx
            self.money -= 3;
            self.actions_remaining -= 1;
            reward += 0.5;
        }

        else if opt[0] == 4{//freeze pet
            self.shop.freeze(opt[1] as usize);
            self.actions_remaining -= 1;
            reward -= 1.0;
        }

        else if opt[0] == 5{//freeze food
            self.shop.freeze_food(opt[1] as usize);
            self.actions_remaining -= 1;
            reward -= 1.0;
        }

        else if opt[0] == 6{//roll
            self.shop.roll();
            self.money -= 1;
            self.actions_remaining -= 1;
            reward += 0.1;
        }

        else if opt[0] == 7{//swap pets at two different indices
            self.friendly_friends.swap(opt[1] as usize, opt[2] as usize);
            self.actions_remaining -= 1;
            reward -= 1.0;
        }

        else if opt[0] == 8{//combine team pets together
            let idx1 = opt[1] as usize;
            let idx2 = opt[2] as usize; 
            let mut combined_pet = self.friendly_friends[idx1] + self.friendly_friends[idx2];

            let (remove_idx1, remove_idx2) = if idx1 > idx2 {
                (idx1, idx2)
            } else {
                (idx2, idx1)
            };

            self.friendly_friends.remove(remove_idx1);
            self.friendly_friends.remove(remove_idx2);

            self.friendly_friends.push(combined_pet);
            self.actions_remaining -= 1;
            reward += 0.1;
        }

        else if opt[0] == 9{//go to battle and reset the shop, gold, etc
            let mut opp_friends = Vec::new();
            let mut mos1 = friends::friend_maker(friends::shop::MOSQUITO, 0);
            let mut mos2 = friends::friend_maker(friends::shop::MOSQUITO, 0);
            opp_friends.push(mos1);
            opp_friends.push(mos2);
            test_battle(self, &mut opp_friends, &mut reward);
            self.turnnum += 1;
            self.shop.turn_num += 1;
            self.money = 10;
            self.actions_remaining = 50;
            self.shop.roll();
        }

        Ok(reward)//place holder for rewward function
    }

    pub fn game_alive(&self) -> PyResult<bool>{
        if self.wins < 10 && self.lives > 0{
            println!("Wins = {}, Lives = {}", self.wins, self.lives);
            Ok(true)
        }
        else{
            Ok(false)
        }
    }

    pub fn gen_game(&self) -> PyResult<Vec<i32>>{
        let mut res = vec![];
        res.push(self.wins);
        res.push(self.lives);
        Ok(res)
    }

}

pub fn test_battle(game: &mut Game, opp_friends: &mut Vec<friends::Friend>, reward: &mut f32) -> (){
    //run start of battle ability for all pets
    let mut my_friends = game.friendly_friends.clone();

    while my_friends.len() > 0 && opp_friends.len() > 0{
        //make them battle
        //print_friends(my_friends);
        //print_enemies(opp_friends);
        println!("{}", print_battle_state(&my_friends, opp_friends));

        let my_attack = my_friends[0].attack;
        let opp_attack = opp_friends[0].attack;
        
        //need to know food situation too 
        friends::do_dmg(&mut my_friends, opp_attack, 0);//my team recieving dmg | should call appropriate friend ahead fns
        friends::do_dmg(opp_friends, my_attack, 0);//opps team recieving dmg | should call appropriate friend ahead fns
        //I believe some kind of hurt queue is prudent here
    }
    println!("Final Team State");
    println!("{}", print_battle_state(&my_friends, opp_friends));

    if my_friends.len() > 0{
        game.wins += 1;
        *reward += 10.0 * game.wins as f32;//placeholder for reward
        println!("We won!");
    }
    else if my_friends.len() == 0 && opp_friends.len() == 0{ //situation where both vecs have len == 0 is a tie and nothing happens
        *reward -= 0.1;
        println!("We tied!");
    }
    else{
        game.lives -= 1;
        *reward -= 10.0 * (5.0 - game.lives as f32);//placeholder for reward
        println!("You lost you fucking loser....what's wrongs with you, why can't you do anything right");
    }
}

//testing pyo3
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello, world!".to_string())
}

#[pymodule]
fn libsap(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_class::<Game>()?;
    Ok(())
}