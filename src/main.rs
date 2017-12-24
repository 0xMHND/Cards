
/*
 * Calculate the probabily of dealing(32 card, 8 players):
 *              0- nothing
 *              1- Highest card Ace
 *              2- 2 of a kind
 *              3- 3 of a kind
 *              4- 4 of a kind
 *              5- 2 of a kind and a 2 of another
 *              6- Staright of same suit
 *
 *              0 -> 7
 *              1 -> 8
 *              2 -> 9
 *              3 -> J
 *              4 -> Q
 *              5 -> K
 *              6 -> X
 *              7 -> A
 */
/*
 * What is the chance of having 2 2x vs 1 2x in one game?
 *
 * Calculate the average of each category in each game then get the prob. 
 *
 * TODO:
 * Calculate winning hand
 *      Straight
 *      4x
 *      3x
 *      2vs2
 *      2x
 *      Ace
 *      nothing : should be 0
 */


//#![allow(dead_code)] 

extern crate rand;
extern crate libcoinche;

//use rand::Rng;
use libcoinche::cards::Card;
use libcoinche::cards::Hand;
use libcoinche::cards::Deck;
use libcoinche::cards::Suit;
use libcoinche::cards::Rank;


const PLAYERS : usize = 5;
const ITERATION : usize = 1000;
const CATEGORY : usize = 7;
const TARGET : usize = Categories::TwoVsTwo as usize;
const BASE : usize = Categories::TwoX as usize;

enum Categories{
    Nothing,
    HighAce,
    TwoX,
    ThreeX,
    FourX,
    TwoVsTwo,
    Straight,
}

fn main(){

    let mut hands: [Hand; PLAYERS] = [Hand::new(); PLAYERS];

    let mut deck_of_cards = Deck::new();

    let mut tests = Vec::new();

    let mut stat: [u32; 7] = [0; 7];
    let mut one_test: [u32; 7] = [0; 7];

    let mut seed : [u32; 1];

    println!("Deck Probabilities!");


/* CALCULATE */
    
    for x in 0..ITERATION{
        seed = rand::random();
        Deck::shuffle_seeded(&mut deck_of_cards, &seed);

        deal_cards(&mut hands, PLAYERS);

        for i in 0..PLAYERS{
            stat[is_2or3or4x(hands[i])] += 1;
            one_test[is_2or3or4x(hands[i])] += 1;
            Hand::clean(&mut hands[i]);
        }
        deck_of_cards = Deck::new();

        tests.push(one_test);
        one_test = [0;7];
    }

    let mut whole_iter : [f32; 7] = [0.;7];

    /**special stat**/
    let mut two_x_stat : [u32; 3] = [0;3];

    for x in &tests{
        if x[BASE] >= 1 {
            two_x_stat[per_game_stats(x, TARGET)] += 1
        }
        average_per_game( x, &mut whole_iter, PLAYERS as f32);
    }

/* PRINT STATS */

    let exp_stat = vec!["nothing", "Highest Card Ace", "2x", "3x", "4x", "2vs2", "Straight"];
    let percentage = 100./(PLAYERS as f32* ITERATION as f32);
    for x in 0..7{
        println!( "{:0.2}% | {:0.2}% ||| {: <16} = {: <5}", stat[x] as f32 * percentage, whole_iter[x] * percentage, exp_stat[x], stat[x]);
    }

    println!( "Freq({}) given {} exist at least one hand", exp_stat[TARGET], exp_stat[BASE]);
    for i in 0..3{
        println!( "[{}] = {} -> {}%", i, two_x_stat[i], (100. * two_x_stat[i] as f32 )/(ITERATION as f32));
    }
}



fn per_game_stats(one_game_stat: &[u32; 7], categ: usize) -> usize {

    if one_game_stat[categ] == 1{
        return 1;
    }
    else if one_game_stat[categ] > 1{
        return 2;
    }

    0
}   



fn is_straight(hand: Hand) -> bool{
    let mut strt = 0;
    let mut curr_card : i32 = 0;
    let mut prev_card : i32 = 0;


    if Hand::has_any(hand, Suit::Heart){
        strt += 1;
    }
    if Hand::has_any(hand, Suit::Diamond){
        strt += 1;
    }
    if Hand::has_any(hand, Suit::Spade){
        strt += 1;
    }
    if Hand::has_any(hand, Suit::Club){
        strt += 1;
    }

    if strt == 1{
        let cards = Hand::list(hand);
        prev_card = Card::id( cards[0] ) as i32;
        curr_card = Card::id( cards[1] ) as i32;

        let mut result: i32 = curr_card - prev_card;
        
        if (result==1) || (result==-1){
            prev_card = curr_card;
            curr_card = Card::id( cards[2] ) as i32;
            result += curr_card - prev_card;
            
            if (result == 2) || (result == -2){
                prev_card = curr_card;
                curr_card = Card::id( cards[3] ) as i32;
                result += curr_card - prev_card;

                if ( result == 3) || ( result == -3) {
                    //println!("Straight hand: {}", Hand::to_string(&hand));
                    return true;
                }
            }
        }
    }
   false 
}

fn is_2or3or4x(hand: Hand) -> usize{
    let mut sum = 0;
    let mut ace = 0;
    
    if is_straight(hand) {
        return Categories::Straight as usize;
    }

    for i in (8-PLAYERS as u32)..8{
        if Hand::has(hand, Card::from_id(i)){
            sum += 1;
            if i==7{
                ace = 1;
            }
        }
        if Hand::has(hand, Card::from_id(i+8)) {
            sum += 1;
            if i==7{
                ace = 1;
            }
        }
        if Hand::has(hand, Card::from_id(i+16)) {
            sum += 1;
            if i==7{
                ace = 1;
            }
        }
        if Hand::has(hand, Card::from_id(i+24)) {
            sum += 1;
            if i==7{
                ace = 1;
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
                            return Categories::TwoX as usize;
                        }, 
                        4 => {
                            //println!("2vs2 hand: {}", Hand::to_string(&hand));
                            return Categories::TwoVsTwo as usize;
                        },
                        _ => {},
                    }
                }
                //println!("2x hand: {}", Hand::to_string(&hand));
                return Categories::TwoX as usize;
            },

            3 => {
                //println!("3x hand: {}", Hand::to_string(&hand));
                return Categories::ThreeX as usize;
            },

            4 => {
                //println!("4x hand: {}", Hand::to_string(&hand));
                return Categories::FourX as usize;
            },

            _ => {},//println!("i = {} Ace = {}",  i, Ace),
        }

        sum = 0;
    }

   //println!("{}", Hand::to_string(&hand));
    if ace == 1{
        //println!("Ace hand: {}", Hand::to_string(&hand));
        return Categories::HighAce as usize;
    }
    Categories::Nothing as usize
}

fn average_per_game( one_game_stat: &[u32; 7], total : &mut [f32;7], hands: f32) {

    for i in 0..7{
        total[i] = (one_game_stat[i] as f32/hands) + total[i];
    }
}

fn deal_cards(hands: &mut [Hand], players_n: usize) {

    let mut deck_of_cards = Deck::new();
    let seed : [u32; 1];
    let mut card : Card;


    let mut cnt = 0;
    let mut cnt_cards = 0;
    let mut hand_n = 0;

    seed = rand::random();
    Deck::shuffle_seeded(&mut deck_of_cards, &seed);

    for i in 0..32{
        card = Deck::draw(&mut deck_of_cards);

        if Card::rank(card) as usize >= ( 1 << (8 - players_n) ) {
            cnt_cards += 1;
            cnt += 1;
            if cnt < 5 {
                //println! ( "{} card {} rank [{}] : {}", cnt, cnt_cards, Card::rank(card) as usize, Card::to_string(card) ); 
                //println!("add to hand");
                Hand::add(&mut hands[hand_n], card);
            }
            else{
                cnt = 1;
                hand_n += 1;
               // println!("start new hand");
               // println! ( "{} card {} rank [{}] : {}", cnt, cnt_cards, Card::rank(card) as usize, Card::to_string(card) ); 
                //println!("add to hand");
                Hand::add(&mut hands[hand_n], card);
            }
        }
    }

}



