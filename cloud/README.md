# SCMessenger Cloud Infrastructure

This directory contains the Decoupled Orchestrator-Worker Architecture files for SCMessenger.

## Directory Layout
- `terraform/`: Infrastructure-as-Code to provision the persistent Orchestrator VM and templates for Ephemeral Spot workers on Google Cloud Platform (GCP).
- `orchestrator/`: FastAPI server + Telegram bot gateway that manages worker lifecycles, receives heartbeats, handles preemption, and relays status back to developers.
- `worker/`: Bootstrap and lifecycle management scripts for on-demand Spot compilation/testing, including Android emulation and WASM browser testing.
- `mesh/`: Proximity network emulation configuration using Docker Compose + WireGuard + network condition profiles (BLE, Wi-Fi Direct, DTN).
- `scripts/`: Initialization and automation scripts for provisioning and cleanups.

---

## 🏗️ 1. Infrastructure Setup (Terraform)

### Prerequisites
1. Install [Terraform](https://developer.hashicorp.com/terraform/downloads) and [gcloud CLI](https://cloud.google.com/sdk/docs/install).
2. Configure GCP credentials:
   ```bash
   gcloud auth application-default login
   ```

### Deployment
1. Navigate to the terraform directory:
   ```bash
   cd cloud/terraform
   ```
2. Initialize and deploy:
   ```bash
   terraform init
   ```
3. Set your variable overrides in a `terraform.tfvars` file:
   ```hcl
   project_id          = "your-gcp-project-id"
   telegram_bot_token  = "your-bot-token"
   openrouter_api_key  = "your-openrouter-key"
   github_repo         = "your-username/SCMessenger"
   ```
4. Deploy the infrastructure:
   ```bash
   terraform apply
   ```
This will provision:
- A persistent `e2-micro` Orchestrator instance.
- Firewall rules allowing webhook incoming connections (port 8080) and secure control channels.
- Service accounts with minimum required permissions.

---

## 🤖 2. Persistent Orchestrator (FastAPI + Telegram)

The orchestrator runs 24/7 on the free-tier `e2-micro` instance. It listens for Telegram bot commands and processes callbacks from active workers and GitHub Actions.

### Telegram Interface
Control your entire development cycle using a simple chat interface. Supported commands:
- `/sprint <prompt>`: Spin up a worker to run tests, write code, or execute complex tasks.
- `/build <platform> [branch]`: Build the SCMessenger workspace target (`android`, `ios`, `wasm`, `linux`, `windows`, `macos`).
- `/status`: Report active sprints, health/heartbeat of current tasks, and active workers.
- `/logs [sprint_id]`: Fetch and tail the last 50 lines of logs from the active worker VM.
- `/kill <sprint_id>`: Instantly destroy a running worker Spot instance.
- `/cost`: Display rough cost estimation based on active instance durations.

---

## ⚡ 3. Ephemeral Workers & Spot Interruption Recovery

When a sprint or build is triggered, the Orchestrator provisions high-resource Spot instances (e.g. `e2-standard-8` for compilation and testing, or `n2-standard-8` for Android emulation with nested virtualization).

### Preemption Recovery Daemon
Spot VMs can be preempted at any time. A lightweight daemon running on the worker polls Google's metadata server every 5 seconds.
Upon detecting preemption:
1. It immediately cancels compiling or testing processes.
2. It commits and pushes current changes to the active sprint branch.
3. It sends an emergency POST request to the `/api/preempted` orchestrator callback.
4. The Orchestrator picks a new region/zone, provisions a new VM, and resumes execution seamlessly.

---

## 🧪 4. Emulated Proximity Networks (Mesh Testing)

To simulate BLE and Wi-Fi Direct mesh routing and Store-and-Carry DTN (Delay Tolerant Network) dynamics:
1. Run the mesh test bed locally or inside a worker VM:
   ```bash
   cd cloud/mesh
   docker-compose -f docker-compose.mesh-test.yml up --build
   ```
2. Run network profile scripts (`cloud/mesh/profiles/`) to apply traffic control (`tc netem`) policies on node interfaces:
   - BLE Nearby (`ble_nearby.sh`): Low throughput, minor delay.
   - BLE Edge (`ble_edge.sh`): High packet loss, high delay.
   - Wi-Fi Direct (`wifi_direct.sh`): High throughput, ultra-low delay.
   - DTN Store-and-Carry (`dtn_carry.sh`): Cyclic connectivity toggling between connected and 100% packet loss.
3. Test Store-and-Carry behavior:
   ```bash
   ./test_dtn_store_and_carry.sh
   ```
