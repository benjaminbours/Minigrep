use ftp::FtpStream;

pub struct ConnectionConfig {
    host: String,
    user: String,
    password: String,
}

impl ConnectionConfig {
    pub fn new(host: &str, user: &str, password: &str) -> ConnectionConfig {
        ConnectionConfig {
            host: host.to_owned(),
            user: user.to_owned(),
            password: password.to_owned(),
        }
    }
}

pub struct FtpConnection {
    pub stream: FtpStream,
}

impl FtpConnection {
    pub fn new(config: ConnectionConfig) -> FtpConnection {
        let mut ftp_stream = FtpStream::connect(config.host).unwrap();
        let _ = ftp_stream.login(&config.user, &config.password).unwrap();

        FtpConnection { stream: ftp_stream }
        // self.stream.login(user: &str, password: &str)
    }
}
