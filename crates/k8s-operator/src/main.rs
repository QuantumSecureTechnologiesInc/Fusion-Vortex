// src/main.rs
use anyhow::Result;
use futures::StreamExt;
use kube::runtime::controller::{Context, Controller};
use kube::{Api, Client, ResourceExt};
use std::sync::Arc;
use tracing_subscriber::{fmt, EnvFilter};

mod crd;
use crd::{FusionApp, FusionAppSpec, FusionAppStatus};

#[derive(Clone)]
struct OperatorContext {
    client: Client,
}

async fn reconcile(fusion_app: Arc<FusionApp>, ctx: Context<OperatorContext>) -> Result<()> {
    let name = fusion_app.name_any();
    tracing::info!("Reconciling FusionApp {}", name);
    // Placeholder logic: just set status to Ready
    let api: Api<FusionApp> = Api::namespaced(ctx.get_ref().client.clone(), "default");
    let mut status = FusionAppStatus {
        available_replicas: 0,
        quantum_job_id: None,
        phase: "Ready".into(),
    };
    // Simulate setting available replicas
    status.available_replicas = fusion_app.spec.replicas;
    let pp = kube::api::PatchParams::apply("fusion-operator");
    let patch = kube::api::Patch::Apply(&serde_json::json!({"status": status}));
    api.patch_status(&name, &pp, &patch).await?;
    Ok(())
}

fn error_policy(
    _error: &anyhow::Error,
    _ctx: Context<OperatorContext>,
) -> kube::runtime::controller::Requeue {
    // Requeue after 5 seconds on error
    kube::runtime::controller::Requeue::After(std::time::Duration::from_secs(5))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise logger
    fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let client = Client::try_default().await?;
    let ctx = Context::new(OperatorContext {
        client: client.clone(),
    });

    let crds = Api::<FusionApp>::all(client.clone());
    // Ensure CRD is installed (skip if already present)
    // In real deployment, you would apply the CRD yaml separately.

    Controller::new(crds, Default::default())
        .run(reconcile, error_policy, ctx)
        .for_each(|res| async move {
            match res {
                Ok(o) => tracing::info!("Reconciled: {}", o.name_any()),
                Err(e) => tracing::error!("Reconcile error: {}", e),
            }
        })
        .await;
    Ok(())
}
