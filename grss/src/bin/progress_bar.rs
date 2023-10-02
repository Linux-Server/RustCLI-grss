fn main() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100000 {
        do_hard_work();
        pb.println(format!("[{i}/100000] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}


fn do_hard_work(){
}