## Pretrained [ r3d18]

# 🎥 Video Action Recognition with PyTorch (R3D-18)

A structured, versioned learning & experimentation repo for **video action recognition** using PyTorch. This document tracks datasets, preprocessing, model choices, pitfalls, and extensions as the project evolves.

---

## 📌 Project Goals

* Build a **working end‑to‑end video classification pipeline**
* Understand **video data loading**, sampling, and transforms
* Train and fine‑tune **pretrained R3D‑18** (3D ResNet)
* Create a base structure that can be extended to other video models (SlowFast, X3D, ViViT, etc.)

---

## 🧾 Versioning

| Version | Description                                         |
| ------- | --------------------------------------------------- |
| `v0.1`  | Dataset setup, video folder structure, basic loader |
| `v0.2`  | Clip sampling + transforms + DataLoader             |
| `v0.3`  | Pretrained R3D‑18 inference                         |
| `v0.4`  | Training loop + metrics                             |
| `v1.0`  | Clean pipeline + reproducibility                    |

> 📍 Updating this table whenever a **conceptual milestone** is reached.

---

## 📂 Dataset

### 🔗 UCF‑101 Action Recognition Dataset

* Source: Kaggle
* Link: [https://www.kaggle.com/datasets/matthewjansen/ucf101-action-recognition](https://www.kaggle.com/datasets/matthewjansen/ucf101-action-recognition)

### Expected Directory Structure

```text
UCF101/
 ├── ApplyEyeMakeup/
 │    ├── v_ApplyEyeMakeup_g01_c01.mp4
 ├── Archery/
 ├── BabyCrawling/
```

> Each **folder name = class label**

---

## 🎞️ Video Data Preparation

### 📘 Reference Guide

* *Preparing Video Data for Training (PyTorch)*
  [https://medium.com/@naneettyagi2004/preparing-video-data-for-training-a-pytorch-guide-fc644ee9e64c](https://medium.com/@naneettyagi2004/preparing-video-data-for-training-a-pytorch-guide-fc644ee9e64c)

### Key Decisions

| Choice                 | Reason                         |
| ---------------------- | ------------------------------ |
| Folder‑based dataset   | Mirrors `ImageFolder` logic    |
| Fixed number of frames | Required for batching          |
| CenterCrop + Resize    | Matches Kinetics preprocessing |

---

## 🧠 Model

### Pretrained Model

* **R3D‑18** (3D ResNet‑18)
* Pretrained on **Kinetics‑400**

```python
from torchvision.models.video import r3d_18
model = r3d_18(pretrained=True)
```

### Expected Input Shape

```text
(B, 3, T, H, W)
```

Where:

* `T` = number of frames per clip (typically 16)
* `H × W` = spatial resolution

---

## 🔁 Data Pipeline Overview

```text
Video → Decode → Frame Sampling → Crop/Resize → Normalize → Model
```

### Typical Shapes

| Stage     | Shape           |
| --------- | --------------- |
| Raw video | (T, H, W, C)    |
| Tensor    | (C, T, H, W)    |
| Batch     | (B, C, T, H, W) |

---

## Common Pitfalls (Logged)

* ❌ `Resize(128, 171)` → wrong argument order
* ❌ Forgetting PyAV (`pip install av`)
* ⚠️ torchvision video IO deprecation warnings
* ❌ Treating filenames as labels instead of folder names

---

## Add‑Ons (Planned Extensions)

### Dataset

* [ ] Multiple clips per video
* [ ] Random temporal sampling (train) vs fixed (val)
* [ ] Frame‑based dataset (pre‑decoded)

### Models

* [ ] Fine‑tuning last layer only
* [ ] SlowFast
* [ ] X3D
* [ ] ViViT / TimeSformer

### Training

* [ ] Mixed precision (AMP)
* [ ] Class imbalance handling
* [ ] Top‑1 / Top‑5 accuracy

---

## Templates

### Experiment Log Template

```text
Experiment ID:
Dataset split:
Frames per clip:
Resolution:
Model:
Pretrained:
Optimizer:
LR:
Epochs:
Results:
Notes:
```

---

### Dataset Checklist

```text
[ ] Folder structure verified
[ ] Class count correct
[ ] Frame sampling working
[ ] Transforms applied per frame
[ ] Batch shape verified
```

---

### Debug Checklist

```text
[ ] Single batch forward pass works
[ ] Loss decreases
[ ] Labels in range
[ ] No NaNs
```

---

## Future Notes

* torchvision video decoding will be deprecated → migrate to **TorchCodec**
* Pre‑extracting frames is faster and more reproducible -> my method only for inference only ( slower for batch processing )
* Most research pipelines avoid on‑the‑fly decoding

---

## Learning Philosophy

> *"Understand the data first. The model is just the last consumer."*

This repo prioritizes:

* Shape awareness
* Explicit data flow
* Minimal magic

---

## Status

 Actively evolving — this README doubles as a **project log for me**.
