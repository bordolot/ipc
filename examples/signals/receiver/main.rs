use ipc::transport_type::signals;

fn main() -> std::io::Result<()> {
    let recev = signals::ReceiverAgent::new();
    recev.create_pid_src()?;
    println!("waiting for msg...");
    recev.wait_for_msg();
    Ok(())
}
