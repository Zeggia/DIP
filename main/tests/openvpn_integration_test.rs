use std::cell::RefCell;

use project_authentication::MyAuthenticationService;
use project_vpn::MyVPNService;
use traits::{AuthenticationService, ServiceProvider, VPNService};

struct MockedAuthenticationService {
    called: RefCell<bool>,
    inner: MyAuthenticationService,
}

impl AuthenticationService for MockedAuthenticationService {
    fn authenticate(&self, username: impl AsRef<str>, password: impl AsRef<str>) {
        self.inner.authenticate(username, password)
    }

    fn is_authenticated(&self) -> bool {
        *self.called.borrow_mut() = true;
        self.inner.is_authenticated()
    }
}

struct TestServiceProvider {
    auth_service: MockedAuthenticationService,
    vpn_service: MyVPNService,
}

impl ServiceProvider<MockedAuthenticationService, MyVPNService> for TestServiceProvider {
    fn get_authentication_service(&self) -> &MockedAuthenticationService {
        &self.auth_service
    }

    fn get_vpn_service(&self) -> &MyVPNService {
        &self.vpn_service
    }
}

#[test]
fn authentication_service_is_called_before_opening() {
    let provider = TestServiceProvider {
        auth_service: MockedAuthenticationService {
            called: RefCell::new(false),
            inner: MyAuthenticationService::new(),
        },
        vpn_service: MyVPNService::new(),
    };

    let _ = provider
        .get_vpn_service()
        .open(provider.get_authentication_service(), "");

    assert!(*provider.get_authentication_service().called.borrow())
}
