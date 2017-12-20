
/*
 * Calculate the probabily of dealing(32 card, 8 players):
 *              1- 4 of a kind
 *              2- 3 of a kind
 *              3- 2 of a kind
 *              4- 2 of a kind and a 2 of another
 *              5- Highest card Ace
 *              6- Staright of same suit
 */
extern crate rand;
extern crate libcoinche;

//use rand::Rng;
use libcoinche::cards::Card;
use libcoinche::cards::Hand;
use libcoinche::cards::Deck;
use libcoinche::cards::Suit;

fn is_4x(hand: Hand) -> bool{
    for i in 0..7{
        if Hand::has(hand, Card::from_id(i)) && Hand::has(hand, Card::from_id(i+8)) && Hand::has(hand, Card::from_id(i+16)) && Hand::has(hand, Card::from_id(i+24)) {
            println!("4x hand: {}", Hand::to_string(&hand));
            return true
        }
    }
    false
}
fn is_2or3or4x(hand: Hand) -> usize{
    let mut sum = 0;
    let mut Ace = 0;

    for i in 0..8{
        if Hand::has(hand, Card::from_id(i)){
            sum += 1;
            if i==7{
                Ace = 1;
            }
        }
        if Hand::has(hand, Card::from_id(i+8)) {
            sum += 1;
            if i==7{
                Ace = 1;
            }
        }
        if Hand::has(hand, Card::from_id(i+16)) {
            sum += 1;
            if i==7{
                Ace = 1;
            }
        }
        if Hand::has(hand, Card::from_id(i+24)) {
            sum += 1;
            if i==7{
                Ace = 1;
            }
        }
        match sum{
            2 => {
                for another in i+1..8{
                    if Hand::has(hand, Card::from_id(another)){
                        sum += 1;
                    }
                    if Hand::has(hand, Card::from_id(another+8)) {
                        sum += 1;
                    }
                    if Hand::has(hand, Card::from_id(another+16)) {
                        sum += 1;
                    }
                    if Hand::has(hand, Card::from_id(another+24)) {
                        sum += 1;
                    }
                    match sum{
                        3 => {
                            //println!("2x hand: {}", Hand::to_string(&hand));
                            return 2;
                        }, 
                        4 => {
                            //println!("2vs2 hand: {}", Hand::to_string(&hand));
                            return 5;
                        },
                        _ => {},
                    }
                }
                //println!("2x hand: {}", Hand::to_string(&hand));
                return 2;
            },

            3 => {
                //println!("3x hand: {}", Hand::to_string(&hand));
                return 3;
            },

            4 => {
                //println!("4x hand: {}", Hand::to_string(&hand));
                return 4;
            },

            _ => {},//println!("i = {} Ace = {}",  i, Ace),
        }

        sum = 0;
    }

   //println!("{}", Hand::to_string(&hand));
    if Ace == 1{
        //println!("Ace hand: {}", Hand::to_string(&hand));
        return 1;
    }

    0
}
fn main(){

    let mut deck_of_cards = Deck::new();
    let mut hands4: [Hand; 4] = [Hand::new(); 4];
    let mut hands8: [Hand; 4] = [Hand::new(); 4];

    let mut stat: [u32; 7] = [0; 7];

    println!("Deck Probabilities!");


    let mut seed : [u32; 1];

    for x in 0..1000{
        seed = rand::random();
        Deck::shuffle_seeded(&mut deck_of_cards, &seed);

        Deck::deal_each(&mut deck_of_cards, &mut hands4, 4);
        Deck::deal_each(&mut deck_of_cards, &mut hands8, 4);

        for i in 0..4{
            stat[is_2or3or4x(hands4[i])] += 1;
            stat[is_2or3or4x(hands8[i])] += 1;
            Hand::clean(&mut hands4[i]);
            Hand::clean(&mut hands8[i]);
        }
        deck_of_cards = Deck::new();
    }

    let exp_stat = vec!["nothing", "Highest Card Ace", "2x", "3x", "4x", "2vs2", "Straight"];
    for x in 0..7{
        println!( "stat[{}] = {}", exp_stat[x], stat[x]);
    }
    
}
    //println!("heart: {}", Suit::to_string(Heart));
    //println!("deck: {}", Deck::to_string(&deck_of_cards));
        
