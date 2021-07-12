import h5py
import numpy as np
import matplotlib.pyplot as plt

origin = 'lower'

filename = "../fdmbpm/slab3d.h5"
lines = 50
with h5py.File(filename, "r") as f:

    for key in f.keys():
        print("Key %s" % key)
    
    [zdelta, ydelta, xdelta] = f['deltas'][()]
    data = f['intensity'][()]
    core = f['core'][()]

    xdepht = data[0][0].size
    ydepht = data[0].size / xdepht
    zdepht = data.size / (ydepht * xdepht)

    y = np.arange(0., ydepht * ydelta, ydelta)
    x = np.arange(0., xdepht * xdelta, xdelta)
    X, Y = np.meshgrid(x, y)

    zstep = zdepht / 4
    fig1, axs = plt.subplots(1, 4, constrained_layout=True)
    for i in range(4):
        index = int(i * zstep)
        Z = data[index]
        ax = axs[i]
        cs = ax.contourf(X, Y, Z, 10, origin=origin)
        cs1 =  ax.contour(cs, levels=cs.levels[::2], origin=origin)
        
        C = core[index]
        cs3 = ax.contour(X, Y, C, origin=origin)
        
        if i == 3:
            cbar = fig1.colorbar(cs, ax=ax)  #barra lateral de intensidade
            cbar.ax.set_ylabel('intensity')
            cbar.add_lines(cs1)
        
    plt.show()
