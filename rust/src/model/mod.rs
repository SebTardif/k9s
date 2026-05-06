pub mod resource;

use std::fmt;

/// The kind of Kubernetes resource currently being viewed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Pod,
    Deployment,
    Service,
    Namespace,
    Node,
    ConfigMap,
    Secret,
    StatefulSet,
    DaemonSet,
    ReplicaSet,
    Job,
    CronJob,
    Ingress,
    PersistentVolume,
    PersistentVolumeClaim,
    ServiceAccount,
    Role,
    RoleBinding,
    ClusterRole,
    ClusterRoleBinding,
    NetworkPolicy,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Pod => "pods",
            Self::Deployment => "deployments",
            Self::Service => "services",
            Self::Namespace => "namespaces",
            Self::Node => "nodes",
            Self::ConfigMap => "configmaps",
            Self::Secret => "secrets",
            Self::StatefulSet => "statefulsets",
            Self::DaemonSet => "daemonsets",
            Self::ReplicaSet => "replicasets",
            Self::Job => "jobs",
            Self::CronJob => "cronjobs",
            Self::Ingress => "ingresses",
            Self::PersistentVolume => "persistentvolumes",
            Self::PersistentVolumeClaim => "persistentvolumeclaims",
            Self::ServiceAccount => "serviceaccounts",
            Self::Role => "roles",
            Self::RoleBinding => "rolebindings",
            Self::ClusterRole => "clusterroles",
            Self::ClusterRoleBinding => "clusterrolebindings",
            Self::NetworkPolicy => "networkpolicies",
        };
        write!(f, "{s}")
    }
}

impl Resource {
    /// Short alias used in the command bar (mirrors k9s aliases).
    pub fn from_alias(alias: &str) -> Option<Self> {
        match alias {
            "po" | "pod" | "pods" => Some(Self::Pod),
            "dp" | "deploy" | "deployments" => Some(Self::Deployment),
            "svc" | "service" | "services" => Some(Self::Service),
            "ns" | "namespace" | "namespaces" => Some(Self::Namespace),
            "no" | "node" | "nodes" => Some(Self::Node),
            "cm" | "configmap" | "configmaps" => Some(Self::ConfigMap),
            "sec" | "secret" | "secrets" => Some(Self::Secret),
            "sts" | "statefulset" | "statefulsets" => Some(Self::StatefulSet),
            "ds" | "daemonset" | "daemonsets" => Some(Self::DaemonSet),
            "rs" | "replicaset" | "replicasets" => Some(Self::ReplicaSet),
            "job" | "jobs" => Some(Self::Job),
            "cj" | "cronjob" | "cronjobs" => Some(Self::CronJob),
            "ing" | "ingress" | "ingresses" => Some(Self::Ingress),
            "pv" | "persistentvolume" | "persistentvolumes" => Some(Self::PersistentVolume),
            "pvc" | "persistentvolumeclaim" | "persistentvolumeclaims" => {
                Some(Self::PersistentVolumeClaim)
            }
            "sa" | "serviceaccount" | "serviceaccounts" => Some(Self::ServiceAccount),
            _ => None,
        }
    }
}