use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 맞혀봅시다");
    
    // let secret_number =	rand::thread_rng().gen_range(1, 101);
    let mut rng = rand::thread_rng();
    let secret_number =	rng.gen_range(1..101);    

    loop {
	// println!("사용자가 맞혀야 할 숫자: {}", secret_number);	
	println!("정답이라고 생각하는 숫자를 입력하세요.");

	let mut guess = String::new();

	io::stdin().read_line(&mut guess)
	    .expect("입력한 값을 읽지 못했습니다.");

	let guess: i32 = match guess.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};

	println!("입력한 값: {}", guess);

	match guess.cmp(&secret_number) {
	    Ordering::Less    => println!("입력한 숫자가 guess number 보다 작습니다!"),
	    Ordering::Greater => println!("입력한 숫자가 guess number 보다 급니다!"),
	    Ordering::Equal => {
		println!("정답!");
		break;
	    }
	}
    }
}
