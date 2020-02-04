#[macro_use]
extern crate clap;

extern crate ftp;
extern crate printer;

// TODO: create ftpparse library from http://cr.yp.to/ftpparse/ftpparse.c

use printer::add_one;

use ftp::FtpStream;
use std::io::Cursor;
use std::str;

use ftp_cli::{ConnectionConfig, FtpConnection};

use ansi_term::Colour::{Blue, Yellow};

fn main() {
    // setup arguments
    let matches = clap_app!(myapp =>
        (version: "0.0.1")
        (author: "Benjamin Bours <boursbenjamin@gmail.com>")
        (about: "CLI FTP app to ease your life")
        (@arg HOST: +required "Sets the host URL")
        (@arg USER: +required "User")
        (@arg PASSWORD: +required "Password")
    )
    .get_matches();

    let connection_config = ConnectionConfig::new(
        matches.value_of("HOST").unwrap(),
        matches.value_of("USER").unwrap(),
        matches.value_of("PASSWORD").unwrap(),
    );

    let mut ftp_connection = FtpConnection::new(connection_config);

    // println!("{:?} {:?} {:?}", host, user, password);
    // FtpConnection::

    // Create a connection to an FTP server and authenticate to it.
    // let mut ftp_stream = FtpStream::connect(host).unwrap();
    // let _ = ftp_stream.login(user, password).unwrap();

    println!("It seems your are connected");
    // // Get the current directory that the client will be reading from and writing to.
    // println!("Current directory: {}", ftp_stream.pwd().unwrap());
    // // Change into a new directory, relative to the one we are currently in.
    // // let _ = ftp_stream.cwd("test_data").unwrap();

    // Retrieve (GET) a file from the FTP server in the current working directory.
    // let remote_file = ftp_connection.stream.simple_retr("hello_world").unwrap();
    // println!("Read file with contents\n{}\n", str::from_utf8(&remote_file.into_inner()).unwrap());

    let list = ftp_connection.stream.list(None);
    println!("list: {:?}", list.unwrap());
    // // // Store (PUT) a file from the client to the current working directory of the server.
    // // let mut reader = Cursor::new("Hello from the Rust \"ftp\" crate!".as_bytes());
    // // let _ = ftp_stream.put("greeting.txt", &mut reader);
    // // println!("Successfully wrote greeting.txt");

    // // Terminate the connection to the server.
    // // let _ = ftp_stream.quit();

    // // println!(
    // //     "Demonstrating {} and {}!",
    // //     Blue.bold().paint("blue bold"),
    // //     Yellow.underline().paint("yellow underline")
    // // );

    // // println!("Yellow on blue: {}", Yellow.on(Blue).paint("wow!"));
}
