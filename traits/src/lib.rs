pub trait AuthenticationService: Default {
    fn authenticate(&self, username: impl AsRef<str>, password: impl AsRef<str>);
    fn is_authenticated(&self) -> bool;
}

pub trait VPNService: Default {
    fn open(
        &self,
        auth_service: &impl AuthenticationService,
        endpoint: impl AsRef<str>,
    ) -> Result<(), String>;
}

pub trait ServiceProvider<T: AuthenticationService, K: VPNService> {
    fn get_authentication_service(&self) -> &T;
    fn get_vpn_service(&self) -> &K;
}
