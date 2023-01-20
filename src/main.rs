mod friends;
//mod animals;

fn main() {
    let mut friendly_friends = vec![];

    let ant = friends::Friend{
        attack: 2,
        health: 1,
        name: "ant".to_string(),
        tier: 1,
        xp: 1,
    };
    let ant1 = friends::Friend{
        attack: 2,
        health: 1,
        name: "ant".to_string(),
        tier: 1,
        xp: 3,
    };
    let flamingo = friends::Friend{
        attack: 4,
        health: 2,
        name: "flamingo".to_string(),
        tier: 2,
        xp: 6,
    };
    let cricket = friends::Friend{
        attack: 1,
        health: 2,
        name: "cricket".to_string(),
        tier: 1,
        xp: 1,
    };
    let otter = friends::Friend{
        attack: 1,
        health: 2,
        name: "otter".to_string(),
        tier: 1,
        xp: 1,
    };

    friendly_friends.push(ant);
    friendly_friends.push(ant1);
    friendly_friends.push(flamingo);
    friendly_friends.push(cricket);

    //let mut friend1 = friends::ant100;

    let dead_pet: friends::Friend = (friendly_friends.pop()).unwrap();

    dead_pet.faint(&mut friendly_friends, 3);

    
    print_friends(&friendly_friends);

    otter.buy(&mut friendly_friends, 3);

    print_friends(&friendly_friends);
}

fn print_friends(friendly_friends: & Vec<friends::Friend>) -> (){
    for i in 0..friendly_friends.len(){
        println!("{} {}/{}", friendly_friends[i].name, friendly_friends[i].attack, friendly_friends[i].health);
    }
}