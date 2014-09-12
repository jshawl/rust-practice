fn main() {
    let mut count = 0i;
    let mut sum = 0i;
    while count < 1000 {
        if count % 3 == 0 || count % 5 == 0{
	  sum += count;
	}
	count += 1;
    }
    println!("{}", sum);
}