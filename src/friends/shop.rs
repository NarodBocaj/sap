use crate::friends::Friend;
use crate::friends::friend_maker;
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
pub const ANT: i32 = 0;
pub const BEAVER: i32 = 1;
pub const CRICKET: i32 = 2;
pub const DUCK: i32 = 3;
pub const FISH: i32 = 4;
pub const HORSE: i32 = 5;
pub const MOSQUITO: i32 = 6;
pub const OTTER: i32 = 7;
pub const PIG: i32 = 8;
pub const FLAMINGO: i32 = 9;

pub const PETS: [(i32, i32, i32, &str); 10] = [
    //id, attack, health
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
];

pub struct Shop{
    pub turn_num: i32,
    pub frozen: Vec<Friend>,
    pub for_sale: Vec<Friend>,
    pub lvl_up: Vec<Friend>,//place holder for the lvlup logic of higher tier pet in shop
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