use std::thread::sleep;

fn main() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
        pb.finish_with_message("done");
    }

}

fn do_hard_work() {
    let two_seconds = std::time::Duration::new(2,0);
    println!("hard work .... {}", two_seconds.as_secs());
    sleep(two_seconds);
}
