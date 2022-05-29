use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vm {
  #[serde(rename = "apiVersion")]
  api_version: String,
  kind: String,
  metadata: VmMetadata,
  spec: VmSpec,
}

#[derive(Debug, Serialize, Deserialize)]
struct VmMetadata {
  name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VmSpec {
  #[serde(rename = "dataVolumeTemplates")]
  data_volume_templates: Vec<DataVolumeTemplate>,
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
  #[serde(rename = "storageClassName")]
  storage_class_name: String,
  #[serde(rename = "accessModes")]
  access_modes: Vec<String>,
  resources: PvcResource,
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
  Illegal,
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
  volumes: Vec<TemplateSpecDomainVolume>,
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
  bus: String,
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
  cloud_init_no_cloud: Option<CloudInitNoCloud>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataVolume {
  name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CloudInitNoCloud {
  #[serde(rename = "userData")]
  user_data: String,
}

impl Vm {
  pub fn new(req: VirtualMachineRequest) -> Self {
    let mut memory = req.spec.memory.clone();
    memory.pop();
    Vm {
      api_version: "v1".into(),
      kind: "VirtualMachine".into(),
      metadata: VmMetadata {
        name: format!("vps-{}", &req.id),
      },
      spec: VmSpec {
        data_volume_templates: vec![DataVolumeTemplate {
          metadata: DataVolumeTemplateMetadata {
            name: format!("vps-{}-datavolume", &req.id),
          },
          spec: DataVolumeTemplateSpec {
            pvc: DataVolumeTemplateSpecPvc {
              storage_class_name: req.storage.class.clone(),
              access_modes: vec!["ReadWriteOnce".into()],
              resources: PvcResource {
                requests: PvcResourceRequests {
                  storage: format!("{}Gi", &req.storage.size),
                },
              },
            },
            source: DataVolumeTemplateSpecSource {
              registry: SpecSourceRegistry {
                url: req.storage.registry.clone(),
              },
            },
          },
        }],
        running: true,
        template: VmSpecTemplate {
          metadata: VmSpecTemplateMetadata {
            labels: VmSpecTemplateMetadataLabels {
              size: kubevirt_size_s(req.spec.cpu.parse().unwrap(), memory.parse().unwrap()),
              domain: req.id.clone(),
            },
          },
          spec: TemplateSpec {
            domain: TemplateSpecDomain {
              devices: TemplateSpecDomainDevices {
                disks: vec![
                  Disk {
                    name: "containerdisk".into(),
                    disk: DiskInfo {
                      bus: "virtio".into(),
                    },
                  },
                  Disk {
                    name: "cloudinitdisk".into(),
                    disk: DiskInfo {
                      bus: "virtio".into(),
                    },
                  },
                ],
                interfaces: vec![Interface {
                  name: "default".into(),
                  masquerade: Masquerade {},
                }],
              },
              networks: vec![TemplateSpecDomainNetwork {
                name: "default".into(),
                pod: DomainNetworkPod {},
              }],
              resources: TemplateSpecDomainResources {
                requests: DomainResourceRequests {
                  cpu: req.spec.cpu,
                  memory: req.spec.memory,
                },
              },
              volumes: vec![
                TemplateSpecDomainVolume {
                  name: "containerdisk".into(),
                  data_volume: Some(DataVolume {
                    name: format!("vps-{}-datavolume", &req.id),
                  }),
                  cloud_init_no_cloud: None,
                },
                TemplateSpecDomainVolume {
                  name: "cloudinitdisk".into(),
                  data_volume: None,
                  cloud_init_no_cloud: Some(CloudInitNoCloud {
                    user_data: format!(
                      r#"#cloud-config
hostname: {}
disable_root: false
ssh_authorized_keys: {:?}
                  "#,
                      req.id, req.cloudinit.ssh_authorized_keys
                    ),
                  }),
                },
              ],
            },
          },
        },
      },
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualMachineRequest {
  id: String,
  storage: VirtualMachineRequestStorage,
  spec: VirtualMachineRequestSpec,
  cloudinit: VirtualMachineRequestCloudInit,
}

#[derive(Debug, Serialize, Deserialize)]
struct VirtualMachineRequestStorage {
  class: String,
  size: String,
  registry: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VirtualMachineRequestSpec {
  cpu: String,
  memory: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VirtualMachineRequestCloudInit {
  ssh_authorized_keys: Vec<String>,
}

fn kubevirt_size_s(cpu: u8, memory: u8) -> LabelSize {
  let n = cpu + memory;
  let mut result = LabelSize::Illegal;
  if n <= 2 {
    result = LabelSize::Demi
  } else if n <= 4 {
    result = LabelSize::Short;
  } else if n <= 6 {
    result = LabelSize::Tall;
  } else if n <= 8 {
    result = LabelSize::Grande;
  } else if n <= 10 {
    result = LabelSize::Venti;
  } else if n <= 12 {
    result = LabelSize::Trenta;
  }
  result
}
