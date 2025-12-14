use crate::system::*;

#[embassy_executor::task]
pub async fn main() {
    let mut t = utils::init_ticker!(1);

    SysMode::Normal.set();

    loop {
        t.next().await
    }
}
