use cpython::{Python, PyResult};

mod friends;


fn main() {
    //let mut friendly_friends = vec![];
    let mut enemy_friends = vec![];
    let mut lives: i32 = 5;
    let mut trophies: i32 = 0;

    let mut game = Game{
        wins: 0,
        lives: 5,
        turnnum: 1,
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
    };

    //testing
    let mut ant1 = friends::friend_maker(friends::shop::ANT, 0);
    let mut ant2 = friends::friend_maker(friends::shop::ANT, 0);
    let mut ant3 = friends::friend_maker(friends::shop::ANT, 0);
    let mut ant = friends::friend_maker(friends::shop::ANT, 0);
    let mut duck = friends::friend_maker(friends::shop::DUCK, 0);
    let mut flamingo = friends::friend_maker(friends::shop::FLAMINGO, 0);
    let combo_ant = ant1 + ant2;
    let combo_ant = combo_ant + ant3;
    game.friendly_friends.push(flamingo);
    game.friendly_friends.push(combo_ant);
    //friendly_friends.push(antant);
    enemy_friends.push(ant);
    enemy_friends.push(duck);

    
    let mut shop = friends::shop::Shop{
        turn_num: 3,
        frozen: Vec::new(),
        for_sale: Vec::new(),
        lvl_up: Vec::new(),
        food: Vec::new(),
        frozen_food: Vec::new(),
        canned_food_cnt: 0,
    };
    
    game.shop.roll();
    print_shop(&game.shop);

    println!("Printing the Game State");
    println!("{:?}", game.game_state());

    let mut my_friends_copy = game.friendly_friends.clone();
    let mut opp_friends_copy = enemy_friends.clone();

    battle(&mut my_friends_copy, &mut opp_friends_copy, &mut trophies, &mut lives);

    //print_friends(&friendly_friends);
}


fn battle(my_friends: &mut Vec<friends::Friend>, opp_friends: &mut Vec<friends::Friend>, trophies: &mut i32, lives: &mut i32) -> (){
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

fn print_friends(friendly_friends: & Vec<friends::Friend>) -> (){
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

fn print_shop(shop: &friends::shop::Shop) -> (){
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
    
}