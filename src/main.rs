#![allow(clippy::zombie_processes, unused_assignments)]

use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::env;

fn _generate_file_name_for_query(song_title: &str) -> String {
    
    song_title.replace(" ", "_").to_string()
}


fn main() {
    let music_folder = "/home/giulio/Music/";
    let mut state = ProgramState::Started;

    let search_query = env::args().skip(1).collect::<Vec<_>>().join(" ");

    // DOWNLOAD
    println!("Downloading...");
    // youtube-dl --extract-audio --audio-format mp3 -o "/path/to/your/folder/%(title)s.%(ext)s" "ytsearch1:Song Name"

    let query = format!("ytsearch1:{}", search_query);
    println!("query: {}", query);

    state = ProgramState::Downloading;
    let mut yt_dlp_process = Command::new("yt-dlp")
        .args(["-f", "bestaudio"])
        .arg("--extract-audio")
        .arg(query)
        // /home/giulio/Musica/downloaded
        .args([
            "-o",
            &(music_folder.to_string() + "dowloaded/%(title)s.%(ext)s"),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn youtube-dl");

    let mut yt_dlp_output = String::new();
    let mut song_name = String::new();

    while let Some(mut child) = yt_dlp_process.stdout.take() {
        if let Ok(_len) = child.read_to_string(&mut yt_dlp_output) {
            let mut intr_line = yt_dlp_output
                .lines()
                .find(|x| x.contains(&(music_folder.to_string() + "dowloaded/")))
                .unwrap_or("")
                .split("/home/giulio/Music/dowloaded/")
                .last()
                .unwrap_or("")
                //.to_string()
                .split(".")
                //.filter(|x|x.len()>2)
                //.take(1)
                .map(str::to_string)
                .collect::<Vec<String>>();

            intr_line.iter_mut().for_each(|x| {
                *x = match x.as_str() {
                    "webm" => "opus".to_string(),
                    _ => x.to_string(),
                }
            });

            //println!("{}", &yt_dlp_output[..len])
            println!("line: {:?}", &intr_line);
            song_name = intr_line.join(".")
        };
    }

    yt_dlp_process
        .wait()
        .expect("Waiting for completion of yt-dlp execution");

    //song_name += ".m4a";
    println!("song_name: {}", &song_name);
    //dbg!(youtube_dl_process);
    println!(
        "Succesfully downloaded to:\n{}",
        (music_folder.to_string() + "dowloaded/").to_string() + &song_name
    );

    state = ProgramState::Downloaded;

    //youtube_dl_process.kill().expect("killing youtube-dl");

    // REPRODUCE SONG

    //let file_name = generate_file_name_for_query(&search_query);
    //println!("file_name: {}",file_name);

    let file_name = Path::new(&(music_folder.to_string() + "dowloaded/")).join(song_name);

    let arg_file_path = file_name.to_str().unwrap();

    println!("{}", &arg_file_path);
    let refs = file_name.as_path();
    loop {
        if refs.exists() && state == ProgramState::Downloaded {
            println!("Reproducing with mpv...");
            let mut mpv_process = Command::new("mpv")
                //.arg("--no-video")
                .arg(arg_file_path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to spawn mpv");

            state = ProgramState::Reproducing;

            print!("\n\ninviare 'q' per uscire oppure 'delete' per cancellare la canzone scaricata\n\n   >>> ");
            stdout().flush().expect("flushing");

            let mut cmd = String::new();
            stdin().read_line(&mut cmd).unwrap();
            let trimmed_cmd = cmd.trim();
            if trimmed_cmd == "q" {
                state = ProgramState::Stop;
            } else if trimmed_cmd == "delete" {
                state = ProgramState::Stop;
                let mut cmd = String::new();
                print!(
                    "File name: {arg_file_path}\n You sure you want to delete that file? (yes/no)",
                );
                stdout().flush().expect("");
                stdin().read_line(&mut cmd).unwrap();
                if cmd.trim() == "yes" {
                    Command::new("rm")
                        .arg(&file_name)
                        .spawn()
                        .expect("error removing the song")
                        .wait()
                        .expect("error waiting for exit from rm command");
                    state = ProgramState::Stop;
                }
            }
            mpv_process
                .kill()
                .expect("error trying to kill mpv process");
            
        }
        match state {
            ProgramState::Stop => std::process::exit(0),
            _ => continue,
        }

        //std::thread::sleep(Duration::from_millis(100))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ProgramState {
    Started,
    Downloading,
    Downloaded,
    Reproducing,
    Stop,
}
