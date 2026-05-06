use anyhow::{Context, Result};
use kube::{Client, Config};

/// Create a Kubernetes client from optional kubeconfig path and context name.
pub async fn create_client(kubeconfig: Option<&str>, context: Option<&str>) -> Result<Client> {
    let config = match kubeconfig {
        Some(path) => {
            let kubeconfig = kube::config::Kubeconfig::read_from(path)
                .context("failed to read kubeconfig")?;
            let mut options = kube::config::KubeConfigOptions::default();
            options.context = context.map(String::from);
            Config::from_custom_kubeconfig(kubeconfig, &options)
                .await
                .context("failed to build config from kubeconfig")?
        }
        None => {
            if let Some(ctx) = context {
                let mut options = kube::config::KubeConfigOptions::default();
                options.context = Some(ctx.to_string());
                Config::from_kubeconfig(&options)
                    .await
                    .context("failed to infer kubeconfig")?
            } else {
                Config::infer()
                    .await
                    .context("failed to infer Kubernetes config")?
            }
        }
    };

    Client::try_from(config).context("failed to create Kubernetes client")
}

/// Convenience re-exports used throughout the crate.
pub use k8s_openapi::api::apps::v1::{DaemonSet, Deployment, ReplicaSet, StatefulSet};
pub use k8s_openapi::api::batch::v1::{CronJob, Job};
pub use k8s_openapi::api::core::v1::{
    ConfigMap, Namespace, Node, PersistentVolume, PersistentVolumeClaim, Pod, Secret, Service,
    ServiceAccount,
};
pub use k8s_openapi::api::networking::v1::{Ingress, NetworkPolicy};
pub use k8s_openapi::api::rbac::v1::{ClusterRole, ClusterRoleBinding, Role, RoleBinding};
pub use kube::api::{Api, ListParams};