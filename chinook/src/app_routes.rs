// Drop the body of this fn into your `Hooks::routes` impl in `src/app.rs`,
// or call `register_generated_routes(routes)` from there.

use loco_rs::controller::AppRoutes;

pub fn register_generated_routes(routes: AppRoutes) -> AppRoutes {
    routes
        .add_route(crate::controllers::albums::routes())
        .add_route(crate::controllers::artists::routes())
        .add_route(crate::controllers::customers::routes())
        .add_route(crate::controllers::employees::routes())
        .add_route(crate::controllers::genres::routes())
        .add_route(crate::controllers::invoices::routes())
        .add_route(crate::controllers::invoice_lines::routes())
        .add_route(crate::controllers::media_types::routes())
        .add_route(crate::controllers::playlists::routes())
        .add_route(crate::controllers::playlist_tracks::routes())
        .add_route(crate::controllers::tracks::routes())
}
