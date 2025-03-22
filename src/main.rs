use std::ffi::OsStr;
use clap::Parser;
use log::info;
use rustfs::RustFS;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    mountpoint: String,

    #[arg(short, long)]
    debug: bool,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let fs = RustFS::new();

    let mut options = vec![
        OsStr::new("-o"),
        OsStr::new("fsname=rustfs"),
    ];

    if args.debug {
         options.push(OsStr::new("-d"));
    }

    info!("Mounting RustFS at {}", args.mountpoint);
    fuse::mount(fs, &args.mountpoint, &options)
         .expect("Failed to mount file system");
}
