use eframe::egui;
use nix::pty::forkpty;
use nix::unistd::ForkResult;
use std::{
    ffi::{CStr, CString},
    fs::File,
    io::Read,
    os::fd::{IntoRawFd, OwnedFd, RawFd},
};

fn main() {
    let fd = unsafe {
        let res = forkpty(None, None).unwrap();
        match res.fork_result {
            ForkResult::Parent { .. } => (),
            ForkResult::Child => {
                let shell_name_bytes = b"ash\0";
                let shell_name = CStr::from_bytes_with_nul(shell_name_bytes)
                    .expect("CStr::from_bytes_with_nul failed");
                nix::unistd::execvp::<CString>(shell_name, &[]).unwrap();
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
    buf: String,
    fd: File,
}

impl TermiGui {
    fn new(_cc: &eframe::CreationContext<'_>, fd: OwnedFd) -> Self {
        TermiGui {
            buf: String::new(),
            fd: fd.into(),
        }
    }
}

impl eframe::App for TermiGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut buf = vec![0u8; 4098];
        match self.fd.read(&mut buf) {}
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Termi");
        });
    }
}
