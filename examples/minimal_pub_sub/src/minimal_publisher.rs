use anyhow::{Error, Result};
use rand::Rng;
use std::env;
use std::{borrow::Cow, time::Duration, time::Instant};
use uhlc::system_time_clock;

use rosidl_runtime_rs::Sequence;

fn main() -> Result<(), Error> {
    let context = rclrs::Context::new(env::args())?;

    let node = rclrs::create_node(&context, "minimal_publisher")?;

    let publisher = node.create_publisher::<std_msgs::msg::rmw::ByteMultiArray>(
        "topic",
        rclrs::QOS_PROFILE_DEFAULT,
    )?;

    let mut publish_count: u32 = 1;
    let sizes = [
        8,
        64,
        512,
        2048,
        4096,
        4 * 4096,
        10 * 4096,
        100 * 4096,
        1000 * 4096,
        10000 * 4096,
        8,
    ];

    // test latency first
    for size in sizes {
        for _ in 0..100 {
            let mut message = std_msgs::msg::rmw::ByteMultiArray::default();
            message.data = rand::thread_rng()
                .sample_iter(rand::distributions::Standard)
                .take(size)
                .collect();
            let t_send = system_time_clock().as_u64();
            let t_send = bytemuck::bytes_of(&t_send);
            let time_copying = Instant::now();
            let mut slice = message.data.as_mut_slice();
            let mut beginning_slice = slice.get_mut(0..8).unwrap();
            beginning_slice.copy_from_slice(t_send);
            // println!("time: copying: {:#?}", time_copying.elapsed());
            //   println!("Publishing: {}", message.data);
            publisher.publish(&message)?;
            // println!("time: publishing2 {:#?}", time_copying.elapsed());
            publish_count += 1;
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    while context.ok() {}
    Ok(())
}
