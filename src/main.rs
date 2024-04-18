use std::process::ExitCode;

use clap::Parser;
use framebuffer::Framebuffer;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// Optional path to framebuffer (/dev/fb*)
    #[arg()]
    fb: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let fb_path = args.fb.unwrap_or("/dev/fb0".to_string());

    let fb = match Framebuffer::new(&fb_path) {
        Ok(fb) => fb,
        Err(e) => {
            eprintln!("Failed to open framebuffer {}: {}", &fb_path, e.details);
            return ExitCode::FAILURE;
        }
    };

    println!("\nInfo for {fb_path}\n");
    println!("{:#?}", fb.fix_screen_info);
    println!("{:#?}", fb.var_screen_info);

    ExitCode::SUCCESS
}
