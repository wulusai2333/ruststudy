fn main() {
    println!("Hello, world!");
    test_println();
    test_var();
    test_data_type()
}
fn test_println() {
    let a = 1;
    //{}是一个占位符
    println!("a is {}", a);
    //可以将后面的可变参数当做数组看待
    println!("a is {0} ,b is {0}", a);
    //输出{}需要转义,其他符号转义与C一致
    println!("{{}}");
}
fn test_var() {
    //rust可以推测变量的类型 a是不可变变量
    let a = 10;
    //rust变量默认不能再次赋值
    //a=10;
    //加了 mut关键字 就成了可变变量
    let mut b = 10;
    b = 20;
    print!("{}", b);
    //rust的重影机制 即一个变量名可以被重复使用,在此之后的程序使用的就是新变量的值了
    let b = 20;
    print!("{}", b);
    //重影
    let mut b = b + 1;
    print!("{}", b);
    // 这里 clion没有提示,实际上数字类型不可以调用len()方法
    //b=b.len();
    print!("{}", b);
    //重影对一个变量名的赋值不需要考虑变量类型
    let b = "asd";
    //错误 因为类型不同,不能赋值
    //b=b.len();
}
//常量 必须类型确定,值确定 不可变变量可以自动推测类型
const C: i32 = 10;

fn test_data_type() {
    //integer i8 -> i128  u8 -> u128
    // isize usize 这两种类型取决于使用的平台是32位还是64位机器
    //十进制 可以用_来分割数字以便阅读
    let x = 1_000;
    //16进制 0x 开头
    let x = 0xff;
    //8进制 0o 开头
    let x = 0o77;
    //二进制 0b 开头
    let x = 0b111_000;
    //字节 只能用u8型
    let x = b'Z';
    print!("x");

    //浮点数
    let x = 1.0;
    let x: f32 = 1.1;
    //布尔类型
    let x = true;
    //rust不支持 ++ -- 操作 因为其弱化了代码可读性

    //字符类型 只支持 utf8格式 Unicode
    let x: char = U + 0000;

    //复合类型,类似于java的list
    let tup: (i32, f64, char) = (1, 1.0, u + 1010);
    let (x, y, z) = tup;
    print!(y);
}
