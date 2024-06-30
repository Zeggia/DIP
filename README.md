# Dependency Inversion
Simple dependency inversion between three actors:
1. ServiceProvider which owns the services
2. AuthenticationService
3. VPNService that needs the AutenticationService

The `main` binary project uses the services implemented in `project-vpn` lib and `project-authentication` lib to create a custom Service Provider `MyServiceProvider`.

Each component is tested indipendently. The integration test tets the usage of `MyServiceProvider` to perform the vpn opening 
