// mod main1;
// use main1::*;
// mod main2;
// use main2::*;
/*A factory is giving away tshirts to their user and if the user have a prefrence and
if that tshirt color is available then they give that or else the most stocked
tshirt */

#[derive(Debug, PartialEq, Clone, Copy)]
enum TShirtColor {
    Red,
    Blue,
}
#[derive(Debug)]
struct Inventory {
    tshirts: Vec<TShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_prefrences: Option<TShirtColor>) -> Option<TShirtColor> {
        let (num_red,num_blue) = self.stock();
        if user_prefrences == Some(TShirtColor::Blue) || user_prefrences == Some(TShirtColor::Red){
            if num_red == 0 && user_prefrences == Some(TShirtColor::Blue) && num_blue > 0{
                println!("{:?} is out of stock!",TShirtColor::Blue);
                Some(TShirtColor::Blue)
            }
            else if num_blue == 0 && user_prefrences == Some(TShirtColor::Red) && num_red > 0 {
                println!("{:?} is out of stock!",TShirtColor::Red);
                Some(TShirtColor::Red)
            }else if num_red > 0 && num_blue > 0{
                Some(user_prefrences.unwrap_or_else(|| self.most_stocked(num_red,num_blue)))
            }else {
                println!("No tshirt available right now");
                return None
            }
        }else{
            Some(user_prefrences.unwrap_or_else(|| self.most_stocked(num_red,num_blue)))
        }
    }

    fn most_stocked(&self,num_red:i32,num_blue:i32) -> TShirtColor {
        
        if num_red > num_blue {
            TShirtColor::Red
        } else {
            TShirtColor::Blue
        }
    }

    fn stock(&self)->(i32,i32){
        let mut num_red = 0;
        let mut num_blue = 0;
        for t_shirt in &self.tshirts {
            match t_shirt {
                TShirtColor::Red => num_red += 1,
                TShirtColor::Blue => num_blue += 1,
            };
        }
        (num_red,num_blue)
    }
}

fn main() {
        let store=Inventory{
            tshirts:vec![TShirtColor::Blue,TShirtColor::Blue]
        };
        let user1_prefrence=Some(TShirtColor::Red);
        let result1=store.giveaway(user1_prefrence);
        println!("the user had preference {:?} and got {:?}",user1_prefrence,result1);
    //     let user2_prefrence=None;
    //    let result2= store.giveaway(user2_prefrence);
    //    println!("the user had preference {:?} and got {:#?}",user2_prefrence,result2);

    // main1::main_1();

    // fn add_one_v1(x: u32) -> u32 {
    //     x + 1
    // }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| x + 1;
    // let add_one_v4 = |x| x + 1;

    // let answer = add_one_v2(add_one_v3(add_one_v4(add_one_v1(0))));
    // println!("{}", answer);
}