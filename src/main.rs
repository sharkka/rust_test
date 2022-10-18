/**
 * brief  : root
 *          RUST
 * date   : 2022-10-09 14:52:03
 * author : ANYZ
 */
mod common;
mod test;
use std::env;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("test rust");
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    crate::test::testbox::test_all();
    //crate::test::testbox::test_console().await?;
    Ok(())
}
