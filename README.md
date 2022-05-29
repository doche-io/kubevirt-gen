# kubevirt-gen
kubevirtのYamlを生成します。

## Build
```bash
cargo build --release
```
リリースフラグを付けると幸せになれます。 <br>
ビルド成果物はカレントディレクトリの`target/release/kubevirt-gen`に排出されます。

## Test environment
テスト環境。
```
OS: Arch Linux x86_64
Kernel: 5.4.0-113-generic
Host: ProLiant DL360 Gen9
CPU: Intel Xeon E5-2680 v3 (48) @ 3.300GHz
Memory: 48GiB
```

## Usage
Httpサーバーとして動きます。起動方法は以下の通りです。
```bash
./kubevirt-gen --port <port> --bind <ip address>
```

### POST /virtualmachine
```bash
curl -X POST http://localhost:3000/virtualmachine -H "Accept: application/json" -H "Content-type: application/json" \
-d '{"id": "test", "storage": {"class": "nfs-client", "size": "10", "registry": "docker://quay.io/containerdisks/ubuntu:22.04"},"spec": {"cpu": "1", "memory": "1G"}, "cloudinit": {"ssh_authorized_keys": ["ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPloV8xFkoybv0ztf1pObVM7Yt3JnP6iZH7xcUFIs9xT root@DESKTOP-K0IKF3F"]}}'
```
返値
```yml
---
apiVersion: v1
kind: VirtualMachine
metadata:
  name: vps-test
spec:
  dataVolumeTemplates:
    - metadata:
        name: vps-test-datavolume
      spec:
        pvc:
          storageClassName: nfs-client
          accessModes:
            - ReadWriteOnce
          resources:
            requests:
              storage: 10Gi
        source:
          registry:
            url: "docker://quay.io/containerdisks/ubuntu:22.04"
  running: true
  template:
    metadata:
      labels:
        kubevirt.io/size: Demi
        kubevirt.io/domain: test
    spec:
      domain:
        devices:
          disks:
            - name: containerdisk
              disk:
                bus: virtio
            - name: cloudinitdisk
              disk:
                bus: virtio
          interfaces:
            - name: default
              masquerade: {}
        resources:
          requests:
            memory: 1G
            cpu: "1"
        networks:
          - name: default
            pod: {}
        volumes:
          - name: containerdisk
            dataVolume:
              name: vps-test-datavolume
          - name: cloudinitdisk
            cloudInitNoCloud:
              userData: |
                #cloud-config
                hostname: test
                disable_root: false
                ssh_authorized_keys: 
                  - ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPloV8xFkoybv0ztf1pObVM7Yt3JnP6iZH7xcUFIs9xT root@DESKTOP-K0IKF3F
```

### POST /service
```bash
## call TCP Service
curl -X POST http://localhost:3000/service -H "Accept: application/json" -H "Content-type: application/json" \
-d '{"id": "test", "protocol": "TCP", "ports": ["8080","25565"]}'
```
返値
```yml
---
apiVersion: v1
kind: Service
metadata:
  name: vps-test-svc-tcp
spec:
  type: LoadBalancer
  selector:
    kubevirt.io/domain: test
  ports:
    - protocol: TCP
      name: tcp8080
      port: "8080"
      targetPort: "8080"
    - protocol: TCP
      name: tcp25565
      port: "25565"
      targetPort: "25565"
```
<br><br><br>

```bash
## call UDP Service
curl -X POST http://localhost:3000/service -H "Accept: application/json" -H "Content-type: application/json" \
-d '{"id": "test", "protocol": "UDP", "ports": ["19132","30000"]}'
```
返値
```yml
---
apiVersion: v1
kind: Service
metadata:
  name: vps-test-svc-udp
spec:
  type: LoadBalancer
  selector:
    kubevirt.io/domain: test
  ports:
    - protocol: UDP
      name: udp19132
      port: "19132"
      targetPort: "19132"
    - protocol: UDP
      name: udp30000
      port: "30000"
      targetPort: "30000"
```
