use sap::*; 

fn main() {
    let mut enemy_friends = vec![];
    let mut lives: i32 = 5;
    let mut trophies: i32 = 0;

    let mut game = Game::new();

    //testing
    let mut ant1 = sap::friends::friend_maker(sap::friends::shop::ANT, 0);
    let mut ant2 = sap::friends::friend_maker(sap::friends::shop::ANT, 0);
    let mut ant3 = sap::friends::friend_maker(sap::friends::shop::ANT, 0);
    let mut ant = sap::friends::friend_maker(sap::friends::shop::ANT, 0);
    let mut duck = sap::friends::friend_maker(sap::friends::shop::DUCK, 0);
    let mut flamingo = sap::friends::friend_maker(sap::friends::shop::FLAMINGO, 0);
    let combo_ant = ant1 + ant2;
    let combo_ant = combo_ant + ant3;
    game.friendly_friends.push(flamingo);
    game.friendly_friends.push(combo_ant);
    //friendly_friends.push(antant);
    enemy_friends.push(ant);
    enemy_friends.push(duck);

    
    game.shop.roll();
    sap::print_shop(&game.shop);

    println!("Printing the Game State");
    println!("{:?}", game.game_state());
    println!("Printing Options Vec");
    println!("{:?}", game.game_options());

    let mut my_friends_copy = game.friendly_friends.clone();
    let mut opp_friends_copy = enemy_friends.clone();

    sap::battle(&mut my_friends_copy, &mut opp_friends_copy, &mut trophies, &mut lives);

    //print_friends(&friendly_friends);
    println!("Checking shop pet ids");
    sap::friends::shop::check_pet_ids();
}