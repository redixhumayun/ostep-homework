use std::ffi::CString;

use libc::O_RDONLY;

#[allow(dead_code)]

static MAX_ITER: u32 = 100;

fn main() {
    unsafe {
        let mut start_time = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        let result = libc::gettimeofday(&mut start_time as *mut _, std::ptr::null_mut());
        assert!(result != -1);

        let mut iter = 0;
        let path = CString::new("Cargo.toml").unwrap();
        let fd = libc::open(path.as_ptr(), O_RDONLY);
        while iter < MAX_ITER {
            let mut buf = [0u8; 1024];
            let read_result = libc::read(fd, buf.as_mut_ptr() as *mut _, buf.len());
            assert!(read_result != -1);
            iter += 1;
        }

        let mut end_time = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        let result = libc::gettimeofday(&mut end_time as *mut _, std::ptr::null_mut());
        assert!(result != -1);
        libc::close(fd);

        let elapsed_secs = end_time.tv_sec - start_time.tv_sec;
        let elapsed_usecs = (end_time.tv_usec - start_time.tv_usec) as i64;
        let elapsed_time = elapsed_secs * 1_000_000 + elapsed_usecs;
        println!("time taken for {} read syscall {}", iter, elapsed_time);
        println!(
            "time taken for a single syscall {}",
            elapsed_time / iter as i64
        );
    }
}
