#![no_std]
#![no_main]

mod vga_buffer;

use core::arch::asm;
use core::panic::PanicInfo;
use backtrace::Backtrace;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", info);
    println!("{:?}", Backtrace::new());
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("wsg cuh{}", "");
    println!("i lov emen{}", "");

    bubble_sort();


    let asd = 1/0;
    println!("{}", asd);

    loop {}
}

fn bubble_sort() {
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
            println!("we cracked{}", "");
            break;
        }

        len = newlen;
    }
}