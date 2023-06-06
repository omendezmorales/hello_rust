// see complete tutorial at https://dev.to/jareds/how-to-get-started-with-rust-for-risc-v-linux-2fop

use ockam::*;

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    println!("Hello, RISC-V!");
    ctx.stop().await
}