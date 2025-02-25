fn main() {
    /*
        栈：后进先出。类型大小都是固定好的。如 i32
        堆：编译时大小未知或不确定大小。由用户自己管理，会增加内存溢出风险。
    */
    let a = 88;
    let b = a;
    println!("a is {} and b is {}", a, b);

    // 总结：赋值并不是唯一涉及所有权的操作，值在作为参数传递或从函数返回时也会被移动。
}
