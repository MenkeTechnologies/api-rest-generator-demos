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
