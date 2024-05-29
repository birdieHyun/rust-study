fn main() {
    let x = 4;

    if x % 2 == 0 {
        println!("{x}는 짝수입니다.")
    }else if x % 2 == 1{
        println!("{x}는 홀수입니다.")
    }

    let condition = false;
    let y = if condition == true { 3 } else { 5 };

    println!("y의 값 : {y}");

    let mut counter = 0;
    loop {
        println!("루프중");
        counter += 1;

        if counter == 3 {
            break
        }
    }

    // loop에서 반환값을 줄 수도 있다.

    let mut counter = 0;
    let x = loop {
        counter += 1;

        if counter == 3 {
            break counter
        }
    };

    println!("loop 반환값 {x}");

    // loop 보단 while을 더 사용
    let mut counter = 0;
    while (counter < 3) {
        println!("while 중");
        counter += 1;
    }

    // for 문을 활용해서 배열 탐색
    let xs = [1, 2, 3, 4, 5];

    for x in xs {
        println!("{x}")
    }

    // for문 숫자 반복
    for i in (0..4){
        println!("{i}")
    }

    let l = xs.len();
    for i in (0..l) {
        println!("{i}")
    }

    // 역순 출력
    for i in (0..l).rev() {
        println!("{i}")
    }
}
