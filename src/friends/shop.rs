use crate::friends::Friend;
use crate::friends::friend_maker;
use crate::friends::Food;
use rand::{thread_rng, Rng};
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
pub const BISON: i32 = 30; //tier 4
pub const DEER: i32 = 31; //tier 4
pub const HIPPO: i32 = 32; //tier 4
pub const PARROT: i32 = 33; //tier 4
pub const PENGUIN: i32 = 34; //tier 4
pub const ROOSTER: i32 = 35; //tier 4
pub const SKUNK: i32 = 36; //tier 4
pub const SQUIRREL: i32 = 37; //tier 4
pub const TURTLE: i32 = 38; //tier 4
pub const WHALE: i32 = 39; //tier 4
pub const WORM: i32 = 40; //tier 4
pub const COW: i32 = 41; //tier 5
pub const CROCODILE: i32 = 42; //tier 5
pub const MONKEY: i32 = 43; //tier 5
pub const RHINO: i32 = 44; //tier 5
pub const SCORPION: i32 = 45; //tier 5
pub const SEAL: i32 = 46; //tier 5
pub const SHARK: i32 = 47; //tier 5
pub const TURKEY: i32 = 48; //tier 5
pub const BOAR: i32 = 49; //tier 6
pub const CAT: i32 = 50; //tier 6
pub const DRAGON: i32 = 51; //tier 6
pub const FLY: i32 = 52; //tier 6
pub const GORILLA: i32 = 53; //tier 6
pub const LEOPARD: i32 = 54; //tier 6
pub const MAMMOTH: i32 = 55; //tier 6
pub const SNAKE: i32 = 56; //tier 6
pub const TIGER: i32 = 57; //tier 6


pub const PETS: [(i32, i32, i32, &str); 58] = [
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
    (BISON, 5, 3, "bison"),
    (DEER, 1, 1, "deer"),
    (HIPPO, 4, 5, "hippo"),
    (PARROT, 4, 2, "parrot"),
    (PENGUIN, 2, 4, "penguin"),
    (ROOSTER, 5, 3, "rooster"),
    (SKUNK, 3, 5, "skunk"),
    (SQUIRREL, 2, 5, "squirrel"),
    (TURTLE, 2, 5, "turtle"),
    (WHALE, 3, 8, "whale"),
    (WORM, 3, 3, "worm"),
    (COW, 4, 6, "cow"),
    (CROCODILE, 8, 4, "crocodile"),
    (MONKEY, 1, 2, "monkey"),
    (RHINO, 5, 8, "rhino"),
    (SCORPION, 1, 1, "scorpion"),
    (SEAL, 3, 8, "seal"),
    (SHARK, 4, 2, "shark"),
    (TURKEY, 3, 4, "turkey"),
    (BOAR, 10, 6, "boar"),
    (CAT, 4, 5, "cat"),
    (DRAGON, 6, 8, "dragon"),
    (FLY, 5, 5, "fly"),
    (GORILLA, 6, 9, "gorilla"),
    (LEOPARD, 10, 4, "leopard"),
    (MAMMOTH, 3, 10, "mammoth"),
    (SNAKE, 6, 6, "snake"),
    (TIGER, 4, 3, "tiger"),
];

pub struct Shop{
    pub turn_num: i32,
    pub frozen: Vec<Friend>,
    pub for_sale: Vec<Friend>,
    pub lvl_up: Vec<Friend>,//place holder for the lvlup logic of higher tier pet in shop
    pub food: Vec<Food>,
    pub frozen_food: Vec<Food>,
    pub canned_food_cnt: i32,
}

impl Shop{
    pub fn new() -> Self{
        let mut shop = Shop{
            turn_num: 1,//remove this when game fn works, all shop fucntions should use game.turnnum
            frozen: Vec::new(),
            for_sale: Vec::new(),
            lvl_up: Vec::new(),
            food: Vec::new(),
            frozen_food: Vec::new(),
            canned_food_cnt: 0,
        };
        shop.roll();
        return shop
    }

    pub fn roll(&mut self) -> (){
        //beginning of every turn should begin with turn_num += 1
        //then a roll
        
        //emptying sale vec and food vec
        self.for_sale.clear();
        self.food.clear();
        
        //filling it with frozen pets from last round
        for friend in &self.frozen{
            self.for_sale.push(*friend);
        }

        //filling it with frozen food from last round
        for fud in &self.frozen_food{
            self.food.push(*fud);
        }
        //empyting frozen vecs
        self.frozen.clear();
        self.frozen_food.clear();
        //getting turn info
        let (shop_size, shop_range, food_range) = turn_num_to_ranges(self.turn_num);
        //filling remaining shop with pets then food
        for _ in self.for_sale.len()..shop_size as usize{
            let rand_num = rand::thread_rng().gen_range(0..shop_range);
            let mut created_friend = friend_maker(rand_num, self.canned_food_cnt);
            self.for_sale.push(created_friend);
        }
        for _ in self.food.len()..2{
            let rand_num = rand::thread_rng().gen_range(0..food_range);
            let mut created_food = Food{
                id: rand_num,
            };
            self.food.push(created_food);
        }
    }

    pub fn freeze(&mut self, idx: usize) -> (){
        let mut new_frozen_friend = self.for_sale.remove(idx);
        self.frozen.push(new_frozen_friend);
    }

    //this is the buy friend function, should change the name
    pub fn buy(&mut self, team: &mut Vec<Friend>, idx:usize) -> (){
        let mut bought_friend = self.for_sale.remove(idx);
        team.push(bought_friend);
    }

    pub fn freeze_food(&mut self, idx: usize) -> (){
        let new_frozen_food = self.food.remove(idx);
        self.frozen_food.push(new_frozen_food);
    }

    pub fn buy_food(&mut self, friendly_friends: &mut Vec<Friend>, food_idx: usize, pet_idx: usize) -> () {
        //check all the food ids to give appropriate affect
        let food = self.food[food_idx];
        if food.id == 0{//apple
            friendly_friends[pet_idx].health += 1;
            friendly_friends[pet_idx].attack += 1;
        }
        else if food.id == 1{//honey
            //figure out how to handle honey
        }
        else if food.id == 2{//pill
            //faint pet, I think I should write fill faint function
        }
        else if food.id == 3{//cupcake
            //temp buff with cupcake
        }
        else if food.id == 4{//meatbone
            friendly_friends[pet_idx].food_id = food.id;
        }
        else if food.id == 5{//salad
            if friendly_friends.len() < 2{
                if friendly_friends.len() == 1{
                    friendly_friends[0].attack += 1;
                    friendly_friends[0].health += 1;
                }
                return
            }
            let mut rng = thread_rng();
            let rand_nums = rand::seq::index::sample(&mut rng, friendly_friends.len(), 2).into_vec();
            for i in rand_nums.iter().map(|&x| x as usize){
                friendly_friends[i].attack += 1;
                friendly_friends[i].health += 1;
            }
        }
        else if food.id == 6{//garlic
            friendly_friends[pet_idx].food_id = food.id;
        }
        else if food.id == 7{//canned food
            self.canned_food_cnt += 1;
            for mut friend in &mut self.for_sale{
                friend.attack += 1;
                friend.health += 1;
            }
        }
        else if food.id == 8{//pear
            friendly_friends[pet_idx].attack += 2;
            friendly_friends[pet_idx].health += 2;
        }
        else if food.id == 9{//chili
            friendly_friends[pet_idx].food_id = food.id;
        }
        else if food.id == 10{//chocolate
            friendly_friends[pet_idx].xp += 1;
            //need to check if lvlup effect happens
        }
        else if food.id == 11{//sushi
            if friendly_friends.len() < 3{
                if friendly_friends.len() == 2{
                    friendly_friends[0].attack += 1;
                    friendly_friends[0].health += 1;
                    friendly_friends[1].attack += 1;
                    friendly_friends[1].health += 1;
                }
                else if friendly_friends.len() == 1{
                    friendly_friends[0].attack += 1;
                    friendly_friends[0].health += 1;
                }
                return
            }
            let mut rng = thread_rng();
            let rand_nums = rand::seq::index::sample(&mut rng, friendly_friends.len(), 3).into_vec();
            for i in rand_nums.iter().map(|&x| x as usize){
                friendly_friends[i].attack += 1;
                friendly_friends[i].health += 1;
            }
        }
        else if food.id == 12{//melon
            friendly_friends[pet_idx].food_id = food.id;
        }
        else if food.id == 13{//mushroom
            friendly_friends[pet_idx].food_id = food.id;
        }
        else if food.id == 14{//pizza
            if friendly_friends.len() < 2{
                if friendly_friends.len() == 1{
                    friendly_friends[0].attack += 2;
                    friendly_friends[0].health += 2;
                }
                return
            }
            let mut rng = thread_rng();
            let rand_nums = rand::seq::index::sample(&mut rng, friendly_friends.len(), 2).into_vec();
            for i in rand_nums.iter().map(|&x| x as usize){
                friendly_friends[i].attack += 2;
                friendly_friends[i].health += 2;
            }
        }
        else if food.id == 15{//steak
            friendly_friends[pet_idx].food_id = food.id;
        }
        self.food.remove(food_idx);
    }
}

pub fn turn_num_to_ranges(turn_num: i32) -> (i32, i32, i32){
    //returning (how many animals are the shop, max animal int in the range, max food int)
    //game will start on turn 1
    //will return number for range 0..current max shop int
    //non-inclusive
    if turn_num < 3{
        return (3, 9, 2)
    }
    else if turn_num < 5{
        return (3, 19, 5)
    }
    else if turn_num < 7{
        return (4, 30, 7)
    }
    else if turn_num < 9{
        return (4, 41, 9)
    }
    else if turn_num < 11{
        return (5, 49, 12)
    }
    else{
        return (5, 58, 16)
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

pub fn check_pet_ids() -> (){
    for i in 0..PETS.len(){
        if PETS[i].0 != i as i32{
            println!{"Pet off = {}", PETS[i].3}
        }
    }
}