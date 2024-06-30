mod my_service_provider;

use my_service_provider::MyServiceProvider;
use traits::{AuthenticationService, ServiceProvider, VPNService};

fn try_open_vpn(provider: &MyServiceProvider) -> bool {
    let vpn_service = provider.get_vpn_service();
    match vpn_service.open(provider.get_authentication_service(), "192") {
        Ok(_) => {
            println!("VPN opened");
            true
        }
        Err(_) => {
            println!("VPN not opened");
            false
        }
    }
}

fn main() {
    let service_provider = MyServiceProvider::init();
    try_open_vpn(&service_provider);

    service_provider
        .get_authentication_service()
        .authenticate("ciao", "ciao");

    try_open_vpn(&service_provider);
}
