use example_messages::msg::HelloWorld as HelloMsg;

fn main() -> Result<(), rclrs::RclrsError> {
    let context = rclrs::Context::new(std::env::args())?;
    let node = rclrs::Node::new(&context, "example_rust_node")?;
    let publisher = node.create_publisher::<HelloMsg>("example_topic", rclrs::QOS_PROFILE_DEFAULT)?;

    let mut cnt: i64 = 0;

    println!("Hola 🦝 from 🦀!");

    while context.ok() {
        let mut msg = HelloMsg::default();
        msg.num = cnt;
        msg.frac = (cnt as f64)/10.0;
        cnt += 1;

        println!("Publishing {:?}", &msg);
        publisher.publish(msg)?;
        
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    Ok(())
}
