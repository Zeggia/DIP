use traits::{AuthenticationService, VPNService};

pub struct MyVPNService {}

impl Default for MyVPNService {
    fn default() -> Self {
        Self {}
    }
}

impl VPNService for MyVPNService {
    fn open(
        &self,
        auth_service: &impl AuthenticationService,
        endpoint: impl AsRef<str>,
    ) -> Result<(), String> {
        if !auth_service.is_authenticated() {
            return Err("not authanticated".to_string());
        }

        // open vpn logic

        println!("Opening VPN on {}", endpoint.as_ref());

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::MyVPNService;
    use std::cell::RefCell;
    use traits::AuthenticationService;
    use traits::VPNService;

    // RefCell useful also for testing
    struct DummyAuthService(RefCell<bool>);
    impl DummyAuthService {
        fn make_authenticted() -> Self {
            Self(RefCell::new(true))
        }

        fn make_unauthenticted() -> Self {
            Self(RefCell::new(false))
        }
    }
    impl Default for DummyAuthService {
        fn default() -> Self {
            Self(Default::default())
        }
    }
    impl AuthenticationService for DummyAuthService {
        fn authenticate(&self, _username: impl AsRef<str>, _password: impl AsRef<str>) {
            *self.0.borrow_mut() = true;
        }

        fn is_authenticated(&self) -> bool {
            *self.0.borrow()
        }
    }

    #[test]
    fn can_open_vpn_if_authenticated() {
        let dummy_auth_service = DummyAuthService::make_authenticted();
        let vpn_service = MyVPNService::default();

        let open_result = vpn_service.open(&dummy_auth_service, "bla");
        assert!(open_result.is_ok())
    }

    #[test]
    fn cant_open_vpn_if_not_authenticated() {
        let dummy_auth_service = DummyAuthService::make_unauthenticted();
        let vpn_service = MyVPNService::default();

        let open_result = vpn_service.open(&dummy_auth_service, "bla");
        assert_eq!(open_result.is_ok(), false)
    }
}
