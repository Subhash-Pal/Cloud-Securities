#![no_std]
#![no_main]
/* 
Type            :   eBPF Component/Kernel Space
Name            :   xdp-drop-ebpf
Description     :   XDP (eXpress Data Path) programs permit our eBPF program to make decisions about packets that have been received on the interface to which our program is attached. To keep things simple, we'll build a very simplistic firewall to permit or deny traffic.
                    Dropping packets and Block IP in eBPF
                    We will create a new map called BLOCKLIST in our eBPF code. 
                    In order to make the policy decision, we will need to lookup the source IP address in our HashMap. If it exists we drop the packet, if it does not, we allow it. We'll keep this logic in a function called block_ip.
                    Here's what the code looks like now:
*/

#![allow(nonstandard_style, dead_code)]

/// Header files and Library to create eBPF Code 
use aya_bpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::HashMap,
    programs::XdpContext,
};

use aya_log_ebpf::info;

use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::Ipv4Hdr,
};

/*
        Type        :   Function 
        Name        :   panic
        Parameter   :   core::panic::PanicInfo(A struct providing information about a panic.)
        Description :   Function call when panic exception occured .
*/

/// The above function is a panic handler in Rust that uses the `unreachable_unchecked` function from
/// the `core::hint` module.
/// 
/// Arguments:
/// 
/// * `_info`: The `_info` parameter is of type `core::panic::PanicInfo`. It contains information about
/// the panic, such as the file and line number where the panic occurred, the panic message, and any
/// associated payload.
#[panic_handler]
/// The function `panic` is a Rust function that is used to handle panics by indicating that the code
/// execution should never reach that point.
/// 
/// Arguments:
/// 
/// * `_info`: The `_info` parameter is of type `core::panic::PanicInfo`. It represents information
/// about the panic that occurred, such as the file and line number where the panic was triggered, and
/// any custom panic message that was provided.
/// The function `panic` is a Rust function that is used to handle panics by indicating that the code
/// execution should never reach that point.
/// 
/// Arguments:
/// 
/// * `_info`: The `_info` parameter is of type `core::panic::PanicInfo`. It represents information
/// about the panic that occurred, such as the file and line number where the panic was triggered, and
/// any custom panic message that was provided.
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

/*   Type           :   map
     Name           :   BLOCKLIST
     Description    :   map called BLOCKLIST in our eBPF code. 
                        In order to make the policy decision, we will need to lookup the source IP address in our HashMap. 
                        If it exists we drop the packet, if it does not, we allow it. 
                        We'll keep this logic in a function called block_ip
 */
///map is lookup table to store IP address to be blocked
#[map(name = "BLOCKLIST")] // (1)
static mut BLOCKLIST: HashMap<u32, u32> =
    HashMap::<u32, u32>::with_max_entries(1024, 0);
/// The above function is an XDP program that acts as a firewall by applying a set of rules to incoming
/// packets.
/// 
/// Arguments:
/// 
/// * `ctx`: The `ctx` parameter in the `xdp_firewall` function is of type `XdpContext`. It represents
/// the context in which the XDP (eXpress Data Path) program is executed. The specific details of the
/// `XdpContext` type are not provided in the code snippet
/// 
/// Returns:
/// 
/// The function `xdp_firewall` returns a `u32` value.
#[xdp]
pub fn xdp_firewall(ctx: XdpContext) -> u32 {
    match try_xdp_firewall(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}
/*      Type            :   Function
        Name            :   ptr_at
        Parameter       :   XdpContext, offset(To read Source/Destination IP and offset )
        Description     :   Here we define ptr_at to ensure that packet access is always bound checked.
 */
/// The `ptr_at` function in Rust returns a pointer to a value of type `T` at a given offset in the
/// XdpContext data, or an error if the offset is out of bounds.
/// 
/// Arguments:
/// 
/// * `ctx`: A reference to an XdpContext object.
/// * `offset`: The `offset` parameter is the number of bytes to offset from the start of the data
/// buffer (`ctx.data()`). It is used to calculate the memory address of the desired element of type
/// `T`.
/// 
/// Returns:
/// 
/// The function `ptr_at` returns a `Result` type. If the conditions in the function are met, it returns
/// an `Ok` variant containing a pointer to the requested type `T`. If the conditions are not met, it
/// returns an `Err` variant.
#[inline(always)]
unsafe fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    let ptr = (start + offset) as *const T;
    Ok(&*ptr)
}

/*
    Type        :   Function
    Name        :   block_ip        
    Description :    Check if we should allow or deny our packet

 */
// (2)
/// Type        :   Function
/// Name        :   block_ip
/// Parameter   :   address: u32 (unsigned 32 bit )
/// Description :   Function is taking IP address as parameter and Check its availability in the look table  
fn block_ip(address: u32) -> bool {
    //unsafe { BLOCKLIST.get(&address).is_some() }
    unsafe{BLOCKLIST.get(&address).is_some()}
}
 /* Type        :   Function
    Name        :   try_xdp_firewall        
    Description :   Reading/inspect Data Packets,Source/Destination IP address,Network Interface,ports,
                    protocols from incoming/outgoing data packets.
                    Through BLOCKLIST map eBPF Kernel code is receiving IP to be bloked and store in the lookup table.

                    We'll block all traffic originating from the IP found in the BLOCKLIST map.  
                     
 */
fn try_xdp_firewall(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };
    match unsafe { (*ethhdr).ether_type } {
        EtherType::Ipv4 => {}
        _ => return Ok(xdp_action::XDP_PASS),
    }

    let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
    let source =  u32::from_be(unsafe { (*ipv4hdr).src_addr });
    let destination=u32::from_be(unsafe { (*ipv4hdr).dst_addr });
    // Call block_ip function which is taking source IP as parameter and check whether its available in BLOCKLIST or not 
        // (3)
    let action = if block_ip(source) {
        xdp_action::XDP_DROP  // Packet drop if source IP found in the BLOCKLIST
    } else {
        xdp_action::XDP_PASS // Allow if not 
    };
    
    // Log the Source/Destination  IP
    info!(
        &ctx,
        "AF_INET6 src addr: {:ipv4}, dest addr: {:ipv4}",
        source,
        destination
    );
    
    // Log the Source IP and Action to br Taken 1: Drop 2: Allow
    info!(&ctx, "SRC: {:ipv4}, ACTION: {}", source, action);

    

    Ok(action)
}
