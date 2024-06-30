pub mod my_service_provider;

use my_service_provider::MyServiceProvider;
use project_authentication::MyAuthenticationService;
use project_vpn::MyVPNService;
use traits::{ServiceProvider, VPNService};

pub type ProjectServiceProvider = MyServiceProvider<MyAuthenticationService, MyVPNService>;

pub fn try_open_vpn(provider: &ProjectServiceProvider) -> bool {
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
