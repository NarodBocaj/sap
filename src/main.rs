mod friends;


fn main() {
    let mut friendly_friends = vec![];
    let mut enemy_friends = vec![];
    let mut lives: i32 = 5;
    let mut trophies: i32 = 0;

    //testing
    let mut ant1 = friends::friend_maker(friends::shop::ANT);
    let mut ant2 = friends::friend_maker(friends::shop::ANT);
    let mut ant3 = friends::friend_maker(friends::shop::ANT);
    let mut ant = friends::friend_maker(friends::shop::ANT);
    let mut duck = friends::friend_maker(friends::shop::DUCK);
    let mut flamingo = friends::friend_maker(friends::shop::FLAMINGO);
    let combo_ant = ant1 + ant2;
    let combo_ant = combo_ant + ant3;
    friendly_friends.push(flamingo);
    friendly_friends.push(combo_ant);
    //friendly_friends.push(antant);
    enemy_friends.push(ant);
    enemy_friends.push(duck);
    // let dead_pet: friends::Friend = (friendly_friends.pop()).unwrap();
    // dead_pet.faint(&mut friendly_friends, 3);
    
    let mut shop = friends::shop::Shop{
        turn_num: 3,
        frozen: Vec::new(),
        for_sale: Vec::new(),
        lvl_up: Vec::new(),
    };
    shop.roll();
    shop.freeze(2);
    print_shop(&shop);
    shop.roll();
    print_shop(&shop);
    shop.buy(&mut friendly_friends, 0);

    let mut my_friends_copy = friendly_friends.clone();
    let mut opp_friends_copy = enemy_friends.clone();

    battle(&mut my_friends_copy, &mut opp_friends_copy, &mut trophies, &mut lives);




    //print_friends(&friendly_friends);
}


fn battle(my_friends: &mut Vec<friends::Friend>, opp_friends: &mut Vec<friends::Friend>, trophies: &mut i32, lives: &mut i32) -> (){
    //need to figure out how to deep copy these vecs

    while my_friends.len() > 0 && opp_friends.len() > 0{
        //make them battle
        //print_friends(my_friends);
        //print_enemies(opp_friends);
        println!("{}", print_battle_state(my_friends, opp_friends));

        let my_attack = my_friends[0].attack;
        let opp_attack = opp_friends[0].attack;

        do_dmg(my_friends, opp_attack, 0);
        do_dmg(opp_friends, my_attack, 0);
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