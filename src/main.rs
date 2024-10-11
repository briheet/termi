use std::os::fd::RawFd;

fn spawn_pty_with_shell(_default_shell: String) -> RawFd {
    unimplemented!()
}

fn read_from_fd(_fd: RawFd) -> Option<Vec<u8>> {
    unimplemented!()
}

fn main() {
    let default_shell =
        std::env::var("SHELL").expect("could not find the default shell form $SHELL");

    // println!("{}", default_shell);
    let stdout_pty = spawn_pty_with_shell(default_shell);

    let mut read_buffer = vec![];

    loop {
        match read_from_fd(stdout_pty) {
            Some(mut read_bytes) => {
                read_buffer.append(&mut read_bytes);
            }
            None => {
                println!("{:?}", String::from_utf8(read_buffer).unwrap());
                std::process::exit(0)
            }
        }
    }
}
