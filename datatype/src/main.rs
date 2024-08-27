fn main() {
    /*
    变量——给某一个内存地址起名字
    命名规范：
    1 可以是字母、数字、下划线
    2 只能以字母 or 下划线开头，不能以数字开头
    3 变量名区分大小写

    对于可变变量：
    let mut 变量名:数据类型 = 值;
    mut 关键字，mutable的缩写
    */
    // let study:&str = "halo!";
    // println!("{}", study);
    //
    // let mut price = 66.5;
    // price = 90.5;
    // print!("price is {}", price);


    /*
    对于常量
    其名称通常是大写字母
    使用 const 关键字
    */
    // const PI: f64 = std::f64::consts::PI;
    // println!("{}", PI);
    // // const AREA:i64 = 86;
    // // println!("{}", AREA);
    //
    // // 常量声明的另外一种方式
    // static BOOK1: &str = "《Rust》";
    // println!("{}", BOOK1);
    // static BOOK2: &'static str = "《Python 程序设计》";
    // println!("{}", BOOK2);

    /*
    string 类型
    创建字符串对象的方式：
    String::new() 创建一个新的空字符串，是静态的
    String::from() 从具体的字符串字面量创建字符串对象


    Rust 语言提供了两种字符串
    - Rust 核心内置的数据类型 &str ——字符串字面量
    - Rust 标准库中的一个公开的 pub 结构体，字符串对象 String
    */

    // &str 是在模块 std:str 的字符串切片
    let lesson = "Julia 程序设计";
    let str1 = String::new();
    let str2 = String::from("面向对象编程");

    println!("{}", lesson);
    println!("str1:{}, str1-len:{}", str1, str1.len());
    println!("str2:{}, str2-len:{}", str2, str2.len());


    let mut str3 = String::new();
    str3.push_str("Java 程序设计");
    println!("{}", str3);

    str3.push('O');
    str3.push('K');
    str3.push('a');
    str3.push('y');
    println!("{}", str3);

    let str4 = String::from("面向AI编程");
    let result = str4.replace("面向AI编程", "www.baidu.com");
    println!("{}", result);

    let str5 = String::from("面向AI编程 www.baidu.com");
    println!("length is {}", str5.len());

    let str6 = "cpp 程序设计".to_string();
    println!("{}", str6);

    let str7 = String::from("V语言程序设计");
    show_name(str7.as_str());

    // 去掉头尾空白符，制表符 \t 空格 回车\r 换行 \n 回车换行 \r\n
    let str8 = " \tDeep learning 花书\t基于Python的计算机视觉 \r\n从0到精通的open cv\r\n   ";
    println!("length is {}", str8.len());
    println!("length is {}", str8.trim().len());
    println!("str8 is {}", str8);

    let str9 ="Deep learning 花书;基于Python的计算机视觉;从0到精通的open c";
    for i in str9.split(';'){
        println!("值得学习的科目：{}", i);
    }

    let str0 = "从0开始学习Python";
    for c in str0.chars(){
        println!("字符：{}", c);
    }


    let str01 = "机器学习--西瓜书".to_string();
    let str02 = ", 写的很好！".to_string();
    let result01 = str01 + &str02;
    println!("{}", result01);
}

fn show_name(name:&str){
    println!("这是一个待学习的科目：{}", name)
}