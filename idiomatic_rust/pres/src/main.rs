use std::{ffi::CString, hint::unreachable_unchecked, mem::MaybeUninit, ptr::NonNull};

extern crate libc;

extern "C" {
    fn puts(s: *const i8) -> i32;
}

impl<T> Vec<T> {
    fn extend_map<U, F>(&mut self, us: &[U], mut f: F)
    where
        F: FnMut(&U) -> T,
    {
        self.reserve(us.len());
        let cur_len = self.len();
        unsafe { self.set_len(cur_len + us.len()) };
        let into = unsafe { self.as_mut_ptr().add(cur_len) };
        for u in us {
            unsafe { std::ptr::write(into, f(u)) };
            into += 1;
        }
    }
}

fn main() {
    let mut x = 0;
    let y = &mut x as *mut i32;
    let z = 12;

    unsafe {
        std::ptr::write(y, z);
        assert_eq!(std::ptr::read(y), 12);
    }

    let x = 0u8 as *const i32 as *mut i32;

    let nn = NonNull::new(x);

    if let Some(_) = nn {
        unsafe { unreachable_unchecked() };
    } else {
        println!("NN is null");
    }

    // let mut x = 0u32;
    // let ptr = unsafe { NonNull::new_unchecked(&mut x as *mut _) };

    // // NEVER DO THIS!!! This is undefined behavior. ⚠️
    // let ptr = unsafe { NonNull::<u32>::new_unchecked(std::ptr::null_mut()) };
    // unsafe {
    //     *ptr.as_ptr() = 5;
    // }

    // let mut b: bool = unsafe { MaybeUninit::uninit().assume_init() };
    // b = true;
    // println!("{}", b);

    let cstring = CString::new("Hello, world!").expect("no NULL bytes");

    unsafe {
        puts(cstring.as_ptr());
    }
}
