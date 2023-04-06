#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
lazy_static::lazy_static! {
    pub static ref PAGE_SIZE: usize = {
        use libc::{sysconf, _SC_PAGESIZE};
        unsafe { sysconf(_SC_PAGESIZE) as usize }
    };
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
lazy_static::lazy_static! {
    pub static ref PAGE_SIZE: usize = {
        use windows::Win32::System::SystemInformation::{SYSTEM_INFO, GetSystemInfo};
        let mut info = SYSTEM_INFO::default();
        let ptr = std::ptr::addr_of_mut!(info);
        unsafe { GetSystemInfo(ptr); }
        info.dwPageSize as usize
    };
}




pub struct ExecutableMemory {
    allocation: *mut u8,
    capacity: usize,
    size: usize
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
impl Drop for ExecutableMemory {
    fn drop(&mut self) {
        use windows::Win32::System::Memory::{VirtualFree, MEM_RELEASE};
        unsafe {
            VirtualFree(self.allocation as _, self.capacity, MEM_RELEASE);
        }
    }
}

impl ExecutableMemory {
    pub fn new(capacity: usize) -> Self {
        let capacity = ceil(capacity, *PAGE_SIZE);
        let allocation: *mut u8;
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        unsafe {
            let mut ptr: *mut libc::c_void = std::mem::MaybeUninit::uninit().assume_init();
            libc::posix_memalign(&mut ptr, *PAGE_SIZE, capacity);
            libc::mprotect(ptr, capacity, libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE);
            allocation = ptr as _;
        }

        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        unsafe {
            use windows::Win32::System::Memory::{VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};

            let ptr = VirtualAlloc(None, capacity, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
            allocation = ptr as _;
        }

        Self {
            allocation,
            capacity,
            size: 0,
        }
    }

    pub fn push(&mut self, value: u8) {
        assert!(self.size < self.capacity);
        unsafe {
            let index = self.allocation.offset(self.size as isize);
            *index = value;
        }
        self.size += 1;
    }

    pub fn get(self, index: usize) -> u8 {
        assert!(0 < index && index < self.size);
        unsafe {
            let index = self.allocation.offset(index as isize);
            return *index;
        }
    }

    pub fn to_fn(&self) -> fn() -> usize {
        let func: fn() -> usize = unsafe { std::mem::transmute(self.allocation)};
        func
    }
}

fn ceil(size: usize, ceiling: usize) -> usize {
    (size / ceiling + 1) * ceiling
}