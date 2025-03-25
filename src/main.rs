use clap::{Parser, Subcommand};
use commands::*;
use mpvipc::Mpv;
use std::thread::sleep;
use std::{
    process::{Command, Stdio},
    time::Duration,
};
mod commands;
mod helper;

const MPVR_SOCKET: &str = "/tmp/mpvr-socket";

#[derive(Parser)]
#[command(arg_required_else_help(true))]
/// An mpv wrapper written in Rust to play music in a single mpv instance.
struct Cli {
    files: Vec<String>,
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long)]
    ordered: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Insert files after the currently played one.
    Queue {
        files: Vec<String>,
        #[arg(short, long)]
        play: bool,
    },
    /// Add files at the end of the current playlist.
    Append { files: Vec<String> },
    /// Pause the player.
    Pause,
    /// Play the next track in the playlist.
    Next,
    /// Play the previous track in the playlist.
    Prev,
    /// Set or get the volume of the player. Range: [0-100]
    Volume {
        #[command(subcommand)]
        command: VolumeCommands,
    },
}

#[derive(Subcommand)]
enum VolumeCommands {
    Set { volume: f64 },
    Increase { volume: f64 },
    Decrease { volume: f64 },
    Get,
}

fn start_mpv() -> std::process::Child {
    Command::new("sh")
        .arg("-c")
        .arg(
            format!(
                "mpv --profile=mpvr --idle --input-ipc-server={} & disown",
                MPVR_SOCKET
            )
            .as_str(),
        )
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .expect("Failed to create mpv child process")
}

fn main() {
    let cli = Cli::parse();
    if let Err(_e) = Mpv::connect(MPVR_SOCKET) {
        start_mpv().wait().unwrap();
        sleep(Duration::from_millis(200)); // mpv takes some time to start and open the socket.
                                           // without the delay, mpvr will fail to connect.
    }
    let mpv = Mpv::connect(MPVR_SOCKET).expect("Failed to connect to socket");
    match &cli.command {
        Some(command) => match command {
            Commands::Queue { files, play } => {
                mpv.set_property("shuffle", !&cli.ordered).unwrap();
                playlist::queue(&mpv, files, play);
            }
            Commands::Append { files } => {
                mpv.set_property("shuffle", !&cli.ordered).unwrap();
                playlist::append(&mpv, files);
            }
            Commands::Pause => player::pause(&mpv),
            Commands::Next => player::next(&mpv),
            Commands::Prev => player::prev(&mpv),
            Commands::Volume { command } => match command {
                VolumeCommands::Set { volume } => player::set_volume(&mpv, *volume),
                VolumeCommands::Increase { volume } => player::increase_volume(&mpv, *volume),
                VolumeCommands::Decrease { volume } => player::decrease_volume(&mpv, *volume),
                VolumeCommands::Get => player::get_volume(&mpv),
            },
        },
        None => {
            mpv.set_property("shuffle", true).unwrap();
            playlist::queue(&mpv, &cli.files, &true);
        }
    }
}
