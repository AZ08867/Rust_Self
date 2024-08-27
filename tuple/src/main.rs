fn main() {
    println!("Hello, world!");
    /*
    let tuple 变量名称：(数据类型1,数据类型2,...) = (数据1,数据2,...)
    let tuple 变量名= (数据1,数据2)
    */

    let t = ("深度学习", "更深入了解机器学习");
    println!("{:?}", t);

    println!("{}", t.0);
    println!("{}", t.1);
    show_tuple(t);

    // 元组解构——就是在tuple中的每一个元素按照顺序一个一个赋值给变量。使用 = ，让右边的 tuple 按照顺序给等号左边的变量一个一个赋值。
    let (book, target) = t;
    println!("{}", book);
    println!("{}", target);
}
fn show_tuple(t:(&str, &str)){
    println!("{:?}", t);
}