use ipc::transport_type::signals;
fn main() -> std::io::Result<()> {
    let sender = signals::SenderAgent::new()?;
    let sig = 24;
    let _ = sender.send_msg(sig);
    Ok(())
}
