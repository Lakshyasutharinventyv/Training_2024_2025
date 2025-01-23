use std::time::Duration;
use tokio::time::sleep;
use rand::{thread_rng, Rng};


pub async fn level4(){
    println!("Loading... level 4");
    sleep(Duration::from_millis(3000)).await;
    println!("\nIf tile you stepped on is marked with even number then you are safe other wise you will die!");
    sleep(Duration::from_millis(3000)).await;
    let n: i8 = thread_rng().gen_range(-128..127);
    let status = match find_mines(n).await{
        Ok(r)=>r,
        Err(e)=>e
    };
    println!("{status}");
   
    super::_5_level_five::level5().await;
}

async fn find_mines(n:i8) -> Result<String, String>{
    // if n < 0{
    //     panic!("Invalid tile. Negative Number found.")
    // }
    if n % 2 == 0{
        sleep(Duration::from_millis(3000)).await;
        Ok(String::from(format!("You are safe. You stepped on {n}")))
    }
    else {
        sleep(Duration::from_millis(3000)).await;
       Err(String::from(format!("You Died mine found. You stepped on {n}")))
    }
}