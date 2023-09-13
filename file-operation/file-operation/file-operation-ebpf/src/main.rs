#![no_std]
#![no_main]
/*
Type            :   Kernel-Space Code
Description     :   Communicate with the kernel and 
                :   take all file system related operation.

Action          :   Restrict file system to access and execute command.
*/

/// The `use` statement is used to bring items into scope so that they can be used without having to
/// specify their full path. In this case, it is importing various items from the `aya_bpf` crate.

use aya_bpf::{
    bindings::path,
    cty::{c_char, c_long},
       
    helpers::bpf_d_path,
    helpers::{bpf_get_current_uid_gid,bpf_get_current_pid_tgid},
    macros::{lsm, map},
    maps::PerCpuArray,
    programs::LsmContext, BpfContext,
};
use aya_log_ebpf::{debug,info,error};
mod vmlinux;
use vmlinux::file;
use vmlinux::task_struct;
use vmlinux::linux_binprm;

use aya_bpf::{helpers::{bpf_probe_read_kernel,bpf_get_current_task,bpf_probe_read,bpf_get_current_comm,}};



#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]


pub const MOUNT_TYPE_LEN: usize = 5;
pub const PATH_LEN: usize = 512;
/// The above code defines a Rust struct called `Path` with a fixed-size array of `u8` representing a
/// path.
/// 
/// Properties:
/// 
/// * `path`: The `path` property is a fixed-size array of `u8` (unsigned 8-bit integers) with a length
/// of `PATH_LEN`.
/// 
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Path {
    pub path: [u8; PATH_LEN],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MountType {
    pub mount_type: [u8; MOUNT_TYPE_LEN],
}

/// The `#[map]` attribute is used to define a BPF map. In this case, it is defining a BPF map called
/// `PATH_BUF` of type `PerCpuArray<Path>`.
/// The `#[map]` attribute is used to define a BPF map. In this case, it is defining a BPF map called
/// `PATH_BUF` of type `PerCpuArray<Path>`.
#[map]
pub(crate) static PATH_BUF: PerCpuArray<Path> = PerCpuArray::with_max_entries(1, 0);
#[map]
pub(crate) static mut MOUNT_TYPE_BUF: PerCpuArray<MountType> = PerCpuArray::with_max_entries(1, 0);
/// The function `my_bpf_d_path` takes a pointer to a path and a mutable byte buffer, and returns the
/// length of the path or an error code.
/// 
/// Arguments:
/// 
/// * `path`: A pointer to a `path` object, which is likely a structure or data type used to represent a
/// file path in the underlying system.
/// * `buf`: `buf` is a mutable reference to a slice of `u8` elements. It is used to store the path name
/// of a file descriptor.
/// 
/// Returns:
/// 
/// a `Result<usize, c_long>`.
#[inline(always)]
pub fn my_bpf_d_path(path: *mut path, buf: &mut [u8]) -> Result<usize, c_long> {
    let ret = unsafe { bpf_d_path(path, buf.as_mut_ptr() as *mut c_char, buf.len() as u32) };
    if ret < 0 {
        return Err(ret);
    }
    Ok(ret as usize)
}

#[lsm(name = "file_open")]
pub fn file_open(ctx: LsmContext) -> i32 {
    match { try_file_open(ctx) } {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}
/// The `try_file_open` function is a helper function that is called by the `file_open` function. It
/// takes a `LsmContext` object as an argument and returns a `Result<i32, i32>`.
fn try_file_open(ctx: LsmContext) -> Result<i32, i32> {
    /// The line `let buf = unsafe { PATH_BUF.get_ptr_mut(0).ok_or(0)? };` is retrieving a mutable
    /// pointer to the first element of the `PATH_BUF` BPF map.
    let buf = unsafe { PATH_BUF.get_ptr_mut(0).ok_or(0)? };
    
    let inode=unsafe{
        let bprm: *const linux_binprm = ctx.arg(0);
        // Filter on the event
    let mode =(*(*(*bprm).file).f_inode).i_mode;
    mode
    };

    /*  The code snippet is retrieving the file path from the `f_path` field of the `file` structure. It
        first converts the `ctx.arg(0)` argument to a pointer to a `file` structure. Then, it obtains a
        mutable pointer to the `f_path` field of the `file` structure. It calls the `my_bpf_d_path`
        function to copy the path from the `f_path` field to the `(*buf).path` array. Finally, it
        converts the copied path to a `&str` using `core::str::from_utf8_unchecked` and returns it.
      */
    let p = unsafe {
        let f: *const file = ctx.arg(0);
    
        let p = &(*f).f_path as *const _ as *mut path;
        //let len = my_bpf_d_path(p, &mut (*buf).path).map_err(|e| e as i32)?;
        let len = my_bpf_d_path(p, &mut (*buf).path).map_err(|e| e as i32)?;
        if len >= PATH_LEN {
            return Err(0);
        }
        core::str::from_utf8_unchecked(&(*buf).path[..len])
    };
    let uid = bpf_get_current_uid_gid() as u32;
    
    //let gid = (bpf_get_current_uid_gid() >> 32) as u32;
    // Filter on the event

        let pid_tgid = bpf_get_current_pid_tgid();
        let t=bpf_get_current_uid_gid();


        if p.starts_with("home/")|| 
        p.starts_with("/usr/bin/mkdir")
     {   
        //info!(&ctx, "inode {} " , inode);
        
       error!(&ctx, "file_open: {}: deny opening {} {}",uid, p,pid_tgid);
      return Err(-1);
     //return Ok(0);
 }               

    
        




////////////
                    
    

        /*
        let mount_type = unsafe {
            let mount_type: *const c_char = ctx.arg(2);
            let buf_ptr = MOUNT_TYPE_BUF.get_ptr_mut(0).ok_or(0)?;
            let buf = &mut *buf_ptr;
            core::str::from_utf8_unchecked(
                bpf_probe_read_kernel_str_bytes(mount_type as *const u8, &mut buf.mount_type)
                    .map_err(|e| e as i32)?,
            )
        };
        info!(&ctx, "{}", mount_type);
         */



  /* 
   let bm=unsafe{ let bprm: *const linux_binprm = ctx.arg(0);
                 bprm
                }; 
    let mode: u16 = unsafe{  (*(*(*bm).file).f_inode).i_mode
                };               
    debug!(&ctx, "{}", mode);
                          
        //let prev_ret: c_int = ctx.arg(1);
        let uid = bpf_get_current_uid_gid() as u32;
        let gid = (bpf_get_current_uid_gid() >> 32) as u32;
    
        // Filter on the event
        let mode: u16 = unsafe{
            (*(*(*bm).file).f_inode).i_mode
        };
        info!(&ctx, "{}", mode);
        
  */
  //handle_enter_execve() ;
            
    Ok(0)
}



#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}