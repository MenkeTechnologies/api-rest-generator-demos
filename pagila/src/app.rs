use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::Queue,
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use std::path::Path;

#[allow(unused_imports)]
use crate::{controllers, tasks};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
        // BEGIN loco-gen routes (auto-generated, do not edit)
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
        // END loco-gen routes
            .add_route(controllers::home::routes())
    }
    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        // tasks-inject (do not remove)
    }
    async fn truncate(_ctx: &AppContext) -> Result<()> {
        Ok(())
    }
    async fn seed(_ctx: &AppContext, _base: &Path) -> Result<()> {
        Ok(())
    }
}
