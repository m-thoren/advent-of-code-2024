use regex::Regex;

fn main() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    for caps in re.find_iter(input) {
        println!("{}", caps.as_str());
    }
}
