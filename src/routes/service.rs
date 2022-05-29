use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
  #[serde(rename = "apiVersion")]
  api_version: String,
  kind: String,
  metadata: ServiceMetadata,
  spec: ServiceSpec,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServiceMetadata {
  name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServiceSpec {
  r#type: String,
  selector: ServiceSpecSelector,
  ports: Vec<ServiceSpecPort>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServiceSpecSelector {
  #[serde(rename = "kubevirt.io/domain")]
  domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServiceSpecPort {
  protocol: String,
  name: String,
  port: String,
  #[serde(rename = "targetPort")]
  target_port: String,
}

impl Service {
  pub fn new(req: ServiceRequest) -> Service {
    let mut ports = vec![];
    for port in &req.ports {
      ports.push(ServiceSpecPort {
        protocol: req.protocol.clone(),
        name: format!("{}{}", &req.protocol.to_ascii_lowercase(), &port),
        port: port.into(),
        target_port: port.into(),
      })
    }
    Service {
      api_version: "v1".into(),
      kind: "Service".into(),
      metadata: ServiceMetadata {
        name: format!("vps-{}-svc-{}", &req.id, &req.protocol.to_ascii_lowercase()),
      },
      spec: ServiceSpec {
        r#type: "LoadBalancer".into(),
        selector: ServiceSpecSelector { domain: req.id },
        ports,
      },
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceRequest {
  id: String,
  protocol: String,
  ports: Vec<String>,
}
