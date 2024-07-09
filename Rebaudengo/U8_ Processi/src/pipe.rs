use std::process::Stdio;
use std::process::Command;
fn main(){
    let echo_child = Command::new("echo")
        .arg("Ciao Mama")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process");

    let echo_out = echo_child.stdout.expect("Failed to open echo stdout");

    let sed_child = Command::new("sed")
        .arg("s/Mama/Mamma/")
        .stdin(Stdio::from(echo_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start sed process");

    let output = sed_child.wait_with_output().expect("Failed to wait on sed");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
