//!
//! # Imu(BMI088) Task
//!

use crate::system::*;
use ahrs::{Ahrs, Mahony};
use libm::{atan2, sqrt};
use nalgebra::{UnitQuaternion, Vector3};
use utils::StaticCell;

mod typedef;

use typedef::BMI088;

#[unsafe(link_section = ".axisram.imu")]
static BUFFER: StaticCell<[u8; 16]> = StaticCell::new();

#[embassy_executor::task]
pub async fn task(p: ImuSrc) -> ! {
    let buffer = BUFFER.init([0; _]);
    let mut imu = BMI088::new(p, buffer);

    while imu.init().await == false {
        defmt::warn!("BMI088 Init Failed, Retrying...");
    }

    let mut acc_last = imu.read_acc().await;

    let quat: _ = UnitQuaternion::from_euler_angles(
        atan2(acc_last.1, acc_last.2),
        atan2(
            -acc_last.0,
            sqrt(acc_last.1 * acc_last.1 + acc_last.2 * acc_last.2),
        ),
        0.,
    );

    // TODO: `0.001s` is determined by Gyro New Data Interrupt ODR(1000Hz)
    let mut ahrs = Mahony::new_with_quat(0.001, 3.5, 0., quat);

    loop {
        imu.wait_new_data().await;

        // TODO: impl `Constant Temperature Control`: 45°C
        let temp = imu.read_temp().await;
        defmt::trace!("BMI088 Temp Data Ready: {:?}", temp);

        let imu: _ = get_imu_data(&mut imu, &mut acc_last).await;
        let (gyro, acc) = imu;

        // Update AHRS
        let Ok(_) = ahrs.update_imu(&gyro, &acc) else {
            defmt::warn!("BMI088 AHRS Update Error!!!");
            continue;
        };

        // // Output Euler Angles
        // let (x, y, z) = sta.euler_angles();
        // let a = (x.to_degrees(), y.to_degrees(), z.to_degrees());
        // defmt::trace!("Roll={}, Pitch={}, Yaw={}", a.0, a.1, a.2);
    }
}

async fn get_imu_data(
    imu: &mut BMI088<'_>,
    acc_last: &mut (f64, f64, f64),
) -> (Vector3<f64>, Vector3<f64>) {
    // Read Gyro Data, And Unit Transform
    let (x, y, z) = imu.read_gyro().await;
    let gyro = (x.to_radians(), y.to_radians(), z.to_radians());

    // Read Acc Data, And Unit Transform
    let (x, y, z) = imu.read_acc().await;
    let norm = sqrt(x * x + y * y + z * z);
    let acc = if norm > 1e-3 {
        (x / norm, y / norm, z / norm)
    } else {
        *acc_last
    };

    *acc_last = acc;

    (
        // Return Data: Gyro, Acc
        Vector3::new(gyro.0, gyro.1, gyro.2),
        Vector3::new(acc.0, acc.1, acc.2),
    )
}
