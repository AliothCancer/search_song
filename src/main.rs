use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use std::{env, fs};

fn generate_file_name_for_query(song_title: &str) -> String {
    let file_name = song_title.replace(" ", "_").to_string();
    file_name
}
fn main() {
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
        .args(["-o", "/home/giulio/Musica/dowloaded/%(title)s.%(ext)s"])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn youtube-dl");
    
    let mut yt_dlp_output = String::new();
    let mut song_name = String::new();

    while let Some(mut child) = yt_dlp_process.stdout.take() {
        if let Ok(_len) = child.read_to_string(&mut yt_dlp_output){
            let mut intr_line = yt_dlp_output.lines()
            .find(|x| x.contains("/home/giulio/Musica/dowloaded/"))
            .unwrap_or("")
            .split("/home/giulio/Musica/dowloaded/")
            .last()
            .unwrap_or("")
            //.to_string()
            .split(".")
           
            //.filter(|x|x.len()>2)
            //.take(1)
            .map(str::to_string)
            .collect::<Vec<String>>();
            
            intr_line.iter_mut()
            .for_each(|x|{
                *x = match x.as_str() {
                    "webm" => "opus".to_string(),
                    _ => x.to_string()
                }
            });
            
            
            //println!("{}", &yt_dlp_output[..len])
            println!("line: {:?}", &intr_line);
            song_name = intr_line.join(".")
        };
    }

    yt_dlp_process.wait().expect("Waiting for completion of yt-dlp execution");
    
    //song_name += ".m4a";
    println!("song_name: {}",&song_name);
    //dbg!(youtube_dl_process);
    println!("Succesfully downloaded to:\n{}", "/home/giulio/Musica/dowloaded/".to_string()+&song_name);

    state = ProgramState::Downloaded;

    //youtube_dl_process.kill().expect("killing youtube-dl");

    // REPRODUCE SONG

    //let file_name = generate_file_name_for_query(&search_query);
    //println!("file_name: {}",file_name);

    let file_name = Path::new("/home/giulio/Musica/dowloaded/")
        .join(song_name);

    let arg_file_path = file_name.to_str().unwrap();

    println!("{}",&arg_file_path);
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

            print!("\n\ninviare 'q' per uscire >>> ");
            stdout().flush().expect("flushing");

            let mut cmd = String::new();
            stdin().read_line(&mut cmd).unwrap();
            if cmd.as_str().trim() == "q" {
                mpv_process
                    .kill()
                    .expect("error trying to kill mpv process");
                state = ProgramState::Stop;
                break;
            }
        }

        std::thread::sleep(Duration::from_millis(100))
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
