use eframe::egui;
use nix::pty::forkpty;
use nix::unistd::ForkResult;
use std::{
    ffi::{CStr, CString},
    fs::File,
    io::Read,
    os::fd::OwnedFd,
};

fn main() {
    let fd = unsafe {
        let res = forkpty(None, None).unwrap();
        match res.fork_result {
            ForkResult::Parent { .. } => (),
            ForkResult::Child => {
                let shell_name_bytes = b"bash\0";
                let shell_name = CStr::from_bytes_with_nul(shell_name_bytes)
                    .expect("CStr::from_bytes_with_nul failed");

                let args: &[&[u8]] = &[b"--noprofile\0", b"--norc\0"];

                let args: Vec<&'static CStr> = args
                    .iter()
                    .map(|v| {
                        CStr::from_bytes_with_nul(v).expect("Should always have null terminator")
                    })
                    .collect::<Vec<_>>();

                nix::unistd::execvp(shell_name, &args).unwrap();
                return;
            }
        }
        res.master
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Termi",
        native_options,
        Box::new(move |cc| Ok(Box::new(TermiGui::new(cc, fd)))),
    );
}

struct TermiGui {
    buf: Vec<u8>,
    fd: File,
}

impl TermiGui {
    fn new(_cc: &eframe::CreationContext<'_>, fd: OwnedFd) -> Self {
        TermiGui {
            buf: Vec::new(),
            fd: fd.into(),
        }
    }
}

impl eframe::App for TermiGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut buf = vec![0u8; 4096];
        match self.fd.read(&mut buf) {
            Ok(read_size) => self.buf.extend_from_slice(&buf[0..read_size]),
            Err(e) => {
                println!("Failed to Read: {e}")
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            unsafe {
                // FIXME: needs text we have vec
                ui.label(std::str::from_utf8_unchecked(&self.buf));
            }
        });
    }
}
