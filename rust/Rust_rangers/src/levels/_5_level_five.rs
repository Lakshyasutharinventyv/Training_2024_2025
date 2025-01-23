use std::time::Duration;
use tokio::time::sleep;
use rand::{thread_rng, Rng};

#[derive(Debug)]
enum Crops{
    Wheat,
    Corn,
    Rice,
}

struct Field{
    area:f32,
    no_of_workers:u16,
    crop_type:Crops
}
impl Field {
    fn plant_crop()->Crops{
        let n: u8 = thread_rng().gen_range(0..3);
        match n {
            0 => Crops::Wheat,
            1 => Crops::Corn,
            _ => Crops::Rice
        }
    }
}

pub async fn level5(){
    println!("Loading... level 5");
    sleep(Duration::from_millis(3000)).await;

    println!("You are farmer!");
    sleep(Duration::from_millis(3000)).await;

    let field = Field {
        area: 433.123,
        no_of_workers: 5,
        crop_type:Field::plant_crop()
    };

    sleep(Duration::from_millis(3000)).await;
    println!(
        "You have a Field with following Details:\nArea: {} acres \nWorkers: {}\nCrop Type: {:?}",
        field.area, field.no_of_workers, field.crop_type
    );
    super::_6_level_six::level6().await;
} 