extern crate wio;

use std::mem;
use std::ffi::OsString;
use self::wio::wide::FromWide;
use std::ptr;

pub struct Process {
    pub name : String,
    pub pid  : winapi::shared::minwindef::DWORD
}

impl Process {
   pub fn new(name: &str) -> Process {
       return Process { 
           name : name.to_string(),
           pid  : Process::get_id_process(&name[..]) 
        }
   }

   pub fn read_address (&self, address: &str) -> u64 {
        let mut num = unsafe { mem::MaybeUninit::uninit().assume_init() };
        let addr = u64::from_str_radix(&address[2..], 16).unwrap();
        let value = unsafe {
            winapi::um::tlhelp32::Toolhelp32ReadProcessMemory(
                                        self.pid,
                                        addr as *const _,
                                        &mut num as *mut _ as *mut _,
                                        mem::size_of::<u64>() as winapi::shared::basetsd::SIZE_T,
                                        ptr::null_mut())
        };
        match value {
            1 => return num,
            _ => return 0
        };
    }

    fn get_id_process(name: &str) -> u32 {
        let mut process: winapi::um::tlhelp32::PROCESSENTRY32W = unsafe{mem::MaybeUninit::uninit().assume_init()}; 
        process.dwSize = mem::size_of::<winapi::um::tlhelp32::PROCESSENTRY32W>() as u32;
        
        let snapshot = unsafe{winapi::um::tlhelp32::CreateToolhelp32Snapshot(winapi::um::tlhelp32::TH32CS_SNAPPROCESS, 0)};
            
        if unsafe{winapi::um::tlhelp32::Process32FirstW(snapshot, &mut process)} != 0{
            while unsafe{winapi::um::tlhelp32::Process32NextW(snapshot, &mut process)} != 0 {    
                
                let process_name = OsString::from_wide(&process.szExeFile);
    
                match process_name.into_string() {
                    Ok(s) => {
                        if s.contains(name) {
                            return process.th32ProcessID;
                        }
                    },               
                    Err(_) => {
                        println!("Error converting process name for PID {}", process.th32ProcessID);
                    }          
                }            
            }
        }
        return 0;
    }
}

