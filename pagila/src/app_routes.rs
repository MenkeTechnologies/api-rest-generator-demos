// Drop the body of this fn into your `Hooks::routes` impl in `src/app.rs`,
// or call `register_generated_routes(routes)` from there.

use loco_rs::controller::AppRoutes;

pub fn register_generated_routes(routes: AppRoutes) -> AppRoutes {
    routes
        .add_route(crate::controllers::customers::routes())
        .add_route(crate::controllers::actors::routes())
        .add_route(crate::controllers::categorys::routes())
        .add_route(crate::controllers::films::routes())
        .add_route(crate::controllers::film_actors::routes())
        .add_route(crate::controllers::film_categorys::routes())
        .add_route(crate::controllers::address::routes())
        .add_route(crate::controllers::citys::routes())
        .add_route(crate::controllers::countrys::routes())
        .add_route(crate::controllers::inventorys::routes())
        .add_route(crate::controllers::languages::routes())
        .add_route(crate::controllers::payments::routes())
        .add_route(crate::controllers::payment_p2022_01s::routes())
        .add_route(crate::controllers::payment_p2022_02s::routes())
        .add_route(crate::controllers::payment_p2022_03s::routes())
        .add_route(crate::controllers::payment_p2022_04s::routes())
        .add_route(crate::controllers::payment_p2022_05s::routes())
        .add_route(crate::controllers::payment_p2022_06s::routes())
        .add_route(crate::controllers::payment_p2022_07s::routes())
        .add_route(crate::controllers::rentals::routes())
        .add_route(crate::controllers::staffs::routes())
        .add_route(crate::controllers::stores::routes())
}
