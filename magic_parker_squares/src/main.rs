use rand_chacha::rand_core::SeedableRng;
use rand_chacha::rand_core::RngCore;
use rand_chacha; // 0.3.0

const MAX_VAR_SIZE: u64 = 1000;
const SQUARE_WIDTH: usize = 3;
const CORNER_CHAR: &str = " ";

fn get_n_random_ints(n: usize, rng: &mut rand_chacha::ChaCha8Rng) -> Vec<u64> {
    let mut v: Vec<u64> = Vec::new();
    let max_size_vars: u64 = (
        (u64::MAX as f64).sqrt() / 1000.) as u64;
    for _ in 0..n {
        v.push(rng.next_u64() % MAX_VAR_SIZE);
    }
    return v;
}

fn get_squared_sum(v: Vec<u64>) -> u64 {
    let mut sum: u64 = 0;
    for x in v {
        sum += (x).pow(2);
    }
    return sum;
}

fn plot_magic_square(v: Vec<u64>) {
    assert_eq!(v.len(), 9);
    let space_len = (MAX_VAR_SIZE - 1).to_string().len() + 4;
    let horiz_line = (
        CORNER_CHAR.to_string() + 
        &"-".repeat(space_len)
    ).repeat(SQUARE_WIDTH) + CORNER_CHAR;
    println!("{}", horiz_line);
    let mut i_v: usize = 0;
    for i_line in 0..SQUARE_WIDTH {
        let mut line = "".to_owned();
        for i_row in 0..SQUARE_WIDTH {
            let num_str = v[i_v].to_string() + &"^2";
            i_v += 1;
            line += &("| ".to_owned() + &num_str + 
                     &" ".repeat(space_len - num_str.len() - 1)
                    ).to_string();
        }
        line += "|";
        println!("{}", line);
        println!("{}", horiz_line);
    }
}

fn main() {
    let mut gen = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    let n_tries = 10E8 as usize;
    for i_try in 0..(n_tries as usize) {
        if (i_try % (n_tries / 10)) == 0 {
            println!("try {}", i_try);
        }
        
        // first, generate outside ring of variables
        let outside: Vec<u64> = get_n_random_ints(8, &mut gen);
        assert_eq!(outside.len(), 8);
        let bit: Vec<u64> = outside.get(0..2).unwrap().to_vec();
        assert_eq!(bit.len(), 2);
        let first_partial_sum: u64 = get_squared_sum(bit);
        for i in 1..4 {
            let other_bit: Vec<u64> = outside.get(
                (2*i)..(2*i+2)).unwrap().to_vec();
            assert_eq!(other_bit.len(), 2);
            let other_partial_sum: u64 = get_squared_sum(other_bit);
            if first_partial_sum == other_partial_sum {
                println!("success try {}", i_try);
                println!("partial_sum {}", first_partial_sum);
                let magic_square: Vec<u64> = vec![
                    // first row
                    outside[0],
                    outside[2],
                    outside[4],
                    // second row
                    outside[6],
                    0,
                    outside[7],
                    // third row
                    outside[5],
                    outside[3],
                    outside[1]
                ];
                plot_magic_square(magic_square);
                println!("{}", "=".repeat(40));
            }
        }

    }
}
