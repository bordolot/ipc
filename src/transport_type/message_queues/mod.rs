pub struct MsgQAgent {
    queues: Vec<Queue>,
}

impl MsgQAgent {
    pub fn new_r() -> Self {
        MsgQAgent { queues: Vec::new() }
    }

    pub fn open(&mut self, name: String) -> std::io::Result<()> {
        if name.bytes().any(|b| b == b'/') {
            return Err(std::io::Error::other("using / is invalid"));
        }
        let mut result: Vec<u8> = Vec::with_capacity(1 + name.len());
        result.push(b'/');
        result.extend_from_slice(name.as_bytes());
        let sliced = result.into_boxed_slice();

        // let oflag = match mode {
        //     Mode::Read => libc::O_RDONLY,
        //     Mode::Write => libc::O_WRONLY,
        //     Mode::Both => libc::O_RDWR,
        // };
        // let a = libc::O_CLOEXEC;
        // let b = libc::O_CREAT;
        // let c = libc::O_EXCL;
        // let d = libc::O_NONBLOCK;
        // let new_name = std::ffi::CString::new("/my_queue")?;
        // let oflag = libc::O_RDWR | libc::O_CLOEXEC;
        // let mqd = unsafe { libc::mq_open(new_name.as_ptr(), oflag) };
        // let mqd = unsafe { libc::mq_open(name.as_ptr(), oflag) };

        // Recommended: always start with '/'

        // let name = std::ffi::CString::new("/qq")?; // or "/1" if you really want that
        let oflag: i32 = libc::O_RDWR | libc::O_CLOEXEC | libc::O_CREAT;
        let user_m = libc::S_IRUSR | libc::S_IWUSR;
        let group_m = libc::S_IRGRP | libc::S_IWGRP;
        let others_m = libc::S_IROTH | libc::S_IWOTH;
        let mode = user_m | group_m | others_m;
        // let mut attr: libc::mq_attr = unsafe { std::mem::zeroed() };
        // attr.mq_maxmsg = 32;
        // attr.mq_msgsize = 4096;
        let mqd: libc::mqd_t = unsafe {
            libc::mq_open(
                sliced.as_ptr() as *const libc::c_char,
                oflag,
                mode,
                // attr,
                std::ptr::null::<libc::mq_attr>(),
            )
        };
        println!("mqd: {:?}, mode: {:?}", mqd, mode);
        if mqd == -1 {
            let err = std::io::Error::last_os_error();
            eprintln!(
                "mq_open failed: {} (errno = {})",
                err,
                err.raw_os_error().unwrap_or(0)
            );
        } else {
            println!("Message queue created/opened successfully! mqd = {}", mqd);
        }

        Ok(())
    }
}

pub struct Queue {
    pub name: std::ffi::CString,
    pub mqd: i32,
}
