use collatz::collatz_length;

fn main() {
    let end = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut max_steps = 0;
    let mut max_input = 1;
    for i in 1..=end {
        let s = collatz_length(i).unwrap();
        if s > max_steps {
            max_steps = s;
            max_input = i;
        }
    }
    println!("{} {}", max_input, max_steps);
}
