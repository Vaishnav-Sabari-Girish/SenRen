# SenRen (セんレン)

SenRen (線連) is a lightweight, command-driven LIDAR communication toolkit designed for robotics and embedded projects. Named from the Japanese words for "wire" (線, sen) and "connect/link" (連, ren), SenRen bridges your LIDAR hardware with modern data acquisition, monitoring, and visualization workflows—right from your terminal.

- Effortlessly connect, monitor, and plot LIDAR data with fast commands.
- Integrates with [ComChan](https://github.com/Vaishnav-Sabari-Girish/ComChan) for advanced serial monitoring and real-time terminal plotting.
- Export scan data for high-quality, publication-grade graphical plots using Python.

SenRen lets you focus on scanning and analysis, combining technical power with a streamlined user experience.
Simple. Connected. Fast.

## How to Install 

Clone the repo 

```bash
cargo run --release
```

## How to Use 

To scan for data use this 

```bash
senren scan
```

To plot data into a graph you can use this

```bash
senren plot
```

Currently `senren` only supports virtual ports. So to open up a virtual port and write data to it use this 

```bash
senren --virtual-mode
```
