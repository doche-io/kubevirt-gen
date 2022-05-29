use actix_web::{post, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Vm {
  apiVersion: String,
  kind: VmKind,
  metadata: VmMetadata,
  spec: VmSpec,
}

#[derive(Debug, Serialize, Deserialize)]
enum VmKind {
  Service,
  VirtualMachine,
}

#[derive(Debug, Serialize, Deserialize)]
struct VmMetadata {
  name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VmSpec {
  dataVolumeTemplates: Vec<DataVolumeTemplate>,
  running: bool,
  template: VmSpecTemplate,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataVolumeTemplate {
  metadata: DataVolumeTemplateMetadata,
  spec: DataVolumeTemplateSpec,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataVolumeTemplateMetadata {
  name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataVolumeTemplateSpec {
  pvc: DataVolumeTemplateSpecPvc,
  source: DataVolumeTemplateSpecSource,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataVolumeTemplateSpecPvc {
  storageClassName: String,
  accessModes: Vec<PvcAccesMode>,
  resources: Vec<PvcResource>,
}

#[derive(Debug, Serialize, Deserialize)]
enum PvcAccesMode {
  ReadWriteOnce,
}

#[derive(Debug, Serialize, Deserialize)]
struct PvcResource {
  requests: PvcResourceRequests,
}

#[derive(Debug, Serialize, Deserialize)]
struct PvcResourceRequests {
  storage: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataVolumeTemplateSpecSource {
  registry: SpecSourceRegistry,
}

#[derive(Debug, Serialize, Deserialize)]
struct SpecSourceRegistry {
  url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VmSpecTemplate {
  metadata: VmSpecTemplateMetadata,
  spec: TemplateSpec,
}

#[derive(Debug, Serialize, Deserialize)]
struct VmSpecTemplateMetadata {
  labels: VmSpecTemplateMetadataLabels,
}

#[derive(Debug, Serialize, Deserialize)]
struct VmSpecTemplateMetadataLabels {
  #[serde(rename = "kubevirt.io/size")]
  size: LabelSize,
  #[serde(rename = "kubevirt.io/domain")]
  domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum LabelSize {
  Demi,
  Short,
  Tall,
  Grande,
  Venti,
  Trenta,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateSpec {
  domain: TemplateSpecDomain,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateSpecDomain {
  devices: TemplateSpecDomainDevices,
  resources: TemplateSpecDomainResources,
  networks: Vec<TemplateSpecDomainNetwork>,
  volumes: Vec<TemplateSpecDomainVolume>
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateSpecDomainDevices {
  disks: Vec<Disk>,
  interfaces: Vec<Interface>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Disk {
  name: String,
  disk: DiskInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct DiskInfo {
  bus: DiskBusType,
}

#[derive(Debug, Serialize, Deserialize)]
enum DiskBusType {
  Virtio,
}

#[derive(Debug, Serialize, Deserialize)]
struct Interface {
  name: String,
  masquerade: Masquerade,
}

#[derive(Debug, Serialize, Deserialize)]
struct Masquerade {}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateSpecDomainResources {
  requests: DomainResourceRequests,
}

#[derive(Debug, Serialize, Deserialize)]
struct DomainResourceRequests {
  memory: String,
  cpu: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateSpecDomainNetwork {
  name: String,
  pod: DomainNetworkPod,
}

#[derive(Debug, Serialize, Deserialize)]
struct DomainNetworkPod {}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateSpecDomainVolume {
  name: String,
  #[serde(rename = "dataVolume")]
  data_volume: Option<DataVolume>,
  #[serde(rename = "cloudInitNoCloud")]
  cloud_init_no_cloud: Option<CloudInitNoCloud>
}

#[derive(Debug, Serialize, Deserialize)]
struct DataVolume {
  name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct CloudInitNoCloud {
  #[serde(rename = "userData")]
  user_data: String
}