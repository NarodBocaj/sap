mod friends;


fn main() {
    let mut friendly_friends = vec![];
    let mut enemy_friends = vec![];
    let mut lives: i32 = 5;
    let mut trophies: i32 = 0;

    let mut ant = friends::friend_maker(friends::shop::ANT);
    let mut ant1 = friends::friend_maker(friends::shop::ANT);
    let mut duck = friends::friend_maker(friends::shop::DUCK);

    friendly_friends.push(ant);    
    enemy_friends.push(ant1);
    enemy_friends.push(duck);
    // let dead_pet: friends::Friend = (friendly_friends.pop()).unwrap();
    // dead_pet.faint(&mut friendly_friends, 3);


    let mut my_friends_copy = friendly_friends.clone();
    let mut opp_friends_copy = enemy_friends.clone();

    battle(&mut my_friends_copy, &mut opp_friends_copy, &mut trophies, &mut lives);

    //print_friends(&friendly_friends);
}

fn print_friends(friendly_friends: & Vec<friends::Friend>) -> (){
    println!("Printing current my team: id attack/health");
    for i in 0..friendly_friends.len(){
        //id attack/health
        println!("{} {}/{}", friendly_friends[i].id, friendly_friends[i].attack, friendly_friends[i].health);
    }
}

fn print_enemies(opp_friends: & Vec<friends::Friend>) -> (){
    println!("Printing current opp team: id attack/health");
    for i in 0..opp_friends.len(){
        //id attack/health
        println!("{} {}/{}", opp_friends[i].id, opp_friends[i].attack, opp_friends[i].health);
    }
}

fn battle(my_friends: &mut Vec<friends::Friend>, opp_friends: &mut Vec<friends::Friend>, trophies: &mut i32, lives: &mut i32) -> (){
    //need to figure out how to deep copy these vecs

    while my_friends.len() > 0 && opp_friends.len() > 0{
        //make them battle
        print_friends(my_friends);
        print_enemies(opp_friends);

        let my_attack = my_friends[0].attack;
        let opp_attack = opp_friends[0].attack;

        do_dmg(my_friends, opp_attack, 0);
        do_dmg(opp_friends, my_attack, 0);
    }
    println!("Final Team State");
    print_friends(my_friends);
    print_enemies(opp_friends);

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