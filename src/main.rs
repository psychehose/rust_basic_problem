fn main() {
    println!("섭씨: {} -> 화씨: {}", 0, c_to_f(0.0));
    println!("화씨: {} -> 섭씨: {}", 32.0, f_to_c(32.0));

    let n = 7;
    println!("{}번째 피보나칠 수열의 값 = {}", n, fibo(n));

    let string = "The Twelve Days of Christmas";

    for c in string.chars() {
        print!("{}", c);
    }

    println!();

    for i in string.char_indices() {
        print!("{}", string.chars().nth(i.0).unwrap());

    }
    println!();

    for i in string.char_indices() {
        print!("{}", i.1);

    }



}


fn c_to_f(c: f64) -> f64 {
    (9.0/5.0) * c + 32.0
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0)
}

fn fibo(n: i32) -> i32 {
    if n <= 2 {
        1
    } else {
        fibo(n-1) + fibo(n-2)
    }
}


/*

1. 화씨와 섭씨를 상호 변환. -> // 배운것: f64로 타입 맞춰줄 것
    f to c
        -> (x°F − 32) × 5/9
        c = (f - 32) (5/9)
    c to f
        -> f = (9/5)c + 32

2. n번째 피보나치 수열 생성.

3. 크리스마스 캐롤 “The Twelve Days of Christmas”의 가사를 반복문을 활용해 출력. // 배운것: nth(x), .chars()

string 인스턴스 함수
.chars() -> 요소 값
.char_indices() -> index랑 요소
.nth() -> n번째 요소 출력(에러 가능성이 있으므로, unwrap()을 사용하거나, expect를 이용해서 에러 핸들 해야함)

*/