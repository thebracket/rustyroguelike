use rand::Rng;

pub fn random_choice(table : Vec<(String, i32)>) -> String {
    let mut rng = rand::thread_rng();

    let n = rng.gen_range(1,100);
    let mut running_sum = 0;
    for (opt,chance) in table.iter() {
        if n < chance+running_sum { return opt.to_string() }
        running_sum += chance;
    }
    table[0].0.to_string()
}