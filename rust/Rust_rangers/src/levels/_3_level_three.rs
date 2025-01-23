use std::time::Duration;
use tokio::time::sleep;

pub async fn level3(){
    sleep(Duration::from_millis(3000)).await;
    println!("Loading... level 3");

    sleep(Duration::from_millis(3000)).await;
    println!("You need to climb the mountain. if mountain height is less than 1000m you can climb it or else you will not be able to climb it");
    
    let mut height_of_mountain = 1000;
    let mut your_max_capacity = 950;
    let name_of_mountain = String::from("mount Everest");
    climb_or_not(&mut height_of_mountain,&mut your_max_capacity,&name_of_mountain).await;
    // print!("{name_of_mountain}"); borrowed value moved so pass ref of string.
}

async fn climb_or_not(h:&mut i32,max_cap:&mut i32,name:&String){
    sleep(Duration::from_millis(3000)).await;
    print!("\n\nheight of {name} is {h} and your max capacity to climb is {max_cap} m so");
    if *h < *max_cap{
        sleep(Duration::from_millis(2000)).await;
        println!("\n\nYou climbed the {name}. Yaayy!");
    }else{
        sleep(Duration::from_millis(3000)).await;
        println!("\n\n{name} is too high. You cannot climb it. Train hard");
        let time_of_training = 10;
        increase_your_max_cap(max_cap, time_of_training).await;
        sleep(Duration::from_millis(3000)).await;
        println!("\n\nYou trined hard! for {time_of_training} hr so you can climb {name} because your cap:{max_cap} > height of {name}:{h}");
    }
    super::_4_level_four::level4().await;
}

async fn increase_your_max_cap(max_cap:&mut i32, time_of_training:i32){
    *max_cap += time_of_training * 10;
}
