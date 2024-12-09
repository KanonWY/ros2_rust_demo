use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;
use r2r::jszr_robots_dog_msg::msg::Monitor;
use r2r::{self, QosProfile};

mod jszr_info;
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let ctx = r2r::Context::create()?;
//     let mut node = r2r::Node::create(ctx, "jszr_monitor", "")?;
//     let publisher = node.create_publisher::<Monitor>("monitor", QosProfile::default())?;
//
//     let mut timer = node.create_wall_timer(std::time::Duration::from_millis(100))?;
//
//     let mut pool = LocalPool::new();
//     let spawner = pool.spawner();
//
//     spawner.spawn_local(async move {
//         loop {
//             let _elapsed = timer.tick().await.unwrap();
//             let mut msg = r2r::jszr_robots_dog_msg::msg::Monitor::default();
//             let mut process_one = r2r::jszr_robots_dog_msg::msg::ProcessInfo::default();
//
//             process_one.process_name = r2r::std_msgs::msg::String {
//                 data: std::format!("{}", "/slam"),
//             };
//             process_one.cpu_usage = 10.2;
//             process_one.memory_usage = 233.0;
//             msg.process_infos.push(process_one);
//
//             let mut process_two = r2r::jszr_robots_dog_msg::msg::ProcessInfo::default();
//             process_two.process_name = r2r::std_msgs::msg::String {
//                 data: std::format!("{}", "/nav2"),
//             };
//             process_two.cpu_usage = 10.2;
//             process_two.memory_usage = 233.0;
//             msg.process_infos.push(process_two);
//
//             println!("publish it");
//             publisher.publish(&msg).unwrap();
//         }
//     })?;
//
//     loop {
//         node.spin_once(std::time::Duration::from_millis(100));
//         pool.run_until_stalled();
//     }
// }

fn main() {
    let all_processes = procfs::process::all_processes()
        .expect("Cant read /proc")
        .filter_map(|p| match p {
            Ok(p) => Some(p),
            Err(e) => match e {
                procfs::ProcError::NotFound(_) => None,
                procfs::ProcError::Io(e, path) => None,
                x => None,
            },
        })
        .collect();
}
