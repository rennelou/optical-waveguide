import h5py
import optical_waveguide
import numpy as np
import matplotlib.pyplot as plt

origin = 'lower'
lines = 50

output_filename = "tridimensional_result.h5"

x_axis = optical_waveguide.get_axis(40, 0.4)
y_axis = optical_waveguide.get_axis(40, 0.4)
z_axis = optical_waveguide.get_axis(200, 0.5)
core = optical_waveguide.get_core(3.377, 3.38, 8, 20, 20)
beam = optical_waveguide.get_beam(5.4636, 4, 20, 20)

simulation = optical_waveguide.get_simulation(core, beam, z_axis, x_axis, y_axis)
optical_waveguide.run("../release/optical_waveguide", simulation, output_filename)

with h5py.File(output_filename, "r") as f:
    
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
