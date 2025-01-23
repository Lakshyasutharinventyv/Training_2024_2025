use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
pub async fn level1() {
    println!("Loading ...");
    sleep(Duration::from_millis(3000)).await;

    println!("Game Started.");
    let cur_no_of_trees = 40;
    let pollution = cur_no_of_trees > 20;
    let name_of_tree = "Magical Tree";
    sleep(Duration::from_millis(5000)).await;

    println!(
        "There is {name_of_tree} which shrinks or increases height by 5 meter in 1 sec and it will disappear or reaches its max height of 50 depending on numbers of trees left in forest."
    );

    let mut tree_height = 30;
    sleep(Duration::from_millis(3000)).await;

    println!("Height of {name_of_tree} is {}", tree_height);
    sleep(Duration::from_millis(3000)).await;

    println!("{} spans {} sq. mt area", name_of_tree, area_of_tree_shadow(&mut tree_height));
    sleep(Duration::from_millis(3000)).await;

    match pollution {
        true => {
            println!("Oh no pollution is less so {name_of_tree} height is shrinking down!");
            shrink(&mut tree_height);
        }
        false => {
            println!(
                "Oh pollution is increasing so in order to reduce pollution {name_of_tree}'s height is increasing!"
            );
            increase(&mut tree_height);
        }
    }
    sleep(Duration::from_millis(1000)).await;

    println!("New Height of {name_of_tree} is {} m", tree_height);

    super::_2_level_two::level2().await;
}

fn shrink(height: &mut i32) {
    *height -= 5;
}
fn increase(height: &mut i32) {
    *height += 5;
}
fn area_of_tree_shadow(height: &mut i32) -> f32 {
    (*height as f32) * 111.11
}
