fn main() {
    for i in 1..11 {
        for l in 1..9 {
            print!("\t{} X {} = {}", l, i, i*l);
        }
        println!("");
    }
}
