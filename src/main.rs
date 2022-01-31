/* to print any thing we use println!

fn main() {
    println!("hellow world");
    println!("i'm rajan")
}*/

/* to assing any value with an given value is

fn main() {
    let x = 5 + 5;
    println!("value of x is {}", x);
}
*/
/*   */
/* fn main() {
    println!("{} value of a. ", 31)
}*/

/*   */
/* fn main() {
    let lang: &str = "Rust";
    let feature: &str = "string interpolation syntax";
    println!("Will {} ever, support {} like this?", lang, feature);
}
*/
/* */
/*fn main() {
    println!("{number: >width$}", number = 1, width = 25);
}*/
/*fn main() {
    println!("hi {0} this is {1} and hi {1} this is {0}", "bob", "ram");
}
fn main() {
    println!("{0}{1}{2}",1, 2, 3);
}*/

//function sikako//

// fn main(){
//     let number = [0,1,2,3,4,5,6,7,8,10];
//     print!("first number is {}",number[0]);
//     print!("last number is{}",number[9]);
//     main1();
//     main2();
// }
// fn main1(){
//     println!("this is my book");
// }
// fn main2(){
//     println!("this is second line");
// }   let c=sum(a:i32+b:i32);

//              creating function

// fn main(){
//     let a=sum(1,2);
//     println!("the value of sum is {}",a);
// }
// fn sum(x:i32,y:i32)->i32{
//     let z=x+y;
//     return z;
// }

// using if else statement

// fn main(){
//     let a=10;
//     if a%3==0{
//         println!("it can be div by 3");
//     }
//     else if a%2==0{
//         println!("it can by div by 2");
//     }
//     else if a%1==0{
//         println!("it can be div by 1");
//     }
//     else{
//         println!("it cant be div by 1,2,3")
//     }
// }

//                                                             using loop statement

// fn main(){
//     let n:i32=50;
//     let mut count =0;
//     loop {
//        println!("printing n {}",n);
//        count+=1;
//        if count>30{
//            println!("the count is equal  to {}",30);
//            break;
//        }
//     }
// }
//                                                             using break and continue statement in loop

// fn main(){
//     let mut n=0;
//     loop {
//         n=n+1;
//     {
//         if n>10{
//             break;
//         }
//         else if n==5{
//             continue;
//         }
//     }
//         println!("the value of n are {}",n);

//     }
// }

//                                              using while loop statement (while pani yeuta loop nai ho tehi vayara hami loop use garna pardaina )

// fn main(){
//     let mut n=1;
//   while n<=50 {
//       n=n+1;
//       if n%5==0{
//         println!("value of n are {}",n);
//       }

//   }
// }

// want to print certain number of between
// fn main(){
//     let mut n=30;
//     while n<=70 {
//         n=n+1;
//         println!("value of n are {}",n);

//     }
// }

//another method to print certain between method using for loop

// fn main(){
//     let number=30..71;
//    for i in number {
//     println!("numbers are {}",i);
//    }

// }

// print decending order from 10 to 0
// fn main(){
//     let number=(1..10).rev();
//     for i in number {
//         println!("the value of n are {}",i);
//     }
// }

// another method is

// fn main(){
//     for i in (1..10).rev() {
//         println!("the value of i are {}",i)
//     }

// }

//to print name of animals using for statement

// fn main(){
//     let animals=vec!["dinosorous","lion","tiger"];
//     for i in animals.iter() {
//         println!("name of animals are {}",i);
//     }
// }

// if want to count then we use enumerate which count the number and  index for the index

// fn main(){
//     for (index,i) in vec!["rat","bat","cat"].iter().enumerate() {
//         println!("the name of the animals and their index {} are {}",index, i);
//     }
// }

//defining function outside the main function

// fn main(){
//     fun_a();
//     fun_b(32,64);
// }
// fn fun_a(){
//     println!("this is main section");
// }
// fn fun_b(a:i64,b:i64){
//     println!("the value of a {} and b {} ",a,b);
// }

//ownership in rust programming

// fn main(){
//     let mut x  = 10;
//     let y=x;
//     println!("the value of x is {} and value of y is {}",x,y);
//     x=5;
//     println!("the value of x is {}",x);
// }

//sum of two digit but different way

// fn main(){
//     let x=5;
//     let y=x;
//     println!("the value of x is {} and y is  {}", x ,y);
//     let x = 20;
//     println!("the value of x is {}",x);
//     sum (x,y);
// }
// fn sum(x:i32,y:i32){
//     let sum  =x+y;
//     println!("the sum is {}",sum);
// }

//concidanate two string

// fn main(){
//     let mut s=String::from("hellow");
//     s.push_str(" world");
//     s.push_str("  this is football");
//     println!("{}",s);
// }

//ownership move

// fn main(){
//     let mut s=String::from("hellow");
//     s.push_str(" world");
//     println!("{}",s);
//     let s1=s;        // if i want to get clone of s then we use              let s1: string =s.clone();
//     println!("s1={}",s1);
//     println!("s={}",s);
// }
//clone formation                          //string ko lagi clone because we cannot find the size of string during compile time
// int type ko lagi copy because we can find fixed size during compile time

// fn main(){
//     let mut  s=String::from("hi");
//     s.push_str("  america");
//     println!("{}",s);
//     let s1=s.clone();      // mathi same data lai 2 la access garako xa vana tala chi same data ko aarko clone banauxa ra clone data lai access garxa
//     println!("s1={}",s1);                 // naki pahila vako data lai access garxa. yesma clone data vako vayra data ko ownership change vako xaina.
//     println!("s={}",s);

// }

// STRUCTURE DEFINATION:- structure ma various types data haru sangai rakhna milxa integer, float, variable , double etc.

// use std::char;

// fn main(){
//     struct User {
//         username: char,              char ko thau ma sting pani rakhna milxa tara tesko thau ma string::from(" ");
//         age:i64,
//         height: f64,
//         email:String,
//     }
//     let hari=User{
//         username:  char::from(1),
//         age: 32,
//         height: 5.6,
//         email: String::from("rajanbhandary@gmail.com"),

//     };
//     let shyam=User{
//         username:char::from(2),
//         age: 16,
//         height: 5.0,
//         email: String::from("email@gmail.com"),
//     };
//     println!("email={}",hari.email);
//     println!("height={}",hari.height);
//     println!("username={}",hari.username);
//     println!("age={}",hari.age);
//     println!("email={}",shyam.email);
// }

// calculate the length of string

// fn main(){
//     let s=String::from("rajan");

//     println!("{} and its length is {}",s,s.len());
// }

// fn main(){
//     let s=String::from("this is my name");
//     println!("{} length is {}",s,len);
// }

// fn main(){
//     let v=vec![0,9];
//     println!("{}",v[1]);
// }

// fn main(){
//     struct User {
//         username: char,
//         age:i64,
//         height: f64,
//         email:String,
//     }
//     let hari=User{
//         username:  char::from(1),
//         age: 32,
//         height: 5.6,
//         email: String::from("rajanbhandary@gmail.com"),

//     };
//     let shyam=User{
//         username:char::from(2),
//         age: 16,
//         height: 5.0,
//         email: String::from("email@gmail.com"),

//     let ram=shortform(em:String::from("user@gmail.com"), name:String::from("username"))
// };
//     println!("badfbas name {}",ram.email)
//     println!("email={}",hari.email);
//     println!("height={}",hari.height);
//     println!("username={}",hari.username);
//     println!("age={}",hari.age);
//     println!("email={}",shyam.email);
// }
// fn shortform(em:String,name:string)->User{
//     User{
//         email:em,
//         username:name,
//         age:i64,
//         height:f64,
//     }
// }

// fn functionname(name:&str,age:i32){         //defining a function
//     let next_time=age+1;
//     println!("hi {} next year you will be {}",name,next_time);
// }
// fn main(){        // calling a function
//     functionname("jack",3);
// }

//return value from function

// fn square(num:i32)->i32{
//     num*num
// }
// fn main(){
//     println!("the multiple of two number is {}",square(2));
// }

// add of two number

// fn main(){
//     let a:i32=2147483647;
//     let b:i32=2147483647;
//     let c:i64=a+b;
//     println!("sum {}",c);
// }

// fn main(){
//      let name= String::from("rt");
//         if name.trim()=="rust"{
//             println!("guess is right");
//         }
//         else {
//             println!(r#"guess is wrong"#);
//         }
//     }

// fn sum(a:i32,b:i32)-> i32{
//    return a+b;
// }

// fn main(){
// let c=sum(45,56);
// print!("the sum is {}",c);
// }

// fn main(){
//     let _number=3;
//     if true{
//         print!("number is three");

//     }
//     else {
//         print!("false number is not three");
//     }
// }

// fn main(){
//     let continous=String::from("a");
//     let zigzag=String::from("b");
//     match (continous,zigzag) {
//         (a,b)=>println!("this is good"),
//         (b,a)=>println!("this is bad"),
//     }
// // }
// enum Color{
//     red,
//     yellow,
//     blue,
// }
// fn my_color(color:Color){
//     match color {
//         Color::blue=> println!("this is color blue"),
//         Color::yellow=>println!("this is color yellow"),
//         Color::red=>println!("this is color red"),
//     }
// }
// fn main(){
//     my_color(Color::blue);
//     my_color(Color::yellow);
// }

// struct Grossaryitems{
//     stock:i32,
//     price:f32,
// }

// fn main(){
//     let nepal= Grossaryitems{
//          stock:32,
//          price:18.0,
//     };
//     println!("this is the price {:?}",nepal.price);
//     println!("this is the price {:?}",nepal.stock);
// }

// organizing similar data using struct

// enum Flavor{
//     sweet,
//     sparkling,
//     fruity,
// }
// struct Drink{
//     flavor: Flavor,
//     fluid_oz : f32,
// }
// fn print_drink(drink: Drink){
//     match drink.flavor {
//     Flavor::fruity=>println!("this is fruity"),
//     Flavor::sparkling=>println!("this is sparkling"),
//     Flavor::sweet=>println!("this is sweet"),
//     }
//     println!("drink oz is {:?}",drink.fluid_oz);
// }
// fn main(){
//  let cherry= Drink{
//     flavor:Flavor::sweet,
//     fluid_oz:16.0,
//  };
//  print_drink(cherry);

//  let juice= Drink{
//     flavor:Flavor::sparkling,
//     fluid_oz:16.0,
//  };
//  print_drink(juice);

//  let mango= Drink{
//     flavor:Flavor::fruity,
//     fluid_oz:16.0,
//  };
//  print_drink(mango);

// }

// enum Light {
//     Bright,
//     Dull
// }
// fn display_light(light:Light){
//     match light {
//         Light::Bright=>println!("light is bright"),
//         Light::Dull=>println!("light is dull"),
//     }
// }
// fn main(){
//     let dull=Light::Dull;
//     display_light(dull);
// }

// struct grossary{
//   quantity:i32,
//   id_number:i32,
// }
// fn display_quant(item:&grossary){
//     println!("the quant of rice is {}",item.quantity);
// }
// fn id_numbe(item:&grossary){
//     println!("the id of rice is {}",item.id_number);
// }
// fn main(){
//     let rice=grossary{
//         quantity:10,
//         id_number:9,
//     };
//     display_quant(&rice);
//     display_quant(&rice);
//     display_quant(&rice);
//     display_quant(&rice);
// }

// using self

// struct temperature{
//     degree:f32,
// }
// impl temperature {
//     fn show_tempr(&self){
//         println!("the tempr is {}",self.degree);
//     }
// }
// fn main(){
//     let fever = temperature{
//         degree:37.0,
//     };
//     fever.show_tempr();
// }

// using Self

// struct temperature{
//     degree:f32,
// }

// impl temperature {
//     fn freezing()->Self{
//         Self{degree:37.0}
//     }
//     fn show_temp(&self){
//         println!("tempr is {}",self.degree)
//     }
// }
// fn main(){
//     let cold=temperature::freezing();
//     cold.show_temp();
// }
// fn main(){
//     let mut marks=Vec::new();
//     marks.push(30);
//     marks.push(30);
//     marks.push(30);
//     marks.push(30);
//     for thing in marks {
//         println!("the marks are {}",thing);
//     }

// }

// fn main(){
//     let marks=vec![10,20,30,40];
//     for num in &marks {
//         match num {
//             30=>println!("thirty"),
//             _=>println!("{}",num),
//         }
//     }
//     println!("number of lenght is {:?}",marks.len());
// }

// fn print_it(data:&str){
//     println!("{}",data );
// }

// fn main(){
//     let name=String::from("this is my name");
//     print_it("this is another string");
//     print_it(&name);
//     println!("{}",name);

// }

// struct character{
//     name:String,
//     age:i32,
//     color:String,
// }

// fn name(data:&str){
//     println!("the name is {:?}",data);
// }

// fn color(data:&str){
//     println!("the color is {:?}",data);
// }

// fn main(){
//     let item=vec![
//         character{
//             name:String::from("rajan"),
//             age:21,
//             color:String::from("red"),
//         },
//         character{
//             name:String::from("ram"),
//             age:20,
//             color:String::from("red"),
//         },
//         character{
//             name:String::from("shyam"),
//             age:19,
//             color:String::from("red"),
//         },
//     ];
//     for select in item {
//         if select.age==20 {
//             name(&select.name);
//             color(&select.color);
//         };
//         if select.age<20 {
//             name(&select.name);
//             color(&select.color);
//         };
//         if select.age>20 {
//             name(&select.name);
//             color(&select.color);
//         };
//     }
// }

// #[derive(Debug)]
// enum Position{
//     manager,supervisier,worker
// }
// #[derive(Debug)]
// struct employee{
//     position:Position,
//     working_hours:f32,
// }
// fn main(){
//     let num=employee{
//        position:Position::supervisier,
//        working_hours:32.0,
//     };
//    println!("{:?}",num);
//     println!("{}",num.working_hours);
// }

// enum Ticket{
//     Backstage(f64,String),
//     vip(f64,String),
//     standard(f64),
// }

// fn main(){
//     let ticket=vec![
//         Ticket::Backstage(50.0,String::from("rajan")),
//         Ticket::vip(50.0,String::from("shyam")),
//         Ticket::standard(50.0),
//     ];
//     for show in ticket {
//         match show {
//             Ticket::Backstage(price,holder)=>println!("holder {},price {}",holder,price),
//             Ticket::vip(price,holder)=>println!("holder {},price {}",holder,price),
//             Ticket::standard(price)=>println!("price {}",price),
//         };
//     }
// }
//option

// struct marks{
//     age:Option<i32>,
//     email:String,
// }
// fn main(){
//     let bench=marks{
//         age:Some(23),
//         email:String::from("rajan2rajan"),
//     };
//     let best=marks{
//         age:None,
//         email:String::from("rajanbhandari"),
//     };
//     match bench.age {
//         Some(age)=>println!("he is age of {:?}",age),
//         _=>println!("he has no age"), or
//         None=>println!("he has no age"),
//     }

// }

// struct Survay{
//     q1:Option<i32>,     //age
//     q2:Option<bool>,    //marriage
//     q3:Option<String>,  //name
// }
// fn main(){
// let response=Survay{
//     q1:Some(13),
//     q2:None,
//     q3:Some(String::from("rajan")),
// };

// match response.q1{
//     Some(ans)=>println!("the age is {}",ans),
//     None=>println!("blank"),

// };

// }

//result

// struct Customer {
//     age: i32,
// }
// fn try_purchase(customer: &Customer) -> Result<(), String> {
//     if customer.age < 21 {
//         Err(String::from("you are underage"))
//     } else {
//         Ok(())
//     }
// }

// fn main() {
//     let rajan = Customer { age: 32 };
//     let sure = try_purchase(&rajan);
//     println!("{:?}", sure);
// }

// hash map
// #[derive(Debug)]
// struct Content {
//     content: String,
// }

// fn main() {
//     let mut locker = HashMap::new();
//     locker.insert(
//         1,
//         Content {
//             content: "shirts".to_owned(),
//         },
//     );
//     locker.insert(
//         2,
//         Content {
//             content: "shirts".to_owned(),
//         },
//     );
//     locker.insert(
//         3,
//         Content {
//             content: "shirts".to_owned(),
//         },
//     );
//     for (HashMap, Content) in locker.iter() {
//         println!("{:?}, {:?}", HashMap, Content);
//     }
// }

// #[derive(Debug)]
// struct equipment {
//     chairs: i32,
//     beds: i32,
//     tables: i32,
//     Couches: i32,
// }
// fn main() {
//     let mut items = HashMap::new();
//     items.insert(
//         1,
//         equipment {
//             chairs: 1,
//             beds: 2,
//             tables: 3,
//             Couches: 4,
//         },
//     );
//     items.insert(
//         2,
//         equipment {
//             chairs: 1,
//             beds: 1,
//             tables: 1,
//             Couches: 1,
//         },
//     );
//     items.insert(
//         3,
//         equipment {
//             chairs: 4,
//             beds: 3,
//             tables: 2,
//             Couches: 1,
//         },
//     );
//     for (k, v) in items {
//         println!("{:?},{:?}", k, v);
//     }

// // }

// fn upper_case(word: &str) -> String {
//     word.to_uppercase()
// }
// fn main() {}

// #[cfg(test)]
// mod test {
//     use crate::*;
//     #[test]
//     fn caps() {
//         let result = upper_case("hello");
//         let expected = String::from("HELLO");
//         assert_eq!(result, expected);
//         assert_eq!(result, expected, "result are correct");
//     }
// }

//using crates for more  lib.io with classification.
// use chrono::prelude::*;

// fn main() {
//     let local: DateTime<Local> = Local::now();
//     println!("{:?}", local.format("%Y-%m-%d %H:%M:%S").to_string());
// }

// fn add(a: i32, b: i32) {
//     a + b;
// }
// fn main() {
//     #[cfg(test)]
//     mod test {
//         use crate::*;
//         #[test]
//         fn compare() {
//             let x = add(5, 6);
//             let y = 0;
//             assert_eq!(x, y, "the expected number is 11");
//         }
//     }
// }
//                              take two string from user

// use ::std::io;
// fn get_data() -> io::Result<String> {
//     let mut buffer = String::new();
//     io::stdin().read_line(&mut buffer);
//     Ok(buffer.trim().to_owned())
// }
// fn main() {
//     let mut all_input = vec![];
//     let mut times_input = 0;
//     while times_input < 2 {
//         match get_data() {
//             Ok(words) => {
//                 all_input.push(words);
//                 times_input += 1;
//             }
//             Err(e) => println!("error occur{:?}", e),
//         }
//     }
//     for input in all_input {
//         println!(
//             "orginal {:?} capitalization {:?}",
//             input,
//             input.to_uppercase()
//         );
//     }
// }

// use ::std::io;

// fn get_input() -> io::Result<String> {
//     let mut buffer = String::new();
//     io::stdin().read_line(&mut buffer);
//     Ok(buffer.trim().to_owned())
// }
// fn main() {
//     let mut all_input = vec![];
//     let mut times_input = 0;
//     while times_input < 2 {
//         match get_input() {
//             Ok(words) => {
//                 all_input.push(words);
//                 times_input += 1;
//             }
//             Err(e) => println!("the error is {:?}", e),
//         }
//     }
//     for input in all_input {
//         println!("orginal {:?} capitlize {:?}", input, input.to_uppercase());
//     }
// }
// use std::io;
// enum powerstate {
//     Off,
//     Sleep,
//     Reboot,
//     Shutdown,
//     Hibernate,
// }
// impl powerstate {
//     fn new(state: &str) -> Option<powerstate> {
//         let state = state.trim().to_lowercase();
//         match state.as_str() {
//             "off" => Some(powerstate::Off),
//             "sleep" => Some(powerstate::Sleep),
//             "reboot" => Some(powerstate::Reboot),
//             "shutdown" => Some(powerstate::Shutdown),
//             "hibernate" => Some(powerstate::Hibernate),
//             _ => None,
//         }
//     }
//     fn print_power_action(state: powerstate) {
//         match state {
//             off => println!("turning off"),
//             sleep => println!("sleeping"),
//             reboot => println!("reboot"),
//             shutdown => println!("shutdown"),
//             hibernate => println!("hubernating"),
//         }
//     }
// }
// fn main() {
//     let mut buffer = String::new();
//     let user_input_status = io::stdin().read_line(&mut buffer);
//     if user_input_status.is_ok() {
//         match powerstate::new(&buffer) {
//             Some(state) => powerstate::print_power_action(state),
//             None => println!("invalid option"),
//         }
//     } else {
//         println!("error reading input");
// //     }
// }

// new project

// use ::std::io;
// #[derive(Debug, Clone)]
// pub struct Bill {
//     name: String,
//     amount: f64,
// }
// pub struct Bills {
//     inner: Vec<Bill>,
// }
// impl Bills {
//     fn new() -> Self {
//         Self { inner: vec![] }
//     }
//     fn add(&mut self, bill: Bill) {
//         //showing parameter self refers to inner.
//         // we are taking the refrence of inner value
//         self.inner.push(bill); //and bill is denoted by Bill which is moved to Vec<Bill>.
//     }
//     fn get_all(&self) -> Vec<&Bill> {
//         self.inner.iter().collect()
//     }
// }

// fn get_input() -> Option<String> {
//     let mut buffer = String::new();
//     while io::stdin().read_line(&mut &mut buffer).is_err() {
//         println!("put valid data");
//     }
//     let input = buffer.trim().to_owned(); //buffer are side and are owned by input.
//     if &input == "" {
//         //if presesd enter without any data.
//         None
//     } else {
//         Some(input)
//     }
// }
// fn get_bill_amount() -> Option<f64> {
//     //option use some or non
//     use crate::*;
//     println!("Amount :-");
//     loop {
//         let input = match get_input() {
//             Some(input) => input,
//             None => return None,
//         };
//         if input == "" {
//             // simply pressed enter
//             return None;
//         } //Parsing is to read the value of one object to convert it to another type.
//         let parsed_input: Result<f64, _> = input.parse(); //result use ok and error
//         match parsed_input {
//             Ok(amount) => return Some(amount),
//             Err(_) => println!("please a number"),
//         }
//     }
// }
// enum Mainmenu {
//     AddBill,
//     ViewBill,
// }
// mod menu {
//     use crate::*; //or {get_bill_amount, get_input, Bill, Bills}
//     pub fn add_bill(bills: &mut Bills) {
//         println!("Bill name");
//         let name = match get_input() {
//             Some(input) => input,
//             None => return,
//         };
//         let amount = match get_bill_amount() {
//             Some(amount) => amount,
//             None => return,
//         };
//         let bill = Bill { name, amount };
//         bills.add(bill);
//         println!("bill added");
//     }
//     pub fn view_bills(bills: &Bills) {
//         for bill in bills.get_all() {
//             println!("{:?}", bill);
//         }
//     }
// }
// impl Mainmenu {
//     fn show() {
//         println!("");
//         println!("==bill manager==");
//         println!("1. Add Bills");
//         println!("2. View Bills");
//         println!("");
//         println!("Enter selection line: ");
//     }
//     fn from_str(input: &str) -> Option<Mainmenu> {
//         match input {
//             "1" => Some(Self::AddBill),
//             "2" => Some(Self::ViewBill),
//             _ => None,
//         }
//     }
// }
// fn main() {
//     let mut bills = Bills::new();
//     loop {
//         Mainmenu::show();
//         let input = get_input().expect("no data entered");
//         match Mainmenu::from_str(input.as_str()) {
//             Some(Mainmenu::AddBill) => menu::add_bill(&mut bills),
//             Some(Mainmenu::ViewBill) => menu::view_bills(&mut bills),
//             None => return,
//         }
//     }
// }

//trait,

// trait Fall {
//     fn hit_ground(&self);
// }
// struct cat;
// impl Fall for cat {
//     fn hit_ground(&self) {
//         println!("cat simply walk away");
//     }
// }
// struct vase;
// impl Fall for vase {
//     fn hit_ground(&self) {
//         println!("vase are broken");
//     }
// }
// fn fall(thing: impl Fall) {
//     thing.hit_ground();
// }
// fn main() {
//     fall(cat);
//     fall(vase);
// }

// trait Perimetre {
//     fn perimeter(&self) -> i32;
// }
// struct square {
//     side: i32,
// }
// struct Triangle {
//     side_a: i32,
//     side_b: i32,
//     side_c: i32,
// }
// impl Perimetre for square {
//     fn perimeter(&self) -> i32 {
//         self.side * 4
//     }
// }
// impl Perimetre for Triangle {
//     fn perimeter(&self) -> i32 {
//         self.side_a + self.side_b + self.side_c
//     }
// }
// fn print_perimeter(shape: impl Perimetre) {
//     let perimeter = shape.perimeter();
//     println!("{:?}", perimeter);
// }
// fn main() {
//     let Square = square { side: 5 };
//     let triangle = Triangle {
//         side_a: 5,
//         side_b: 5,
//         side_c: 5,
//     };
//     print_perimeter(Square);
//     print_perimeter(triangle);
// }

//trait with multiple function

// trait CheckIn {
//     fn checkin(&self); // for checking when passenger arrive,checking cargo
//     fn process(&self); //taking loggage, placing plane
// }
// struct Pilot;
// impl CheckIn for Pilot {
//     fn checkin(&self) {
//         println!("checked in as pilot ");
//     }
//     fn process(&self) {
//         println!("pilot enter cockpit");
//     }
// }

// struct Passenger;
// impl CheckIn for Passenger {
//     fn checkin(&self) {
//         println!("checked in as pilot ");
//     }
//     fn process(&self) {
//         println!("passenger takes a seat ");
//     }
// }

// struct Cargo;
// impl CheckIn for Cargo {
//     fn checkin(&self) {
//         println!("checked in cargo ");
//     }
//     fn process(&self) {
//         println!("cargo moved to storage ");
//     }
// }
// fn process_item<T: CheckIn>(item: T) {
//     item.checkin();
//     item.process();
// }
// fn main() {
//     let paul = Pilot;
//     let rajan = Passenger;
//     let cargol = Cargo;
//     let cargo2 = Cargo;
//     process_item(paul);
//     process_item(rajan);
//     process_item(cargo2);
//     process_item(cargol);
// }

//storing multiple type of data in single space.

// trait Sale {
//     fn amount(&self) -> f64;
// }
// struct Fullsales(f64);
// impl Sale for Fullsales {
//     fn amount(&self) -> f64 {
//         self.0
//     }
// }
// struct Onedollaroff(f64);
// impl Sale for Onedollaroff {
//     fn amount(&self) -> f64 {
//         self.0 - 1.0
//     }
// }
// struct tenpercentageoff(f64);
// impl Sale for tenpercentageoff {
//     fn amount(&self) -> f64 {
//         self.0 * 0.9
//     }
// }
// fn calculate_rev(sales: &Vec<Box<dyn Sale>>) -> f64 {
//     sales.iter().map(|sale| sale.amount()).sum()
// }
// fn main() {
//     let price = 20.0;
//     let regular = Box::new(Fullsales(price));
//     let coupon = Box::new(Onedollaroff(price));
//     let promo = Box::new(tenpercentageoff(price));
//     let sales: Vec<Box<dyn Sale>> = vec![regular, coupon, promo];
//     let revenue = calculate_rev(&sales);
//     println!("the sales {:?}", revenue);
// }

// trait Material {
//     fn cost_per_sq_meter(&self) -> f64;
//     fn square_meter(&self) -> f64;
//     fn total_cost(&self) -> f64 {
//         self.cost_per_sq_meter() * self.square_meter()
//     }
// }
// struct Carpet(f64);
// impl Material for Carpet {
//     fn cost_per_sq_meter(&self) -> f64 {
//         10.0
//     }
//     fn square_meter(&self) -> f64 {
//         self.0
//     }
// }
// struct Tile(f64);
// impl Material for Tile {
//     fn cost_per_sq_meter(&self) -> f64 {
//         15.0
//     }
//     fn square_meter(&self) -> f64 {
//         self.0
//     }
// }
// struct Wood(f64);
// impl Material for Wood {
//     fn cost_per_sq_meter(&self) -> f64 {
//         20.0
//     }
//     fn square_meter(&self) -> f64 {
//         self.0
//     }
// }

// fn total_cost(material: &Vec<Box<dyn Material>>) -> f64 {
//     material.iter().map(|mat| mat.total_cost()).sum()
// }
// fn main() {
//     let carpet = Box::new(Carpet(20.0));
//     let tiles = Box::new(Tile(10.0));
//     let wood = Box::new(Wood(30.0));

//     let materials: Vec<Box<dyn Material>> = vec![carpet, tiles, wood];
//     let materialss = total_cost(&materials);
//     println!("{:?}", materialss);
// }

// fn longest<'a>(one: &'a str, two: &'a str) -> &'a str {
//     if two > one {
//         two
//     } else {
//         one
//     }
// }
// fn main() {
//     let short = "hello";
//     let long = "tjis is long";
//     println!("{:?}", longest(short, long));
// }

//custome error handling
// u
// #[derive(Debug, Clone, Copy)]
// struct Neverzero(i32);
// impl Neverzero {
//     pub fn new(i: i32) -> Result<Self, String> {
//         if i == 0 {
//             Err(String::from("error occur"))
//         } else {
//             Ok(Self(i))
//         }
//     }
// }
// fn divide(a: i32, b: Neverzero) -> i32 {
//     let b = b.0;
//     a / b
// }
// fn main() {
//     match Neverzero::new(5) {
//         Ok(nz) => println!("{:?}", divide(10, nz)),
//         Err(e) => println!("{:?}", e),
//     }
// }

// enum TreasureItem {
//     Gold,
//     SuperPower,
// }

// struct TreasureChest {
//     content: TreasureItem,
//     amount: usize,
// }

// struct Pressure(u16);

// enum BrickStyle {
//     Dungeon,
//     Gray,
//     Red,
// }

// enum Tile {
//     Brick(BrickStyle),
//     Dirt,
//     Grass,
//     Sand,
//     Treasure(TreasureChest),
//     Water(Pressure),
//     Wood,
// }
// fn print_tile(tile: Tile) {
//     use Tile::*;
//     match tile {
//         // Brick(brick @ BrickStyle::Dungeon | brick @ BrickStyle::Gray) => {
//         //     println!("color can be {:?}", brick)}
//         Brick(BrickStyle::Dungeon | BrickStyle::Red) => println!("color can be of anything"),
//         Dirt | Grass | Sand | Wood => println!("other tiles"),
//         //* If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
//         // * Everything else shoud not print any messages
//         Treasure(TreasureChest {
//             content: Gold,
//             amount: 1000..,
//         }) => println!("gold a lots"),
//         Water(pressure) if pressure.0 < 10 => println!("water pressure is {}", pressure.0),
//         Water(pressure) if pressure.0 > 10 => println!("high pressure {}", pressure.0),
//         _ => (),
//     }
// }
// fn main() {
//     let title = Tile::Brick(BrickStyle::Red);
//     print_tile(title);
//     let title = Tile::Treasure(TreasureChest {
//         content: TreasureItem::Gold,
//         amount: 500,
//     });
//     print_tile(title);
// }

// fn data() -> &'static [u64] {
//     &[5, 5, 4, 4, 3, 3, 1]
// }
// fn process_chunk(data: &[u64]) {
//     match data {
//         [lhs, rhs] => println!("{}+{}={}", lhs, rhs, (lhs + rhs)),
//         [single] => println!("single value is {}", single),
//         [] => println!("data steram complete"),
//         [..] => unreachable!("this code should never execute"), //this code will never excute.
//     }
// }

// fn main() {
//     // `stream` is an iterator of Option<&[u64]>
//     let stream = data().chunks(2);
//     for chunk in stream {
//         process_chunk(chunk);
//     }
// }

// struct Temperature {
//     degree: f64,
// }
// impl Temperature {
//     fn show_temp(&self) {
//         println!("the temper is {:?}", self.degree);
//     }
// }
// fn main() {
//     let hot = Temperature { degree: 98.0 };
//     hot.show_temp();
// }
// struct Temperature {
//     degree: f64,
// }
// impl Temperature {
//     fn freezing() -> Self {
//         Self { degree: 32.0 }
//     }
//     fn show_temp(&self) {
//         println!("the temper is {:?}", self.degree);
//     }
// }

// fn main() {
//     let cold = Temperature::freezing();
//     cold.show_temp();
// // }
// #[derive(Debug)]
// enum Color {
//     Brown,
//     Red,
// }
// impl Color {
//     fn print(&self) {
//         match self {
//             Color::Brown => println!("brown"),
//             Color::Red => println!("Red"),
//         }
//     }
// }
// #[derive(Debug)]
// struct Dimension {
//     width: f64,
//     height: f64,
//     depth: f64,
// }
// impl Dimension {
//     fn print(&self) {
//         println!("width {:?}", self.width);
//         println!("height {:?}", self.height);
//         println!("depth {:?}", self.depth);
//     }
// }
// #[derive(Debug)]
// struct ShippingBox {
//     color: Color,
//     weight: f64,
//     dimension: Dimension,
// }
// impl ShippingBox {
//     fn new(weight: f64, color: Color, dimension: Dimension) -> Self {
//         Self {
//             weight,
//             color,
//             dimension,
//         }
//     }
//     fn print(&self) {
//         self.color.print();
//         self.dimension.print();
//         println!("weight {:?}", self.weight);
//     }
// }

// fn main() {
//     let small_dimension = Dimension {
//         width: 1.0,
//         height: 2.0,
//         depth: 5.0,
//     };
//     let small_box = ShippingBox::new(5.0, Color::Brown, small_dimension);
//     small_box.print();
// }

// use thiserror::Error;
// #[derive(Debug, Error)]
// enum ProgramError {}
// #[derive(Debug, Error)]
// enum MathError {
//     #[error("divide by zero")]
//     DivideByZero,
// }
// #[derive(Debug, Error)]
// enum MenuError {
//     #[error("menu not found")]
//     Notfound,
// }
// fn pick_menu(choice: &str) -> Result<i32, MenuError> {
//     match choice {
//         "1" => Ok(1),
//         "2" => Ok(2),
//         "3" => Ok(3),
//         _ => Err(MenuError::Notfound),
//     }
// }
// fn divide(a: i32, b: i32) -> Result<i32, MathError> {
//     if b != 0 {
//         Ok(a / b)
//     } else {
//         Err(MathError::DivideByZero)
//     }
// }
// fn run(step: i32) -> Result<(), ProgramError> {
//     if step == 1 {
//         pick_menu(("4"));
//     } else if step == 2 {
//         divide(1, 0);
//     }
//     Ok(())
// }

// fn main() {
//     println!("{:?}", run(1));
//     println!("{:?}", run(2)); 
// }
