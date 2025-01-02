
fn check_prime(primes_vector: &Vec<u32>, integer: &u32, current_length_of_prime_vector: &usize) -> u32 {
    let mut current_position: usize = 1; 
    for prime in &*primes_vector {
        if current_position == *current_length_of_prime_vector {
                match integer % prime {
                    0 => return 0, // not prime
                    _ => return *integer, // return the found prime
                }
        } else {
            current_position += 1
        } 
        match integer % prime {
            0 => return 0, // not prime
            _ => continue, // relativly prime with current prime
        }
    }
    return 0;
}





fn main() {
    const MAX_NUM: u32 = 1000;
    
    let _number_vector: Vec<u32> = (2..=MAX_NUM).collect();
    
    let mut primes_vector: Vec<u32> = vec![];
    
    primes_vector.push(2);

    for integer in &_number_vector {
        
        let current_length_of_prime_vector = primes_vector.len();
        let _new_prime = check_prime(&primes_vector, &integer, &current_length_of_prime_vector); 
        
        if _new_prime != 0 {
            primes_vector.push(_new_prime);
        } 
    }

    println!("{:?}", primes_vector);
}
