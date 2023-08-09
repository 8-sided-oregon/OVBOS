#![no_std]
#![no_main]

#![no_mangle]

use core::panic::PanicInfo;
use ovbos::{print, println};

pub extern "C" fn _start() -> ! {
    unsafe { bubble_sort(); }
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ovbos::test_panic_handler(info)
}

#[test_case]
unsafe fn test_bubble_sort() {
    let mut arr: [i32; 22] = [123,6,345,3245,23,532,452345,2345,3,45,23456,567,5677,576,456,3456,4,56,45,64,56,45];
    let mut newlen: usize;
    let mut len = arr.len();
    loop {
        newlen = 0;
        for i in 1..len {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                newlen = i;
            }
            print!("[{}", arr[0]);
            for i in 1..arr.len()-1 {
                print!("{}, ", arr[i]);
            }
            print!("{}]", arr[arr.len()-1]);

            println!("{}", arr[i]);
        }
        if newlen == 0 {
            println!("should be sorted.");
            break;
        }
        len = newlen;
    }

    for i in 1..arr.len() {
        assert!(arr[i-1] <= arr[i]);
    }
    let arrCopy = arr.clone();
    if arr != arrCopy.sort() {
        panic!("bubble_sort() did not sort correctly");
    }
}