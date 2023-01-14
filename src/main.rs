mod friends;

fn main() {
    let mut friendly_friends = vec![];

    let ant = friends::Friend{
        attack: 2,
        health: 1,
        name: "ant".to_string(),
        tier: 1,
        xp: 0,
    };
    let ant1 = friends::Friend{
        attack: 2,
        health: 1,
        name: "ant".to_string(),
        tier: 1,
        xp: 0,
    };

    friendly_friends.push(ant);
    friendly_friends.push(ant1);

    let dead_pet = friendly_friends.pop();

    friendly_friends = (dead_pet.faint(&mut friendly_friends)).to_vec();

    
    println!("Ant's health is: {}", friendly_friends[0].health);
}