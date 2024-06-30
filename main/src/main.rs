use traits::{AuthenticationService, ServiceProvider};

fn main() {
    let service_provider = main::ProjectServiceProvider::default();
    main::try_open_vpn(&service_provider);

    service_provider
        .get_authentication_service()
        .authenticate("ciao", "ciao");

    main::try_open_vpn(&service_provider);
}
