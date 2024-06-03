use crate::rhm::Rhm;
mod rhm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut rhm = Rhm::new().await?;
    println!("{}", rhm.insert("name", "Kamil").await?);
    println!("{}", rhm.insert("name", "Gimba").await?);
    println!("{}", rhm.insert("tree", "Pine").await?);
    println!("{}", rhm.insert("plant", "Grass").await?);
    println!("{}", rhm.get("name"));
    println!("{}", rhm.get("another_example"));
    Ok(())
}
