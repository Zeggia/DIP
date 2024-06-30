use traits::{AuthenticationService, ServiceProvider, VPNService};

pub struct MyServiceProvider<T: AuthenticationService, K: VPNService> {
    authentication_service: T,
    vpn_service: K,
}

impl<T: AuthenticationService, K: VPNService> Default for MyServiceProvider<T, K> {
    fn default() -> Self {
        Self {
            authentication_service: Default::default(),
            vpn_service: Default::default(),
        }
    }
}

impl<T: AuthenticationService, K: VPNService> ServiceProvider<T, K> for MyServiceProvider<T, K> {
    fn get_authentication_service(&self) -> &T {
        &self.authentication_service
    }

    fn get_vpn_service(&self) -> &K {
        &self.vpn_service
    }
}
