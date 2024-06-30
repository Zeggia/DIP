use std::cell::RefCell;
use traits::AuthenticationService;

#[derive(Clone)]
enum AuthenticationState {
    Authenticated,
    NotAuthenticated,
}

pub struct MyAuthenticationService {
    status: RefCell<AuthenticationState>, // other things...
}

impl MyAuthenticationService {
    fn set_authentication_state(&self, new_state: AuthenticationState) {
        *self.status.borrow_mut() = new_state;
    }

    fn get_authentication_state(&self) -> AuthenticationState {
        self.status.borrow().clone()
    }
}

impl Default for MyAuthenticationService {
    fn default() -> Self {
        Self {
            status: RefCell::new(AuthenticationState::NotAuthenticated),
        }
    }
}

impl AuthenticationService for MyAuthenticationService {
    fn authenticate(&self, username: impl AsRef<str>, password: impl AsRef<str>) {
        // auth logic

        if username.as_ref() == "ciao" && password.as_ref() == "ciao" {
            self.set_authentication_state(AuthenticationState::Authenticated)
        }
    }

    fn is_authenticated(&self) -> bool {
        match self.get_authentication_state() {
            AuthenticationState::Authenticated => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::MyAuthenticationService;
    use traits::AuthenticationService;

    #[test]
    fn initial_state_is_not_authenticated() {
        let auth_service = MyAuthenticationService::default();
        assert_eq!(auth_service.is_authenticated(), false)
    }

    #[test]
    fn right_credentials_authenticate_properly() {
        let auth_service = MyAuthenticationService::default();
        auth_service.authenticate("ciao", "ciao");

        assert!(auth_service.is_authenticated())
    }

    #[test]
    fn wrong_credentials_doesnt_authenticate() {
        let auth_service = MyAuthenticationService::default();
        auth_service.authenticate("wrong", "wrong");

        assert_eq!(auth_service.is_authenticated(), false)
    }
}
