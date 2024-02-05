use local_ip_address::local_ip;

fn main() {
    let my_local_ip = local_ip().unwrap();
    println!("My local ip address is {:?}", my_local_ip);
}
