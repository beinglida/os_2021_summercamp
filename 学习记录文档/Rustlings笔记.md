# 1.variables

1. exercises/variables/variables1.rs

   S: x前加let即可解决错误；

   在variables1.rs中删除‘I AM NOT DONE`这行注释之后，进如下一题；
   
2. exercises/variables/variables2.rs

   S: 给x指定类型并且赋初始值，也可以让x为mut

   

3. exercises/variables/variables3.rs

   S: 让x为mutable ，在如该语句let mut x = 初始值；

   

4. exercises/variables/variables4.rs

   S: x未赋初始值，x后面赋值即可；



5. exercises/variables/variables5.rs

   S: 重新声明一个变量，使其为整数类型即可进行加减；

   

6. exercises/variables/variables6.rs

   S: const后面必须说明参数类型，如const NUMBER:i32 = 3;



# 2.functions

1. exercises/functions/functions1.rs!

   S: 函数外增加一个函数，以供main()函数调用；

2. S: 函数传入参数要指定类型，如下；

   ```
   fn call_me(num:i32) {
   
   ​    for i in 0..num {
   
   ​        println!("Ring! Call number {}", i + 1);
   
   ​    }
   
   }
   ```

3. 函数中要传入参数即可；

4. 函数要指定返回的值的类型，如下：

   ```
   fn sale_price(price: i32) -> i32{//在此处添加返回值的类型；
   
   ​    if is_even(price) {
   
   ​        price - 10
   
   ​    } else {
   
   ​        price - 3
   
   ​    }
   
   }
   ```

   

5. 函数中的返回语句不需要加分号，把函数体中的返回语句分号去掉即可；

# 3.if

1. 函数中加入判断语句即可解决；

   ```
   pub fn bigger(a: i32, b: i32) -> i32 {
   
       if(a > b) {
   
           a
   
       }
   
       else {
   
           b
   
       }
   ```

2. 函数中加入字符串判断语句即可；



# 4.move_semantics

1. 让vec1编程mutable即可，代码如下；

   ```
   fn main() {
       let vec0 = Vec::new();
   
       let mut vec1 = fill_vec(vec0);
   
       println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
   
       vec1.push(88);
   
       println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
   }
   
   ```

   

2. 交换一下语句的顺序即可，问题的原因在于当vec1获得vec0的所有权之后，vec0这个名称就不存在了；

   ```
   fn main() {
       let vec0 = Vec::new();
   
       // Do not change the following line!
       println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
       
       let mut vec1 = fill_vec(vec0);
   
       vec1.push(88);
   
       println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
   }
   ```

3. 函数中传入的参数，也要使得该参数为mutable，如下：

   ```
   fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
       vec.push(22);
       vec.push(44);
       vec.push(66);
   
       vec
   }
   ```

4. 声明unmutalble向量时，要指明声明的向量类型；声明mutable向量时，可以不指明向量的类型；

   ```
   fn main() {
       let vec0:Vec<i32> = Vec::new();
   
       let mut vec1 = fill_vec();
   
       println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
   
       vec1.push(88);
   
       println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
   }
   
   // `fill_vec()` no longer takes `vec: Vec<i32>` as argument
   fn fill_vec() -> Vec<i32> {
       let mut vec = Vec::new();
   
       vec.push(22);
       vec.push(44);
       vec.push(66);
   
       vec
   ```

5. 同第2题，交换一次次序即可；

   ```
   fn main() {
       let mut x = 100;
       let y = &mut x;
       *y += 100;
       let z = &mut *y;
       *z += 1000;
       assert_eq!(x, 1200);
   }
   ```



# 6.primitive_types

1. 这种结构的判断，需要先给is_morning赋予true or false，使其成为bool类型；

```
fn main() {
    // Booleans (`bool`)

    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening = false;// Finish the rest of this line like the example! Or make it be false!
    if is_evening {
        println!("Good evening!");
    }
}
```

2. 给your_character赋予单个字符，注意is_alphabetic()、is_numeric()函数只能用于判断单个字符；

   ```
   fn main() {
       // Characters (`char`)
   
       let my_first_initial = 'C';
       if my_first_initial.is_alphabetic() {
           println!("Alphabetical!");
       } else if my_first_initial.is_numeric() {
           println!("Numerical!");
       } else {
           println!("Neither alphabetic nor numeric!");
       }
   
       let your_character = '1';// Finish this line like the example! What's your favorite character?
       // Try a letter, try a number, try a special character, try a character
       // from a different language than your own, try an emoji!
       if your_character.is_alphabetic() {
           println!("Alphabetical!");
       } else if your_character.is_numeric() {
           println!("Numerical!");
       } else {
           println!("Neither alphabetic nor numeric!");
       }
   }
   ```

3. 声明一个数组即可，注意len()函数只能用于判断数组的长度；

   ```
   fn main() {
       let a = [1, 2, 3, 4, 5];
   
       if a.len() >= 100 {
           println!("Wow, that's a big array!");
       } else {
           println!("Meh, I eat arrays like that for breakfast.");
       }
   }
   ```

4. 字符串分隔，&a[sp. .ep+1] 其中s为起始位置startposition，ep为终止位置endposition；

   ```
   fn slice_out_of_array() {
       let a = [1, 2, 3, 4, 5];
   
       let nice_slice = &a[1..4];
   
       assert_eq!([2, 3, 4], nice_slice)
   }
   ```

5. 元组tuple的复制，(value1, value2, ……) = name_of_tuple；

   ```
   fn main() {
       let cat = ("Furry McFurson", 3.5);
       let (name, age) = cat;
   
       println!("{} is {} years old.", name, age);
   }
   ```

6. 元组的访问方式区别于数组，name_of_tuple.index_number;

   ```
   fn indexing_tuple() {
       let numbers = (1, 2, 3);
       // Replace below ??? with the tuple indexing syntax.
       let second = numbers.1;
   
       assert_eq!(2, second,
           "This is not the 2nd number in the tuple!")
   }
   ```



# 7.structs

1. struct具体用法；

   ```
   struct ColorClassicStruct {
       // TODO: Something goes here
       name: String,
       hex: String
   }
   
   struct ColorTupleStruct(String, String);
   
   #[derive(Debug)]
   struct UnitStruct;
   
   #[cfg(test)]
   mod tests {
       use super::*;
   
       #[test]
       fn classic_c_structs() {
           // TODO: Instantiate a classic c struct!
           // let green =
           let green = ColorClassicStruct {
               name:String::from("green"),
               hex: String::from("#00FF00")
           };
   
           assert_eq!(green.name, "green");
           assert_eq!(green.hex, "#00FF00");
       }
   
       #[test]
       fn tuple_structs() {
           // TODO: Instantiate a tuple struct!
           // let green =
           let green = ColorTupleStruct(String::from("green"), String::from("#00FF00"));
   
           assert_eq!(green.0, "green");
           assert_eq!(green.1, "#00FF00");
       }
   
       #[test]
       fn unit_structs() {
           // TODO: Instantiate a unit struct!
           // let unit_struct =
           let unit_struct = UnitStruct;
           let message = format!("{:?}s are fun!", unit_struct);
   
           assert_eq!(message, "UnitStructs are fun!");
       }
   }
   ```

2. struct具体用法，如特定struct类型变量的声明、；

   如果想要新建一个结构体的实例，其中大部分属性需要被设置成与现存的一个结构体属性一样，仅需更改其中的一两个字段的值，可以使用结构体更新语法，如：

   ```
   let site = Site {
       domain: String::from("www.runoob.com"),
       name: String::from("RUNOOB"),
       ..runoob
   };
   ```

   本题具体代码如下：

   ```
   #[derive(Debug)]
   struct Order {
       name: String,
       year: u32,
       made_by_phone: bool,
       made_by_mobile: bool,
       made_by_email: bool,
       item_number: u32,
       count: u32,
   }
   
   fn create_order_template() -> Order {
       Order {
           name: String::from("Bob"),
           year: 2019,
           made_by_phone: false,
           made_by_mobile: false,
           made_by_email: true,
           item_number: 123,
           count: 0,
       }
   }
   
   #[cfg(test)]
   mod tests {
       use super::*;
   
       #[test]
       fn your_order() {
           let order_template = create_order_template();
           // TODO: Create your own order using the update syntax and template above!
           // let your_order =
           let your_order = Order {
               name: String::from("Hacker in Rust"),
               count: 1,
               ..order_template
           };
           assert_eq!(your_order.name, "Hacker in Rust");
           assert_eq!(your_order.year, order_template.year);
           assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
           assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
           assert_eq!(your_order.made_by_email, order_template.made_by_email);
           assert_eq!(your_order.item_number, order_template.item_number);
           assert_eq!(your_order.count, 1);
       }
   }
   
   ```

   3. **impl**实现struct的另外的一些功能，有点类似于C++的派生类；

      ```
      // structs3.rs
      // Structs contain data, but can also have logic. In this exercise we have
      // defined the Package struct and we want to test some logic attached to it.
      // Make the code compile and the tests pass!
      // If you have issues execute `rustlings hint structs3`
      
      
      #[derive(Debug)]
      struct Package {
          sender_country: String,
          recipient_country: String,
          weight_in_grams: i32,
      }
      
      impl Package {
          fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
              if weight_in_grams <= 0 {
                  // Something goes here...
                  assert!(false);
                  return Package {
                      sender_country,
                      recipient_country,
                      weight_in_grams,
                  };
              } else {
                  return Package {
                      sender_country,
                      recipient_country,
                      weight_in_grams,
                  };
              }
          }
      
          fn is_international(&self) -> bool {
              // Something goes here...
              if self.recipient_country != self.sender_country {
                  true
              } else {
                  false
              }
      
          }
      
          fn get_fees(&self, cents_per_gram: i32) -> i32 {
              // Something goes here...
              self.weight_in_grams * cents_per_gram
          }
      }
      
      #[cfg(test)]
      mod tests {
          use super::*;
      
          #[test]
          #[should_panic]
          fn fail_creating_weightless_package() {
              let sender_country = String::from("Spain");
              let recipient_country = String::from("Austria");
      
              Package::new(sender_country, recipient_country, -2210);
          }
      
          #[test]
          fn create_international_package() {
              let sender_country = String::from("Spain");
              let recipient_country = String::from("Russia");
      
              let package = Package::new(sender_country, recipient_country, 1200);
      
              assert!(package.is_international());
          }
      
          #[test]
          fn create_local_package() {
              let sender_country = String::from("Canada");
              let recipient_country = sender_country.clone();
      
              let package = Package::new(sender_country, recipient_country, 1200);
      
              assert!(!package.is_international());
          }
      
          #[test]
          fn calculate_transport_fees() {
              let sender_country = String::from("Spain");
              let recipient_country = String::from("Spain");
      
              let cents_per_gram = 3;
      
              let package = Package::new(sender_country, recipient_country, 1500);
      
              assert_eq!(package.get_fees(cents_per_gram), 4500);
          }
      }
      ```



# 8.enums

1. 枚举类型的定义

   ```
   enum Message {
       // TODO: define a few types of messages as used below
       Quit,
       Echo,
       Move,
       ChangeColor,
   }
   ```

2. 枚举类型中，各参数的赋值括号注意匹配，如{ }、 ( )， 以及枚举也可以如struct一样，通过impl进行函数的派生；

   ```
   #[derive(Debug)]
   enum Message {
       // TODO: define the different variants used below
       Move{x:i32, y:i32},
       Echo(String),
       ChangeColor(u8, u8, u8),
       Quit,
   }
   
   impl Message {
       fn call(&self) {
           println!("{:?}", &self);
       }
   }
   
   fn main() {
       let messages = [
           Message::Move { x: 10, y: 30 },
           Message::Echo(String::from("hello world")),
           Message::ChangeColor(200, 255, 255),
           Message::Quit,
       ];
   
       for message in &messages {
           message.call();
       }
   }
   ```

3. struct和enum的互用；

```
enum Message {
    // TODO: implement the message variant types based on their usage below
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        match message {
            Message::Move(Point{x, y}) => self.move_position(Point{x: x, y: y}),
            Message::Echo(String) => self.echo(String),
            Message::ChangeColor(r, g, b) => self.change_color((r, g, b)),
            Message::Quit => self.quit()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }
}
```



# 9.modules

1. mod中的fn默认为private，如果想要调用，需要在fn前加pub关键词；

   ```
   mod sausage_factory {
       pub fn make_sausage() {
           println!("sausage!");
       }
   }
   
   fn main() {
       sausage_factory::make_sausage();
   }
   ```

   

2. mod中的use关键字，要放在public作用域下；

   ```
   mod delicious_snacks {
       pub use self::fruits::PEAR as fruit;
       pub use self::veggies::CUCUMBER as veggie;
    
       mod fruits {
           pub const PEAR: &'static str = "Pear";
           pub const APPLE: &'static str = "Apple";
       }
   
       mod veggies {
           pub const CUCUMBER: &'static str = "Cucumber";
           pub const CARROT: &'static str = "Carrot";
       }
   }
   
   fn main() {
       println!(
           "favorite snacks: {} and {}",
           delicious_snacks::fruit,
           delicious_snacks::veggie
       );
   }
   ```

3. 



剩下的懒得写了😂













# quiz

1. 写一个函数，在函数中加入判断条件即可，代码如下；

   ```
   fn calculate_apple_price(n: i32) -> i32 {
       if n <= 40 {
           n * 2
       } else {
           n
       }
   }
   ```

   