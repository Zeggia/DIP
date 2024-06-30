use std::cell::RefCell;

use main::my_service_provider::MyServiceProvider;
use project_authentication::MyAuthenticationService;
use project_vpn::MyVPNService;
use traits::{AuthenticationService, ServiceProvider, VPNService};

struct MockedAuthenticationService {
    called: RefCell<bool>,
    inner: MyAuthenticationService,
}

impl Default for MockedAuthenticationService {
    fn default() -> Self {
        Self {
            called: RefCell::new(false),
            inner: MyAuthenticationService::default(),
        }
    }
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

type TestServiceProvider = MyServiceProvider<MockedAuthenticationService, MyVPNService>;

#[test]
fn authentication_service_is_called_before_opening() {
    let provider = TestServiceProvider::default();

    let _ = provider
        .get_vpn_service()
        .open(provider.get_authentication_service(), "");

    assert!(*provider.get_authentication_service().called.borrow())
}
