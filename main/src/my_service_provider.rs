use project_authentication::MyAuthenticationService;
use project_vpn::MyVPNService;
use traits::ServiceProvider;

pub(super) struct MyServiceProvider {
    authentication_service: MyAuthenticationService,
    vpn_service: MyVPNService,
}

impl MyServiceProvider {
    pub(super) fn init() -> Self {
        Self {
            authentication_service: MyAuthenticationService::new(),
            vpn_service: MyVPNService::new(),
        }
    }
}

impl ServiceProvider<MyAuthenticationService, MyVPNService> for MyServiceProvider {
    fn get_authentication_service(&self) -> &MyAuthenticationService {
        &self.authentication_service
    }

    fn get_vpn_service(&self) -> &MyVPNService {
        &self.vpn_service
    }
}
