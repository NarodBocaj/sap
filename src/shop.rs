mod friends
//counts
//tier 1: 9     idxs: 0-8
//tier 2: 10    idxs: 9-18
//tier 3: 11    idxs: 19-29
//tier 4: 11    idxs: 30-40
//tier 5: 8     idxs: 41-48
//tier 6: 9     idxs: 49-57
//total 58
const ANT = 0;
const BEAVER = 1;
const CRICKET = 2;
const DUCK = 3;
const FISH = 4;
const HORSE = 5;
const MOSQUITO = 6;
const OTTER = 7;
const PIG = 8;

const PETS = [
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
]

pub struct Shop{
    pub turn_num: i32,
    pub frozen: Vec<friends::Friend>,
    pub for_sale: Vec<friends::Friend>,
}

impl Shop{
    pub fn roll(&self, &PETS) -> (){
        //remove all elements from for_sale vec
        //add all the elements in frozen to for_sale
        //empty frozen vec
        //fill the remaining spots with random from PETS
    }
    pub fn freeze(&self, idx){
        //takes and idx from for_sale vec and adds it here
    }
}   



