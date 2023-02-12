use crate::friends::Friend;
use crate::friends::friend_maker;
use crate::friends::Food;
use rand::Rng;
//counts
//tier 1: 9     idxs: 0-8
//tier 2: 10    idxs: 9-18
//tier 3: 11    idxs: 19-29
//tier 4: 11    idxs: 30-40
//tier 5: 8     idxs: 41-48
//tier 6: 9     idxs: 49-57
//total 58
//need different id's for summons, zombie cricket is currently 100
pub const ANT: i32 = 0; //tier 1
pub const BEAVER: i32 = 1; //tier 1
pub const CRICKET: i32 = 2; //tier 1
pub const DUCK: i32 = 3; //tier 1
pub const FISH: i32 = 4; //tier 1
pub const HORSE: i32 = 5; //tier 1
pub const MOSQUITO: i32 = 6; //tier 1
pub const OTTER: i32 = 7; //tier 1
pub const PIG: i32 = 8; //tier 1
pub const FLAMINGO: i32 = 9; //tier 2
pub const CRAB: i32 = 10; //tier 2
pub const DODO: i32 = 11; //tier 2
pub const ELEPHANT: i32 = 12; //tier 2
pub const HEDGEHOG: i32 = 13; //tier 2
pub const PEACOCK: i32 = 14; //tier 2
pub const RAT: i32 = 15; //tier 2
pub const SHRIMP: i32 = 16; //tier 2
pub const SPIDER: i32 = 17; //tier 2
pub const SWAN: i32 = 18; //tier 2
pub const BADGER: i32 = 19; //tier 3
pub const BLOWFISH: i32 = 20; //tier 3
pub const CAMEL: i32 = 21; //tier 3
pub const DOG: i32 = 22; //tier 3
pub const DOLPHIN: i32 = 23; //tier 3
pub const GIRAFFE: i32 = 24; //tier 3
pub const KANGAROO: i32 = 25; //tier 3
pub const OX: i32 = 26; //tier 3
pub const RABBIT: i32 = 27; //tier 3
pub const SHEEP: i32 = 28; //tier 3
pub const SNAIL: i32 = 29; //tier 3

pub const PETS: [(i32, i32, i32, &str); 30] = [
    //id, attack, health, name
    (ANT, 2, 1, "ant"),
    (BEAVER, 3, 2, "beaver"),
    (CRICKET, 1, 2, "cricket"),
    (DUCK, 2, 3, "duck"),
    (FISH, 2, 2, "fish"),
    (HORSE, 2, 1, "horse"),
    (MOSQUITO, 2, 2, "mosquito"),
    (OTTER, 1, 2, "otter"),
    (PIG, 4, 1, "pig"),
    (FLAMINGO, 4, 2, "flamingo"),
    (CRAB, 3, 1, "crab"),
    (DODO, 3, 3, "dodo"),
    (ELEPHANT, 3, 5, "elephant"),
    (HEDGEHOG, 3, 2, "hedgehog"),
    (PEACOCK, 2, 5, "peacock"),
    (RAT, 4, 5, "rat"),
    (SHRIMP, 2, 3, "shrimp"),
    (SPIDER, 2, 2, "spider"),
    (SWAN, 1, 3, "swan"),
    (BADGER, 5, 3, "badger"),
    (BLOWFISH, 3, 5, "blowfish"),
    (CAMEL, 2, 6, "camel"),
    (DOG, 3, 4, "dog"),
    (DOLPHIN, 4, 3, "dolphin"),
    (GIRAFFE, 1, 3, "giraffe"),
    (KANGAROO, 1, 2, "kangaroo"),
    (OX, 1, 3, "ox"),
    (RABBIT, 1, 2, "rabbit"),
    (SHEEP, 2, 2, "sheep"),
    (SNAIL, 2, 2, "snail"),
];

pub struct Shop{
    pub turn_num: i32,
    pub frozen: Vec<Friend>,
    pub for_sale: Vec<Friend>,
    pub lvl_up: Vec<Friend>,//place holder for the lvlup logic of higher tier pet in shop
    pub food: Vec<Food>,
    pub frozen_food: Vec<Food>,
}

impl Shop{
    pub fn roll(&mut self) -> (){
        //beginning of every turn should begin with turn_num += 1
        //then a roll
        
        //emptying sale vec
        self.for_sale.clear();
        
        //filling it with frozen pets from last round
        for friend in &self.frozen{
            self.for_sale.push(*friend);
        }
        //empyting frozen vec
        self.frozen.clear();
        //getting turn info
        let (shop_size, shop_range) = turn_num_to_shopcnt_shoprng(self.turn_num);
        //filling remaining shop
        for _ in self.for_sale.len()..shop_size as usize{
            let rand_num = rand::thread_rng().gen_range(0..shop_range);
            let mut created_friend = friend_maker(rand_num);
            self.for_sale.push(created_friend);
        }
    }

    pub fn freeze(&mut self, idx: usize) -> (){
        let mut new_frozen_friend = self.for_sale.remove(idx);
        self.frozen.push(new_frozen_friend);
    }

    pub fn buy(&mut self, team: &mut Vec<Friend>, idx:usize) -> (){
        let mut bought_friend = self.for_sale.remove(idx);
        team.push(bought_friend);
    }
}

pub fn turn_num_to_shopcnt_shoprng(turn_num: i32) -> (i32, i32){
    //returning (how many animals are the shop, max animal int in the range)
    //game will start on turn 1
    //will return number for range 0..current max shop int
    //non-inclusive
    if turn_num < 3{
        return (3, 9)
    }
    else if turn_num < 5{
        return (3, 19)
    }
    else if turn_num < 7{
        return (4, 30)
    }
    else if turn_num < 9{
        return (4, 41)
    }
    else if turn_num < 11{
        return (5, 49)
    }
    else{
        return (5, 58)
    }
}

pub const APPLE: i32 = 0;   //tier 1
pub const HONEY: i32 = 1;   //tier 1
pub const PILL: i32 = 2;    //tier 2
pub const CUPCAKE: i32 = 3; //tier 2
pub const MEATBONE: i32 = 4;//tier 2
pub const SALAD: i32 = 5;   //tier 3
pub const GARLIC: i32 = 6;   //tier 3
pub const CANNEDFOOD: i32 = 7;//tier 4
pub const PEAR: i32 = 8;    //tier 4
pub const CHILI: i32 = 9;   //tier 5
pub const CHOCOLATE: i32 = 10;//tier 5
pub const SUSHI: i32 = 11;  //tier 5
pub const MELON: i32 = 12;  //tier 6
pub const MUSHROOM: i32 = 13;//tier 6
pub const PIZZA: i32 = 14;  //tier 6
pub const STEAK: i32 = 15;  //tier 6