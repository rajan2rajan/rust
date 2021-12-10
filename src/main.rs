





// use std::string;


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