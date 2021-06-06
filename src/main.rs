//fn main() {

// let  str = "hello";                            //字符串字面值是一个字符串切面的引用
// println!("{}",str.to_string());
//
// let  str = 5;                                   //变量隐藏，以实现重新赋值，此时类型可以不一致。
// println!("{}",str);
//
// let  mut prefix :String = String::from("hello");  // String 不是简单数据类型，但此处依旧发生了字面值到变量的借用，字符串未实现copy trait
// println!("{}",prefix);
// let  app_str :&str  = "world";                       // 显示声明一个字符串的切片引用，或者说字符串切片。当谈论字符串时，指的就是String或者&str
// prefix.push_str(app_str);                      // 引用，所有权未发生转移
// println!("{}",app_str);
// println!("{}",prefix);
//
// let x =5;
// let y =&x;                                     // 1 & 在rust 中，表示的是引用 ，其结果不是一个 类似long的类型也可以看出，此处任然是一个指针指向的值
// let z=&y;                                     // 2 对引用再次引用
// println!("{}",z);
//
// let x = 5;
//
// //是否发生了copy????  个人分析,虽然是引用，但是也发生了copy. 因为在栈上，不管引用，还是拷贝，本身就意味着分配内存！！！
// let y= &x;       //事实上 如果写成 let y =x ,是利用了自动引用和解引用的机制???，可以看到以下代码结果完全一样
// println!("&y={},*y={},y={}",&y,*y,y);
// let x = 6;
// assert_eq!(*y,5);      //注意*y ,此处结果为真 ，但此处没有利用到自动引用和解引用的机制？？？
//
//
//
//
// //简单数据，整型，浮点，布尔，字符、字符串字面值，复合（元组，数组）， 其压在栈上，实现了copy trait;
// //切片没有所有权
// //String 类型，分配在堆上，在创建时是不知道分配具体的内存大小。
//
// //1 如下，数据发生了cpoy,即在栈上分配了两个5，变量与数据交互方式是移动,类似其他语言的浅拷贝，这里没有深浅拷贝的区别
// let a =5;
// let b =a;
// println!("{},{}",a,b);
//
// //2 接上，以下代码会在编译时报错，应为String 类型的值不能copy
// let str1 =String::from("hello");
// //let str2 =str1;
// // 这里可以使用克隆
// let str2 =str1.clone();
// println!("{},{}",str1,str2);
//
// //我们将获取引用作为函数参数称为 借用（borrowing）
// //转移数据的所有权，数据在栈上进行了拷贝
//
//
// // box 在 main 的末尾离开作用域时，它将被释放。这个释放过程作用于 box 本身（位于栈上）和它所指向的数据（位于堆上）
// let b = Box::new(5);
// println!("b = {}", b);


//}


// // rust的函数类型，其实应该是属于指针类型（Pointer Type）。rust的Pointer Type有两种，一种为引用（Reference&），另一种为原始指针（Raw pointer *）
// // 函数类型是引用，但是不需要& 显示的声明或者调用。
// fn main() {
//     let func: IncType = inc;
//     println!("3 + 1 = {}", func(3));
//     println!("3 + 1 = {}", inc(3));
//     println!("4 + 1 = {}", inc(4));
// }
// type IncType = fn(i32) -> i32;
// fn inc(n: i32) -> i32 {
//     n + 1
// }

// fn main(){
//     let x = 1;
//     let c = 'c';
//     match c {
//         // x 被 c 的值 'c'覆盖
//         x => println!("x: {} c: {}", x, c),
//     }
//     //  内部x作用域之外， 外部x还原
//     println!("x: {}", x);
// }
// struct Point {
//     x: i64,
//     y: i64,
// }
// fn  main() {
//     let point = Point { x: 1, y: 0 };
//     match point {
//         //将point 赋值给 Point { x, y }
//         Point { x, y } => println!("({},{})", x, y),
//     }
//     match point {
//         // 对字段重新命名
//         Point { x: x1, y: y1} => println!("({},{})", x1, y1),
//     }
//     match point {
//         // 忽略不感兴趣的字段
//         Point { y, .. } => println!("y is {}", y),
//     }
// }

// fn main(){
//
//     let tuple: (u32, String) = (5, String::from("five"));
//     let (_x, _s) = tuple;
//     // 以下行将导致编译错误，因为String类型并未实现Copy, 所以tuple被整体move掉了。
//     // println!("Tuple is: {:?}", tuple);
//     let tuple = (5, String::from("five"));
//     // 忽略String类型，而u32实现了Copy，则tuple不会被move
//     let (_x, _) = tuple;
//     println!("Tuple is: {:?}", tuple);
//
// }

// fn main(){
//     // 对于简单数据类型，或者实现了copy trait的数据类型，当只想引用尔不想获取其所有权的时候
//     let mut x = 5;
//     match x {
//         ref mut mr => println!("mut ref :{}", mr),
//     }
//     // 在let表达式里也能用
//     let ref mut mrx = x;
// }

// fn main(){
//     let x = 1u32;
//     match x {
//         //变量绑定，e绑定了x的值 | 多重匹配
//         e @ 1 ... 5 | e @ 10 ... 15 => println!("get:{}", e),
//         _ => (),
//     }
//
// }

// #[derive(Debug)]
// struct Person {
//     name: Option<String>,
// }
// fn main() {
//     let name = "Steve".to_string();
//     let x: Option<Person> = Some(Person { name: Some(name) });
//     match x {
//         // a 绑定到some(_) ，此处_ 有点范型的意思
//         Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
//         _ => {}
//     }
// }

// fn main(){
//     let x = 4;
//     let y = false;
//     match x {
//         // if y ... 为后置条件，被称为match guards
//         4 | 5 if y => println!("yes"), //  相当于 IF y AND (x IN List[4, 5])
//         _ => println!("no"),
//     }
// }

// fn main() {
//
//     let s = format!("{1}是个有着{0:>0width$}KG重，{height:?}cm高的大胖子",
//                     81, "wayslog", width=4, height=178);
//     print!("{}", s);
//
//     // format!("{:b}", 2);
//     // // 调用 `Binary` trait
//     // // Get : 10
//     // format!("{:?}", "Hello");
//     // // 调用 `Debug`
//     // // Get : "Hello"
// }

// trait HasArea {
//     fn area(&self) -> f64;
// }
//
// struct Circle {
//     x: f64,
//     y: f64,
//     radius: f64,
// }
// impl HasArea for Circle {
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * (self.radius * self.radius)
//     }
// }
//
// use std::fmt::Debug;
// // 用+ 链接所有实现的trait
// fn foo<T: Debug + Clone>(s: T) {
//     s.clone();
//     println!("{:?}", s);
// }

// use std::fmt::Debug;
// // where 从句
// fn foo<T, K>(x: T, y: K)
//     where T: Clone,
//           K: Clone + Debug {
//     x.clone();
//     y.clone();
//     println!("{:?}", y);
// }


// trait 继承
// trait Foo {
//     fn foo(&self);
// }
// trait FooBar : Foo {
//     fn foobar(&self);
// }
//
// struct Baz;
// impl Foo for Baz {
//     fn foo(&self) { println!("foo"); }
// }
// impl FooBar for Baz {
//     fn foobar(&self) { println!("foobar"); }
// }


// trait HasArea {
//     fn area(&self) -> f64;
// }
// impl HasArea for i32 {
//     fn area(&self) -> f64 {
//         *self as f64;
//         (self * self) as f64
//     }
// }
//
// fn main() {
//     // let c = Circle {
//     //     x: 0.0f64,
//     //     y: 0.0f64,
//     //     radius: 1.0f64,
//     // };
//     // println!("circle c has an area of {}", c.area());
//     println!("{}",5.area());
// }
//能被derive使用的trait包括：Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd


//x: &Foo其中x是一个trait对象，这里用指针是因为x可以是任意实现Foo的类型实例，内存大小并不确定，但指针的大小是固定的。
// trait Foo { fn method(&self) -> String; }
// impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
// impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }
// fn do_something(x: & Foo) {
//     x.method();
// }
//
// fn main() {
//     let x = "Hello".to_string();
//     do_something(&x);
//     let y = 8u8;
//     do_something(&y);
// }

// 如果一个trait方法是object safe的，它需要满足：
//
// 方法有Self: Sized约束， 或者
// 同时满足以下所有条件：
// 没有泛型参数
// 不是静态函数
// 除了self之外的其它参数和返回值不能使用Self类型
// 如果一个trait是object-safe的，它需要满足：
//
// 所有的方法都是object-safe的，并且
// trait 不要求 Self: Sized 约束

//fn main(){
//let mut num = 5;
// {
//     // mut 修饰的是一个闭包操作的环境的变量num,否则一个函数是可变的理所当然
//     let mut add_num = |x: i32| num += x;
//     add_num(5);
// }
// //true
// assert_eq!(10, num);

//     // 发生恐慌，num=5
//     let mut num = 10;
//     {
//         let mut add_num = move |x: i32| {
//             num += x;
//             println!("{}",num);
//         };
//         add_num(5);
//     }
//     // 发生恐慌，num=10, 用move 之后，离开作用域名之后，move 归还
//     assert_eq!(5, num);
//
// }

// //  闭包是一个trait
// //  F 闭包的类（范）型，原文叫做类型，感觉范性擦次准确
// fn call_with_one<F>(some_closure: F) -> i32
//     // Fn 是一个trait
//     where F: Fn(i32) -> i32 {
//     some_closure(1)
// }
//
// fn main() {
//     let answer = call_with_one(|x| x + 2);
//     assert_eq!(3, answer);
// }

// // 上面的例子可以改为
// fn call_with_one(some_closure: & dyn Fn(i32) -> i32) -> i32 {  // 注意 &Fn,这意味着 我们传递（也一定可以）的是一个闭包/函数的引用
//     some_closure(1)
// }
//
// fn main() {
//     let answer = call_with_one(&|x| x + 2);
//     assert_eq!(3, answer);
// }

// //闭包/函数 做为返回值
// fn factory() -> Box<dyn Fn(i32) -> i32> {
//     let num = 5;
//     Box::new(move |x| x + num)
// }
//  fn main() {
//     let f = factory();
//     let answer = f(1);
//     assert_eq!(6, answer);
//  }


// fn find(haystack: &str, needle: char) -> Option<usize> {
//     for (offset, c) in haystack.char_indices() {
//         if c == needle {
//             return Some(offset);
//         }
//     }
//     None
// }
// fn extension_explicit(file_name: &str) -> Option<&str> {
//     match find(file_name, '.') {
//         None => None,
//         Some(i) => Some(&file_name[i+1..]),
//     }
// }
// fn main() {
//     match extension_explicit("foo.rs") {
//         None => println!("no extension"),
//         Some(ext) =>  assert_eq!(ext, "rs"),
//     }
// }

//fn main() {

// let x = 5;
// let raw = &x as *const i32;
// let points_at = unsafe { *raw };
// println!("raw points at {}", points_at);

// 作用域
// let x: i32 = 1;
// {
//     let y: i32 =2;
//     println!("x = {}",x);
//     println!("y = {}",y);
// }
// println!("x = {}",x);

// 浅copy
// let s1 = "hello";
// let s2 = s1;
// println!("s2 ={}",s2);
// println!("s1 ={}",s1);
//
// // 深copy
// let s1 = String::from("hello");
// let s2 = s1.clone();
// println!("s2 ={}",s2);
// println!("s1 ={}",s1);


//}

// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
//
//
// async fn index() -> impl Responder {
//     "Hello world!"
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(
//             // prefixes all resources and routes attached to it...
//             web::scope("/cn ")
//                 // ...so this handles requests for `GET /app/index.html`
//                 .route("/index.html", web::get().to(index)),
//         );
//     })
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await
// }

// use std::fmt::{Display, Debug};

// 此处为什么不比较 两个参数的声明周期？ 因为其中一个的引用发生问题，比较的过程也会变得没有意义啊
// fn match_str<'a>(arg0: &'a str, arg1: &'a str) -> &'a str {
//     if arg0.len() >= arg1.len() {
//         arg0
//     } else {
//         arg1
//     }
// }
// #[derive(Debug)]
// pub struct Stu<'a> {
//     name: &'a str,
// }
//
// impl<'a> Stu<'a> {
//     //声明周期省略规则第三条
//     fn get_name(&self) -> &str {
//         &self.name
//     }
// }
// 以下是一个问题,假设  ShaoxianStu  是 学生的另一个这状态, 他的声明周期b 应该和a 是不一样的
// 当然在引用范围内，b和a 是重叠的  ,也就是只写全部生命周期就可以都是a   以下两个关于ShaoxianStu 的声明都是正确的
// #[derive(Debug)]
// struct ShaoxianStu<'b,'a>{
//     stu: &'b  Stu<'a>,     //类似继承
//     age: u32,
// }

// struct ShaoxianStu<'a> {
//     stu: &'a Stu<'a>,
//     age: u32,
// }

// 生命周期生命的目标是避免悬垂引用
// impl<'a> Stu<'a> {
//     fn get_shaoxian(stu: &'a Stu<'a>, age: u32) -> ShaoxianStu {
//         let shaoxian = ShaoxianStu {
//             stu,
//             age,
//         };
//         shaoxian
//     }
// }
//
//
// // 这两个函数的 arg1生命周期是可以省略的，
// //不管不参与的参数做了什么操作, 函数的生命周期只和 影响返回的引用或者借用参数相关
// fn match_bigger1<'a>(arg0: &'a i32, arg1: &mut i32) -> &'a i32 {
//     *arg1 = 3;
//     println!("arg1 = {}", *arg1);
//     arg0
// }
//
// fn match_bigger0<'a, 'b>(arg0: &'a i32, arg1: &'b mut i32) -> &'a i32 {
//     arg0
// }
//
// fn match_bigger2<'a>(arg0: &'a i32, arg1: &'a mut i32) -> &'a i32 {
//     if arg0 >= arg1 { arg0 } else { arg1 }
// }
//
// // 所有的字符，字符串字面值都是 'static
// fn test_fn<'a,T>(x: &'a str,y:&'a str,t:T) ->&'a str
// where T: Debug
// {
//
//     println!("t = {:#?}",t);
//     if x.len() >=y.len() { x } else { y }
//
// }
//
//
// fn main() {
//     let s1 = "hello";
//     let s2 = "world";
//     println!("the longer str is :{}", match_str(s1, s2));
//     let obj = Stu { name: "hanrui" };
//     println!("{}", obj.get_name());
//     let a = 1;
//     let mut b = 2;
//     println!("{}", match_bigger0(&a, &mut b));
//     println!("{}", match_bigger1(&a, &mut b));
//     println!("{}", match_bigger2(&a, &mut b));
//     let name = "张三";
//     let obj = Stu { name };     // obj 隐藏
//     let shaoxian = Stu::get_shaoxian(&obj, 10);
//     println!("age is {}", shaoxian.age);
//     println!("{}",test_fn(s1,s2,obj));
//
// }
//use std::fmt::{Display, Debug};
// fn main() {
//     let x =10;
//     let closer =|_x:u32|{
//         println!("{}",_x);
//     } ;
//     closer(10);
//
// }

// fn main(){
//    let a = vec![1,2,3,4,5];
//
//     // 类型其实就是一种 trait
//     //  let closer  = |i:u32|{   //报错
//
//     let closer  =  |i|  {
//
//        //1  unwrap 是将错误抛出，调用者去处理
//        //   a.get(i).unwrap()
//        //   a.get(i)?   //直接报错
//        //   a.get(i)   // 返回一个None,或者Some(X)
//        //   Some(a.get(i))? //返回Some(X) 或者 None
//        //   Some(a.get(i)) //返回Some(Some(x))  Some(None)
//        //   所以正常情况返回值 是 Some(fn XXXX(closure_param))? 或者是 fn XXXX(closure_param).unwrap()
//        //2  闭包 没有实现 std::fmt:Display::fmt
//        //3  函数体为空，没有返回值，返回闭包一个()
//        //   Some(a.get(i))?
//
//        // let  c = Some(8)     ;
//        // match c {
//        //     Some(v)=>v,
//        //     None=>0,
//        //   }
//    };
//     println!("{:?}",closer(1));
// }


// fn main(){
//
//     let vet = vec![1,2,3,4,5,6];
//
//     let c = &vet[0];
//
//     println!("c = {} ---- vet = {:?}",c,vet);
//
//     let closer= |x| {
//
//         //   let a = vet[0];
//         //   println!("a = {}",a);
//         //   let b = &vet[0];
//         //   println!("b = {}",b);
//         //
//         //  // let c:i32 = vet[10];
//         //  // println!("c = {}",c); //越界后只能 抛出panic
//         //
//         //  let d: &i32 =&vet[0];
//         //  println!("d = {}",d);
//         //
//         // // let d: &i32 =&vet[10];
//         // // println!("d = {}",d);
//
//
//         // let e = vet.get(x);
//         // println!("e = {:?}", e); // x >5 时 抛出错误
//
//         let f:Option<&i32> = vet.get(x);
//         match f {
//            Some(v)=>{println!("v = {}",v)},
//            None=>{println!("v = {}",0)},
//         }
//     };
//     closer(1);
//     // iter 是惰性的
//     for e in vet.iter(){
//         println!("e = {}",e);
//     }
//
// }

//use std::iter::Iterator;
// trait Iterator{
//      type Item;
//      fn next(mut self) ->Option<Self::Item>;
// }
// fn main(){
//     let vet = vec![1,2,3];
//     let mut vi = vet.iter(); //可变引用 用iter_mut()
//     if let Some(v) = vi.next(){
//         println!("{}",v);
//     }else{
//         println!("end");
//     }
//     if let Some(v) = vi.next(){
//         println!("{}",v);
//     }else{
//         println!("end");
//     }
//     if let Some(v) = vi.next(){
//         println!("{}",v);
//     }else{
//         println!("end");
//     }
//     if let Some(v) = vi.next(){
//         println!("{}",v);
//     }else{
//         println!("end");
//     }
//     println!("sssss {}",vi.len());
//
//     let vet =vec![1,2,3];
//     let vi = vet.iter();
//     // 消费适配器
//     let t: i32 = vi.sum();
//     println!("sum is {}", t);
//     //vi 在消费时被move
//     //println!("sssss {}",vi.len()); //报错
//
//     let vet =vec![1,2,3];
//     let vi = vet.iter();
//
//     // 迭代适配器用法
//     let y:Vec<_> = vi.map(|x| x+1).collect();
//     println!("{:?}",y);
//
// }

// fn main() {
//   let v = vec![10,20,30];
//   let x:Vec<_> = v.into_iter().map(|x|x+1).collect();
//   println!("{:?}",x);
//   let y  =x.into_iter();
//   let z:Vec<_> =y.map(|x|x+4).collect();
//   println!("{:?}",z);
//   //y 被消费掉
//   //let m:Vec<_> = y.filter(|x|*x>25).collect(); //报错
//   let m:Vec<_>= z.into_iter().filter(|x|*x>25).collect();
//   println!("{:?}",m);
// }

// struct Count {
//     count: i32,
// }
//
// impl Count {
//     fn new() -> Count {
//         Count { count: 0 }
//     }
// }

// impl Iterator for Count {
//     type Item = i32;  //别名？什么意思
//     // 唯一要求实现的方法
//     fn next(&mut self) -> Option<Self::Item> {  //Self::Item 什么意思
//         self.count += 1;
//         if self.count < 6 {
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }
//
// fn main() {
//     let mut count = Count::new();
//     for i in 0..7 {
//         if let Some(v) = count.next() {
//             println!("when index = {}, value = {}", i, v);
//         } else {
//             println!("when index = {}, value = None", i);
//             break; // 没有分号也行
//         }
//     }
// }

// cargo run/build --release
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use rust_demo::com::lssoft::common::People;
use hello::com::lssoft::common::People;


fn main() {
    for _ in 0..10 {
        println!("hello cargo!");
    }

    let mut hasher = Sha3::sha3_256();
    hasher.input_str("hello");
    let r = hasher.result_str();
    println!("{}", r);
    let  mut p_name = String::from("hanrui");
    let  mut p = People {
        name: & mut p_name,
    };
    println!("{:#?}",p);
    let  mut m_name = String::from("lisi");
    p.modify_name(& mut m_name);
    println!("{:#?}", p);
}