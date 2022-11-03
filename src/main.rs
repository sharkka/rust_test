/**
 * brief  : root
 *          RUST
 * date   : 2022-10-09 14:52:03
 * author : ANYZ
 */

mod newland;
use std::env;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("test rust");
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    //crate::test::testbox::test_all();
    //crate::test::testbox::test_console().await?;

    newland::ohio::file_write();
    //newland::ohio::wserver();
    newland::rstd::newland_all();

    Ok(())
}
