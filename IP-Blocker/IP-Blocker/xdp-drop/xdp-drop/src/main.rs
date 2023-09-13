/*
User Space code

Runs in the user space of an application.
Executes in a restricted context and operates on data passed from the kernel.
Type            :   User Application (User Space eBPF)
Name            :   xdp-drop
Description     :   Communication with Kernel Space code xdp-drop-ebpf
                    Load the eBPF object file as raw bytes at compile-time and load it at

*/        
///User Space eBPF:
///Runs in the user space of an application.
///Executes in a restricted context and operates on data passed from the kernel.

///Type            :   Use Application (User Space eBPF)
///Name            :   xdp-drop
///Description     :   Communication with Kernel Space code xdp-drop-ebpf
///                    Load the eBPF object file as raw bytes at compile-time and load it at
/*
Header files and lib to load eBPF code,communicate with kernel code ,send and recieve data
 */
use anyhow::Context;
use aya::maps::MapRefMut;
use aya::{
    include_bytes_aligned,
    maps::HashMap,
    programs::{Xdp, XdpFlags},
    Bpf,
};
use aya_log::BpfLogger;
use clap::Parser;
use log::{info, warn};
use std::iter::Map;
use std::ops::Deref;
use tokio::signal;

/*
This user space code is also monitoring the externel file(.json) and notify ,
Header file and lib to watch the file for any changes
 */
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};
use std::net::Ipv4Addr;
use std::path::Path;
use std::thread;
use std::time::Duration;

mod iptool; // Convert the json data and filter the IP address
mod util; // Utility to read the json file
          //mod wacher;
#[derive(Debug, Deserialize, Serialize)]
struct Policy {
    ip: String,
}

#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long, default_value = "eth0")]
    iface: String,
}
///main function : entry point to connect eBPF module
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    env_logger::init();
    /*This will include your eBPF object file as raw bytes at compile-time and load it at
    runtime. This approach is recommended for most real-world use cases. If you would
    like to specify the eBPF program at runtime rather than at compile-time, you can
    reach for `Bpf::load_file` instead.
    */

    #[cfg(debug_assertions)]
    let mut bpf = Bpf::load(include_bytes_aligned!("../../target/bpfel-unknown-none/debug/xdp-drop"
))?;

    #[cfg(not(debug_assertions))]
    let mut bpf = Bpf::load(include_bytes_aligned!("../../target/bpfel-unknown-none/release/xdp-drop"
    ))?;
    
    if let Err(e) = BpfLogger::init(&mut bpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {}", e);
    }
    // Read and load the xdp program from eBPF
    let program: &mut Xdp = bpf.program_mut("xdp").unwrap().try_into()?;
    program.load()?;
    //Attach the Network Interface
    program.attach(&opt.iface, XdpFlags::SKB_MODE)
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    // Connect with map declared in eBPF kernel code to send new IP reading from externel json file
    let mut blocklist: HashMap<_, u32, u32> = HashMap::try_from(bpf.map_mut("BLOCKLIST")?)?;
    ////////////////////////////////////////////////////////
    // Insert an IP for testing purpose
    let block_addr: u32 = Ipv4Addr::new(162, 159, 136, 234).try_into()?;
    blockip(&mut blocklist, block_addr);
    /*
    To implement a file watcher with asynchronous functionality,
    can leverage Rust's asynchronous programming capabilities along with libraries like tokio or async-std. Here's an example using tokio to create an asynchronous file watcher function:
    call            : asyncall function
    parameter       : blocklist to insert new ip
    */
    asyncall(&mut blocklist);

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    info!("Exiting...");

    Ok(())
}

/*fn blockip( map: &mut HashMap<MapRefMut, u32, u32> ) -> Result<(), anyhow::Error>{
    let block_addr: u32 = Ipv4Addr::new(192,168,0,2).try_into()?;
    map.insert(block_addr,0,0);
    Ok(())

} */
/// Type        :   Function
/// Name        :   blockip
/// Parameter   :   HashMap<MapRefMut, u32, u32> accept IP address in key value pair
/// Description :   To insert IP into lookup table of eBPF Kernel code     
fn blockip(map: &mut HashMap<MapRefMut, u32, u32>, block_addr: u32) -> Result<(), anyhow::Error> {
    map.insert(block_addr, 0, 0);
    Ok(())
}
///fn asyncall() to detect any events read ,write ,delete and notify

/// The `asyncall` function is responsible for detecting any events related to file changes (read,
/// write, delete) and notifying the program. It takes a mutable reference to a `HashMap` as a
/// parameter, which is used to insert new IP addresses into the lookup table of the eBPF kernel code.
/// The function sets up a file watcher using the `async_watcher` function and waits for events using
/// the `rx.next().await` method. When an event is received, the function reads the JSON file, extracts
/// the IP addresses, and inserts them into the `HashMap`.
fn asyncall(map: &mut HashMap<MapRefMut, u32, u32>) {
    //let path="./";
    let path = "../../";
    futures::executor::block_on(async {
        if let Err(e) = async_watch(path, map).await {
            print!("error: {:?}", e)
        }
    });
}

/// The `async_watcher()` function creates a new asynchronous file watcher. It sets up a channel for
/// communication between the watcher and the caller, and returns a tuple containing the watcher and a
/// receiver for receiving events from the watcher.
pub fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

/// The `async_watch` function is responsible for setting up a file watcher and detecting any events
/// related to file changes (read, write, delete). It takes a path to the file or directory to be
/// watched and a mutable reference to a `HashMap` as parameters.
pub async fn async_watch<P: AsRef<Path>>(
    path: P,
    map: &mut HashMap<MapRefMut, u32, u32>,
) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    println!("Watcher called ");
    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                println!("changed: {:?}", event);
                thread::sleep(Duration::from_millis(2000));
                let data = util::readjson().unwrap();
                //println!("{:?}", data);
                let parsed_data: Value = serde_json::from_value(data).unwrap();
                //println!("Parsed data: {:?}", parsed_data[0]["ip"]);

                if let Some(array) = parsed_data.as_array() {
                    // Iterate over the elements in the array
                    for element in array {
                        // Access the values in each element
                        let ip = element["ip"].as_str().unwrap();

                        // Do something with the values
                        //println!("ip: {}", ip);
                        let ip: Result<Ipv4Addr, _> = ip.parse();

                        match ip {
                            Ok(ip_addr) => {
                                // Successfully parsed the IP address
                                println!("Parsed IP: {}", ip_addr);
                                let octets = ip_addr.octets();
                                println!("{:?}", octets);
                                let ip = iptool::create_ipv4_addr(ip.unwrap()).unwrap();

                                map.insert(ip, 0, 0);
                            }
                            Err(_) => {
                                // Failed to parse the IP address
                                println!("Invalid IP address");
                            }
                        }
                    }
                } else {
                    println!("Invalid JSON array");
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

/*
use std::net::Ipv4Addr;

use aya::map::{HashMap, MapRefMut};

fn block_ip(map: &mut HashMap<MapRefMut, u32, u32>, address: Ip) {
    let address: u32 = address.into();
    map.insert(address, 0);
}

fn main() {
    [here goes your usual code to load and attach the program]

    let mut blocklist: HashMap<_, u32, u32> =
        HashMap::try_from(bpf.map_mut("BLOCKLIST")?)?;

    block_ip(&mut blocklist, Ipv4Addr::new(192, 168, 0, 100));
}
 */
/*
let mut only = ||-> Result<(), anyhow::Error>{
    println!("From closure: ");
    let block_addr: u32 = Ipv4Addr::new(192,168,29,123).try_into()?;
    blocklist.insert(block_addr, 0, 0);

    Ok(())
};

only();
*/
