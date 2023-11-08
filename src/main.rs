use std::io::{
    self, Write
};
use std::net::{
    IpAddr, Ipv4Addr
};
use std::sync::mpsc::{
    channel, Sender
};

use tokio::task;
use tokio::net::TcpStream;

use bpaf::Bpaf;


const MAX: u16 = 65535;
const IPFALLBACK: IpAddr = IpAddr::V4(
    Ipv4Addr::new(127, 0, 0, 1)
);


#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments 
{
    #[
        bpaf(
            long, short, 
            argument("Address"), 
            fallback(IPFALLBACK)
        )
    ]
    pub address: IpAddr,

    #[
        bpaf(
            long("start"), short('s'),
            guard(start_port_guard, "Must be greater than 0."),
            fallback(1u16)
        )
    ]
    pub start_port: u16,

    #[
        bpaf(
            long("end"), short('e'),
            guard(end_port_guard, "Must be less than or equal to 65535"),
            fallback(MAX)
        )
    ]
    pub end_port: u16,
}


fn start_port_guard(input: &u16) -> bool
{
    *input > 0
}

fn end_port_guard(input: &u16) -> bool
{
    *input <= MAX
}


async fn scan(
    tx: Sender<u16>, port: u16, addr: IpAddr
)
{
    match TcpStream::connect(
        format!("{}:{}", addr, port)
    ).await
    {
        Ok(_) => {
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }

        Err(_) => {

        }
    }
}


#[tokio::main]
async fn main() 
{
    let options = arguments().run();

    let (tx, rx) = channel();

    for i in options.start_port..options.end_port
    {
        let tx = tx.clone();

        task::spawn(
            async move 
            {
                scan(tx, i, options.address).await
            }
        );
    }

    let mut output = vec![];
    drop(tx);

    for port in rx
    {
        output.push(
            port
        );
    }

    println!("\nOpen Ports:\n...........");
    output.sort();

    for port in output
    {
        println!(
            "{} - open.", port
        )
    }
}
