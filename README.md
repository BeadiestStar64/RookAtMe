<h1 align="center">♜Rook At Me👀</h1>

<p align="center">
  異種GPUを束ね、限られた計算資源を工夫して使い切るための分散AIランタイム
</p>

## Rook At Meとは

**Rook At Me** は、性能・VRAM容量・世代の異なる複数のGPUをネットワーク越しに協調させ、
単一GPUでは収まらないモデルの学習・推論を可能にすることを目標とした実験的プロジェクトです。

高価なデータセンター向けGPUや専用クラスタを前提とせず、GeForceやノートPC向けGPUなど、
手元にある異種ハードウェアを組み合わせて利用することを重視しています。

例えば、次のような構成を1つの計算資源として扱うことを目指します。

```text
RTX 3080 Ti         12 GiB
RTX 4050 Laptop      6 GiB
--------------------------
合計VRAM            18 GiB
```

これは18 GiBの巨大な仮想GPUを作るという意味ではありません。

モデルや中間状態を複数GPUへ分割し、それぞれのGPUが担当する処理を協調して実行することで、
**物理的に分散したVRAMを学習・推論へ活用する**ことを目指します。

## 目標

Rook At Meでは、特に次の点を重視します。

- 異なるGPU間での非対称なモデル分割
- GPUごとのVRAM容量・演算性能を考慮した配置
- 低帯域ネットワークでも動作可能な分散学習
- Pipeline Parallelismを中心とした通信量の削減
- 通信とGPU計算のオーバーラップ
- Activation / Gradientの圧縮
- ノードの自動検出・性能測定・トポロジ認識
- 高価な専用機材に依存しない構成
- Linux / WSL2環境での運用

性能だけを追求するのではなく、

> 「遅くてもよいので、手元のGPUを組み合わせれば単体では扱えないモデルを動かせる」

ことを重要な目標としています。

## 想定アーキテクチャ

Rook At Meは、概ね次の構成を想定しています。

```text
                         ┌─────────────────────┐
                         │   rook-controller   │
                         │                     │
                         │ Topology / Planner  │
                         │ Scheduler / Monitor │
                         └──────────┬──────────┘
                                    │
                         Control / Metadata
                                    │
             ┌──────────────────────┴──────────────────────┐
             │                                             │
             ▼                                             ▼
    ┌─────────────────┐                           ┌─────────────────┐
    │   rook-agent    │                           │   rook-agent    │
    │                 │                           │                 │
    │ GPU Node A      │◀───── Tensor Traffic ────▶│ GPU Node B      │
    │ RTX 3080 Ti     │                           │ RTX 4050 Laptop │
    └─────────────────┘                           └─────────────────┘
```

ControllerはTensorデータを中継せず、主に次の処理を担当する予定です。

- ノード検出
- GPU情報収集
- 性能プロファイリング
- ネットワークトポロジの管理
- モデル分割計画
- ジョブ管理
- Heartbeat
- Checkpoint管理

実際のTensor通信はGPUノード間で直接行う構成を想定しています。

## 現在の開発状況

現在は **GPUノードのハードウェア情報を安全に取得するためのNVMLラッパー** を実装しています。

```text
crates/
└── rook-nvml/
    ├── csrc/
    │   ├── nvml_wrapper.c
    │   └── nvml_wrapper.h
    └── src/
        ├── lib.rs
        ├── safe_nvml.rs
        └── unsafe_nvml.rs
```

`rook-nvml`では、NVIDIA Management Library（NVML）のC APIを直接Rustへ大量に公開するのではなく、
薄いCラッパーを介して必要な機能だけを利用します。

```text
Safe Rust API
      │
      ▼
safe_nvml.rs
      │
      ▼
unsafe_nvml.rs
      │
      │ C ABI / FFI
      ▼
nvml_wrapper.c
      │
      ▼
NVIDIA NVML
      │
      ▼
NVIDIA Driver / GPU
```

この構成により、

- NVIDIA固有のC API
- Rustのunsafe FFI
- Rust側のSafe API

を明確に分離します。

## 開発ロードマップ

現時点では、次のような段階的な実装を想定しています。

### Phase 0 — Hardware Discovery

- NVMLの初期化
- GPU数の取得
- GPU名の取得
- GPU UUIDの取得
- VRAM容量の取得
- CUDA Compute Capabilityの取得
- ノード情報のControllerへの送信

### Phase 1 — Transport

- RustによるController / Agent通信
- GPUノード間のTensor転送
- TCPベースの通信
- Host Memory経由の転送
- Chunking / Buffering

### Phase 2 — Distributed Execution

- 2 GPU間でのモデル分割
- 2-stage Pipeline Parallelism
- Forward Pass
- Backward Pass
- Microbatch Pipeline

### Phase 3 — Optimization

- 通信と計算のオーバーラップ
- Pinned Memory
- Double / Triple Buffering
- Activation圧縮
- Gradient圧縮
- 低精度通信

### Phase 4 — Automatic Planner

- GPU性能の自動測定
- VRAM使用量の推定
- ネットワーク帯域・遅延の測定
- 異種GPU向けの非対称レイヤー配置
- 通信コストを含めた自動分割

## 技術スタック

現時点では、次の技術を利用・検討しています。

| 分野 | 技術 |
|---|---|
| Controller / Agent | Rust |
| FFI | Rust + C |
| GPU情報取得 | NVIDIA NVML |
| 非同期通信 | Tokio |
| 初期Transport | TCP |
| ML Runtime | JAX / Python（予定） |
| GPU Kernel | CUDA C++（必要になった場合） |
| CI | GitHub Actions |
| C Formatter | clang-format |
| C Static Analysis | clang-tidy |
| Rust Formatter | rustfmt |
| Rust Lint | Clippy |

## 設計方針

### GPUのVRAM容量だけで分割しない

例えば12 GiBと6 GiBのGPUが存在しても、単純に2:1でモデルを分割するとは限りません。

GPUごとに、

- 演算性能
- VRAM容量
- Memory Bandwidth
- PCIe帯域
- ネットワーク帯域
- 熱・電力制限

が異なるため、実測値を利用して配置を決定することを目指します。

### 低速ネットワークを前提にする

初期実験では、あえて一般家庭でも利用可能なEthernet環境を対象とします。

低帯域環境ではTensor Parallelismのように大量の同期通信を行う方式よりも、
Pipeline Parallelismを利用し、Stage境界を通過するデータ量を減らす方針を重視します。

### Control PlaneとData Planeを分離する

Controllerはスケジューリングや状態管理を担当し、
大量のTensorデータをController経由で転送しない設計を目指します。

```text
Controller
    │
    ├── Control Plane
    │
GPU A ◀──────────────▶ GPU B
          Data Plane
```

## CIについて

GitHub Actions上ではGPUを利用できないため、通常のPull Requestでは主に次の検査を行います。

- `cargo fmt`
- `cargo clippy`
- Cソースの`clang-format`
- Cソースの`clang-tidy`
- Cコンパイラの警告検査
- GPUを必要としないUnit Test

実GPUを必要とするNVML / CUDA / 分散実行テストについては、
将来的に専用のSelf-hosted Runnerなどへ分離する予定です。

## 対応環境

開発初期では、対象を次の環境へ限定する予定です。

- Linux x86_64
- Ubuntu 24.04 LTS
- Ubuntu 26.04 LTS
- WSL2 Ubuntu
- NVIDIA GPU

Native Windowsへの直接対応は、現時点では優先していません。

## プロジェクト名について

**Rook At Me** は、"Look at me" をもじった名前です。

`Rook`はチェスの駒である「ルーク（♜）」でもあり、
複数ノードを盤上の駒のように組み合わせて計算資源として扱うイメージも込めています。

## Status

> [!WARNING]
> Rook At Meは現在、初期開発段階の実験的プロジェクトです。
>
> API、プロトコル、ディレクトリ構成、アーキテクチャは今後大きく変更される可能性があります。

現時点では、本番環境での利用を想定していません。

