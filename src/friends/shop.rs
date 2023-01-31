use crate::friends::Friend;

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

pub const PETS: [[i32; 3]; 10] = [
    //id, attack, health
    [ANT, 2, 1],
    [BEAVER, 3, 2],
    [CRICKET, 1, 2],
    [DUCK, 2, 3],
    [FISH, 2, 2],
    [HORSE, 2, 1],
    [MOSQUITO, 2, 2],
    [OTTER, 1, 2],
    [PIG, 4, 1],
    [FLAMINGO, 4, 2],
];

pub struct Shop{
    pub turn_num: i32,
    pub frozen: Vec<Friend>,
    pub for_sale: Vec<Friend>,
    pub lvl_up: Vec<Friend>,
}

// impl Shop{
//     pub fn roll(&self, &PETS) -> (){
//         //remove all elements from for_sale vec
//         //add all the elements in frozen to for_sale
//         //empty frozen vec
//         //fill the remaining spots with random from PETS
//     }
//     pub fn freeze(&self, idx){
//         //takes and idx from for_sale vec and adds it here
//     }
// }