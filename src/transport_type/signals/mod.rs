use std::{
    fs::{File, OpenOptions},
    io,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

const PID_SRC_FILE: &str = "ipc_sigrecid";

pub enum Level {
    Child,
    Parent,
}

pub enum Mode {
    Forked(Level),
    PidFile,
}

pub struct ReceiverAgent {}

impl ReceiverAgent {
    pub fn new() -> Self {
        ReceiverAgent {}
    }
    pub fn create_pid_src(&self) -> io::Result<()> {
        writeln!(BufWriter::new(get_file()?), "{}", std::process::id())?;
        Ok(())
    }

    pub fn wait_for_msg(&self) {
        let msg = unsafe { libc::pause() };
        println!("msg: {msg}")
    }
}

pub struct SenderAgent {
    pub rec_pid: i32,
}
impl SenderAgent {
    pub fn new() -> io::Result<Self> {
        Ok(SenderAgent {
            rec_pid: get_pid_from_src()?,
        })
    }
    pub fn send_msg(&self, sig: i32) -> Result<(), std::io::Error> {
        let res = unsafe { libc::kill(self.rec_pid, sig) };
        println!("send with res: {res}");
        Ok(())
    }
}

fn get_file() -> io::Result<File> {
    let temp_file_path = PathBuf::from(format!(
        "{}/{}",
        std::env::temp_dir().display(),
        PID_SRC_FILE
    ));
    Ok(OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .append(false)
        .open(temp_file_path)?)
}

fn get_pid_from_src() -> io::Result<i32> {
    let mut buf = String::new();
    let mut reader = BufReader::new(get_file()?);
    reader.read_line(&mut buf)?;
    let id: i32 = match buf.trim().parse() {
        Ok(id) => id,
        Err(er) => panic!("{er}"),
    };
    Ok(id)
}
