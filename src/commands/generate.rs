use crate::keygen::ethereum::get_random_keys_and_address;

pub fn run(network: &str) {
    match network {
        "ethereum" => {
            let (pvk, pbk, address) = get_random_keys_and_address();
            print_output(network, &pvk, &pbk, &address);
        }
        _ => panic!("Error: Unsupported network!"),
    }
}

fn print_output(network: &str, private_key: &str, public_key: &str, address: &str) {
    println!("Generating keys valid for {network}.\n");

    println!("Private Key: {private_key}");
    println!("Public Key:  {public_key}");
    println!("Address:     {address}");
}
