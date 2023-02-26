use std::env;

use util::data;

fn asn(risk_limit: i32, winner_votes: i32, loser_votes:i32, total_votes:i32) -> i32{
    // TODO: this conversion feels gross. 
    let diluted_margin: f32 = ((winner_votes - loser_votes) as f32)/(total_votes as f32);
    println!("Generating a bravo sample size with risk-limit {}, diluted margin {}", risk_limit, diluted_margin);

    let p_w: f32 = (winner_votes as f32)/(total_votes as f32);
    let p_l: f32 = (loser_votes as f32)/(total_votes as f32);

    let s_w: f32 = (winner_votes as f32)/(total_votes as f32);

    let alpha: f32 = risk_limit as f32/100.0;

    let z_w: f32 = (2.0*s_w).ln();
    let z_l: f32 = (2.0 - 2.0*s_w).ln();


    let num: f32 = (1.0/alpha).ln() + (z_w/2.0);
    let denom: f32 = (p_w*z_w + p_l*z_l);
    let asn: f32 = num/denom;

    return asn.ceil() as i32;

}

fn main() {

    let args: Vec<String> = env::args().collect();


    let risk_limit: i32 = (&args[1]).parse::<i32>().unwrap();
    let winner_votes: i32 = (&args[2]).parse::<i32>().unwrap();
    let loser_votes: i32  = (&args[3]).parse::<i32>().unwrap();
    let total_votes: i32  = (&args[4]).parse::<i32>().unwrap();

    let a = asn(risk_limit, winner_votes, loser_votes, total_votes);
    println!("The ASN is {}", a);

}
