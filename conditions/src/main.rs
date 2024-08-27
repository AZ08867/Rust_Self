fn main() {
    println!("Hello, world!");
    /*
    if 条件表达式{
        //条件表达式为真，就执行这个代码
    }
    */

    let total:f32 = 666.00;
    if total > 500.00{
        println!("打8折,{}", total * 0.8);
    }


    /*
    if - else 如下面的代码示例：
    */
    let total:f32 = 166.00;
    if total > 500.00 {
        println!("打8折：{}", total * 0.8);
    } else {
        println!("没达到优惠的条件，只能原价：{}", total);
    }

    let code = "10010";
    let choose = match code {
        "10010" => "联通",
        "10086" => "移动",
        _ => "Unknown"
    };
    println!("选择{}", choose);


    /*
        Rust 中的循环有三种:
            loop 重复执行且永远不会结束的循环
            while 在某些条件为真的情况下就会永远执行下去的循环
            for 有确定次数的循环

        - for 临时变量 in 左区间..右区间{
            //执行业务逻辑
            //区间是左闭右开——[)
        }
    */

    for num in 1..5{
        println!("num is {}", num);
    }
    for num1 in 1..=5{
        println!("num1 is {}", num1);
    }

    let study_list = vec![
        "人生",
        "斜面",
        "深度学习",
    ];
    // iter()--->每次迭代是借用集合中的一个元素。元素本身不会改变，循环之后还可以使用。
    for name in study_list.iter(){
        match name {
          &"深度学习" => println!("恭喜你进入下一个阶段的学习--{}", name), _ => println!("继续努力学习：{}", name),
        }
    }


    let study_list2 = vec![
        "人生",
        "斜面",
        "深度学习",
    ];
    // iter()--->会消耗集合，每次迭代，集合中的数据本身被提供，一旦集合被消耗完了。之后无法再使用，因为它已经在循环中被move了
    for name in study_list2.into_iter(){
        match name {
            "深度学习" => println!("恭喜你进入下一个阶段的学习--{}", name), _ => println!("继续努力学习：{}", name),
        }
    }


    let  mut study_list3 = vec![
        "人生",
        "斜面",
        "深度学习",
    ];
    // 借用集合中的每个元素，从而允许集合被就地修改。
    for name in study_list3.iter_mut(){
        *name = match name {
            &mut "深度学习" => {"恭喜你进入下一个阶段的学习--深度学习"}, _ => *name,
        }
    }
    println!("study_list3:{:?}", study_list3);


    /*
        while(条件表达式){
            执行业务代码
        }
    */

    let mut num = 1;
    while num < 20 {
        println!("num is {}", num);
        num *= 2;
    }


    let mut num1 = 1;
    loop {
        if num1 > 20{
            break;
        }
        println!("num1 is {}", num1);
        num1 *= 3;
    }
}
