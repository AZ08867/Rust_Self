fn main() {
    println!("Hello, world!");
    // 借用 &变量名
    let study_list = vec!["机器学习", "计算机视觉", "深度学习"];
    let study_list2 = study_list;
    show(&study_list2);
    println!("{:?}", study_list2);
    let mut study_list3 = vec!["机器学习", "计算机视觉", "深度学习"];
    println!("{:?}", study_list3);
    show2(&mut study_list3);
    println!("{:?}", study_list3);
    // borrow 从一个函数中的变量传递给另外一个函数作为参数暂时使用。函数离开以后将所有权返还给当初传递给它的变量。
    // 可变借用，定义时候和使用的时候都要用 &mut
}

fn show(v:&Vec<&str>){
    println!("{:?}", v);
}

fn show2(v:&mut Vec<&str>){
    v[0] = "第一本书已开始";
    println!("{:?}", v);
}