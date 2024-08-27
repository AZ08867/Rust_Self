fn main() {
    /*
    let 变量名称:[数据类型;数组长度] = [值1.值2,值3,...];
    let 变量名称 = [值1.值2,值3,...];
    let 变量名称:[数据类型;数组长度] = [默认值;数组长度];
    */

    let arr1: [&str; 3] = ["机器学习", "集成式学习", "深度学习"];
    let  arr2= ["机器学习", "集成式学习", "深度学习"];
    // let arr3: [&str; 3] = [""; 3];
    println!("{}", arr1.len());

    // for item in arr1 {
    //     println!("必学书本：{}\n", item);
    // }
    //
    // for item in arr1.iter() {
    //     println!("必学书本：{}\n", item);
    // }

    println!("{:?}", arr2);
    show_arr(arr2);
    println!("{:?}", arr2);
}
fn show_arr(mut arr:[&str; 3]){
    let l = arr.len();
    for i in 0..l{
        if i == 0{
            arr[0] = "";
        }
        println!("必学：{}", arr[i])
    }
}
