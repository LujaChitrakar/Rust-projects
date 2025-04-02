use std::{env, io::{self, Write}, net::{IpAddr, TcpStream}, process, str::FromStr, sync::mpsc::{channel, Sender}, thread};

const MAX:u16=65535;
struct Arguments{
    flag:String,
    ip_addr:IpAddr,
    threads:u16,
}

impl Arguments {
    fn new(args:&[String])->Result<Arguments,&'static str>{
        if args.len()<2{
            return Err("Not enough arguments");
        }else if args.len()>4{
            return Err("Too many arguments");
        }

        let flag=args[1].clone();

        if let Ok(ip_addr) = IpAddr::from_str(&flag) {
            return Ok(Arguments { flag: String::new(), ip_addr, threads:4 });
        }else{
            if (flag.contains("-h")||flag.contains("-help"))&&args.len()==2{
                println!("Usage -j to select how many threads you want and -help to show help message");
                return Err("help");
            }else if flag.contains("-h")||flag.contains("-help") {
                return Err("too many arguments");
            }else if  flag.contains("-j"){
                let ip_addr=match IpAddr::from_str(&args[3]) {
                    Ok(s)=>s,
                    Err(_)=>return Err("Not a valid IPAddr")
                };
                let threads=match args[2].parse::<u16>() {
                    Ok(s)=>s,
                    Err(_)=>return Err("Failed to parse thread number")
                };
                return Ok(Arguments { flag, ip_addr, threads });
            }else {
                return Err("invalid syntax");
            }
        }
    }
}

fn scan(tx:Sender<u16>,start_port:u16,addr:IpAddr,num_thread:u16){
    let mut port:u16=start_port+1;
    loop{
        match TcpStream::connect((addr,port)) {
            Ok(_)=>{
                println!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_)=>{}
        }
        if(MAX-port)<=num_thread{
            break;
        }
        port+=num_thread;
    }
}

fn main() {
    let args:Vec<String>=env::args().collect();
    let program=args[0].clone();
    let arguments=Arguments::new(&args).unwrap_or_else(
        |err|{
            if err.contains("help"){
                process::exit(0);
            }
            else{
                eprintln!("{} problem parsing arguments: {}",program,err);
                process::exit(0);
            }
        });

    let num_threads=arguments.threads;
    let addr=arguments.ip_addr;
    let (tx,rx)=channel();

    for i in 0..num_threads{
        let tx=tx.clone();

        thread::spawn(move ||{
            scan(tx,i,addr,num_threads);
        });
    }

    let mut out=vec![];
    drop(tx);

    for p in rx{
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out{
        println!("{} is open",v);
    }


}
