use std::process::Command;

pub fn ssh_ping(ip: &String) {
    let ping = Command::new("ping")
        .args(["-c", "1"])
        .args(["-W", "1"])
        .arg(ip)
        .output()
        .expect("Konnte den Ping nicht starten");
    if ping.status.success() {
        println!("[ OK ] - Ping erfolgreich");
    } else {
        panic!("[ FAILED ] - Konnte das Gerät nicht pingen: {}", ip);
    }
    let ssh_command = format!("root@{}", ip);
    let ssh = Command::new("ssh")
        .arg(&ssh_command)
        .output()
        .expect("Konnte den SSH nicht starten");
    if ssh.status.success() {
        println!("[ OK ] - SSH erfolgreich");
    } else {
        panic!("[ FAILED ] - Konnte das Gerät nicht sshen: {}", ssh_command);
    }
}
