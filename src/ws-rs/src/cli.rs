use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)] //TODO: fill values
pub struct Args {
    #[arg(
        short = 'p',
        long = "port",
        help = "Local port to bind to (9090 if unset)"
    )]
    port: Option<u16>,
    #[arg(
        short = 'a',
        long = "address",
        help = "Address to bind to (binds on all addresses if unset)"
    )]
    address: Option<String>,
    #[arg(long = "no-tls", help = "Don't use TLS")]
    no_tls: bool,
    #[arg(
        long = "for-tls-proxy",
        help = "Act behind a https-terminating proxy: accept only https:// origins by default"
    )]
    tls_proxy: Option<String>,
    #[arg(long = "local-ssh", help = "Log in locally via SSH")]
    ssh: bool,
    #[arg(
        long = "local-session",
        help = "Launch a bridge in the local session (path to cockpit-bridge or '-' for stdin/out); implies --no-tls"
    )]
    session: bool,
}

impl Args {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(9090)
    }

    pub fn address(&self) -> String {
        match self.address.clone() {
            Some(address) => address,
            None => String::from("0.0.0.0"),
        }
    }
}
