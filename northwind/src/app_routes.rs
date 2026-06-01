// Drop the body of this fn into your `Hooks::routes` impl in `src/app.rs`,
// or call `register_generated_routes(routes)` from there.

use loco_rs::controller::AppRoutes;

pub fn register_generated_routes(routes: AppRoutes) -> AppRoutes {
    routes
        .add_route(crate::controllers::employees::routes())
        .add_route(crate::controllers::categories::routes())
        .add_route(crate::controllers::customers::routes())
        .add_route(crate::controllers::shippers::routes())
        .add_route(crate::controllers::suppliers::routes())
        .add_route(crate::controllers::orders::routes())
        .add_route(crate::controllers::products::routes())
        .add_route(crate::controllers::order_details::routes())
        .add_route(crate::controllers::customer_customer_demos::routes())
        .add_route(crate::controllers::customer_demographics::routes())
        .add_route(crate::controllers::regions::routes())
        .add_route(crate::controllers::territories::routes())
        .add_route(crate::controllers::employee_territories::routes())
}
