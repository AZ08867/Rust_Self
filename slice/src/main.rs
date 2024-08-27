fn main() {
    println!("Hello, world!");
    // slice : 切片
    /*
        let 切片变量 = &变量[起始位置..结束位置]
    */

    let mut v = Vec::new();
    v.push("线性代数");
    v.push("高等数学");
    v.push("概率论与数理统计");
    println!("len:{}", v.len());
    let  s1 = &v[0..3];
    println!("{:?}", s1);
    show_slice(s1);

    let mut v2 = Vec::new();
    v2.push("线性代数");
    v2.push("高等数学");
    v2.push("概率论与数理统计");
    println!("{:?}", v2);
    modify_slice(&mut v2[1..3]);
}

fn show_slice(s:&[&str]){
    println!("show_slice函数内:{:?}", s);
}

fn modify_slice(s:&mut [&str]){
    s[0] = "高等代数";
    println!("modify_slice函数内:{:?}", s);
}