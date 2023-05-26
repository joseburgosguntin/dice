use rand::Rng;

fn main() {
    const DICE_COUNT: usize = 6;
    let mut thread_rng = rand::thread_rng();

    let mut rounds = 0;
    let value = loop {
        rounds += 1;
        let first = thread_rng.gen_range(1u8..=6);
        println!("start");
        if [0u8; DICE_COUNT - 1]
            .iter()
            .map(|_| thread_rng.clone().gen_range(1..=6))
            .inspect(|dice| println!("{dice}"))
            .all(|dice| dice == first)
        {
            break first;
        }
    };
    println!("It took {rounds} rounds to get six {value}'s");
}
