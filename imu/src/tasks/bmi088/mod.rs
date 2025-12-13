//!
//! # Imu(BMI088) Task
//!

use crate::system::*;
use ahrs::{Ahrs, Mahony};
use nalgebra::Vector3;
use utils::StaticCell;

mod typedef;

use typedef::{BMI088, WaitInt};

#[unsafe(link_section = ".axisram.imu")]
static BUFFER: StaticCell<[u8; 16]> = StaticCell::new();

#[embassy_executor::task]
pub async fn task(p: ImuSrc) -> ! {
    let buffer = BUFFER.init([0; _]);
    let mut imu = BMI088::new(p, buffer);

    let mut ahrs = Mahony::new(0.001, 0.5, 0.);
    let mut acc = (0., 0., 0.);

    while imu.init().await == false {
        defmt::warn!("BMI088 Init Failed, Retrying...");
        utils::T::after_millis(100).await;
    }

    utils::T::after_millis(100).await;

    loop {
        match imu.wait().await {
            WaitInt::Gyro => {
                let (x, y, z) = imu.read_gyro().await;
                let gyro = (x.to_radians(), y.to_radians(), z.to_radians());

                let gyro: _ = Vector3::new(gyro.0, gyro.1, gyro.2);
                let acc: _ = Vector3::new(acc.0, acc.1, acc.2);

                let Ok(sta) = ahrs.update_imu(&gyro, &acc) else {
                    defmt::warn!("BMI088 AHRS Update Error!!!");
                    continue;
                };

                let (x, y, z) = sta.euler_angles();
                let a = (x.to_degrees(), y.to_degrees(), z.to_degrees());
                defmt::trace!("Roll={}, Pitch={}, Yaw={}", a.0, a.1, a.2);
            }

            WaitInt::Acc => {
                // acc = imu.read_acc().await;
                let (x, y, z) = imu.read_acc().await;
                let norm = libm::sqrt(x * x + y * y + z * z);
                acc = (x / norm, y / norm, z / norm);
            }
        }

        // let temp = imu.read_temp().await;
        // defmt::trace!("BMI088 Temp Data Ready: {:?}", temp);
    }
}
