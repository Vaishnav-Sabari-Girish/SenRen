import sys
import numpy as np
import matplotlib.pyplot as plt

def main(csv_path):
    data = np.genfromtxt(csv_path, delimiter=',', skip_header=1)
    angles = np.deg2rad(data[:, 0])
    dists = data[:, 1]
    ax = plt.subplot(111, polar=True)
    ax.scatter(angles, dists, s=4, c='blue')
    ax.set_title("SenRen LIDAR Scan", va='bottom')
    ax.set_ylim(0, max(dists)*1.1)
    plt.tight_layout()
    plt.show()

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python plot_scan.py scan_data.csv")
    else:
        main(sys.argv[1])
