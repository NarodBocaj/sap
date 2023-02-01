mod friends;


fn main() {
    let mut friendly_friends = vec![];
    let mut lives: i32 = 5;
    let mut trophies: i32 = 0;

    let mut ant = friends::friend_maker(friends::shop::ANT);

    friendly_friends.push(ant);    

    // let dead_pet: friends::Friend = (friendly_friends.pop()).unwrap();

    // dead_pet.faint(&mut friendly_friends, 3);

    
    print_friends(&friendly_friends);
}

fn print_friends(friendly_friends: & Vec<friends::Friend>) -> (){
    for i in 0..friendly_friends.len(){
        //id attack/health
        println!("{} {}/{}", friendly_friends[i].id, friendly_friends[i].attack, friendly_friends[i].health);
    }
}

fn battle(friendly_friends: &mut Vec<friends::Friend>, enemy_friends: &mut Vec<friends::Friend>, trophies: &mut i32, lives: &mut i32) -> (){
    //need to figure out how to deep copy these vecs

    while friendly_friends.len() > 0 && enemy_friends.len() > 0{
        //make them battle
        let my_attack = friendly_friends[0].attack;
        let opp_attack = enemy_friends[0].attack;
        
        friendly_friends[0].health -= opp_attack;
        enemy_friends[0].health -= my_attack;
        
        if friendly_friends[0].health < 1{
            let my_faint = (friendly_friends.pop()).unwrap();
            my_faint.faint(friendly_friends, 0); //idx placeholder for right now
        }
        else{
            //run the .hurt ability
        }

        if enemy_friends[0].health < 1{
            let opp_faint = (enemy_friends.pop()).unwrap();
            opp_faint.faint(enemy_friends, 0); //idx placeholder for right now
        }
        else{
            //run the .hurt ability
        }

    }

    if friendly_friends.len() > 0{
        *trophies += 1;
    }
    else if friendly_friends.len() == 0 && enemy_friends.len() > 0{
        *lives -= 1; //situation where both vecs have len == 0 is a tie and nothing happens
    }
}