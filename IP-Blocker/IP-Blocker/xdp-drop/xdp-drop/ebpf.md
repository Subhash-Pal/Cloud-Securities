# xdp-drop

User Space code
User Space eBPF:
Runs in the user space of an application.
Executes in a restricted context and operates on data passed from the kernel.
Type            :   Use Application (User Space eBPF)
Name            :   xdp-drop
Description     :   Communication with Kernel Space code xdp-drop-ebpf
                    Load the eBPF object file as raw bytes at compile-time and load it at


### Userspace code for eBPF
```rust
   async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    env_logger::init();
    /*This will include your eBPF object file as raw bytes at compile-time and load it at
    runtime. This approach is recommended for most real-world use cases. If you would
    like to specify the eBPF program at runtime rather than at compile-time, you can
    reach for `Bpf::load_file` instead.
    */

    #[cfg(debug_assertions)]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/debug/xdp-drop"
    ))?;
    #[cfg(not(debug_assertions))]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/xdp-drop"
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

```
