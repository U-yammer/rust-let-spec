fn main() {
    // let x = 5;


    // println!("The value of x is: {}", x);

    // x = 6;　コンパイルエラー

    // println!("The value of x is: {}", x);　
    // println!("println!")

    // shadowing

    let x = 5;
    let x = x + 1; // 間違っても，letを付けず x = x + 1とかにしない
    let x = x * 2;

    println!("shadowing: {}", x); // 12

    let spaces = "   ";
    let spaces = spaces.len(); // re 宣言しているので，こういう書き方が出来る（でも普通，名前かえるよね．混乱するし）

    println!("spaces: {}", spaces); // 12
}
