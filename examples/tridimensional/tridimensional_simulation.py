import h5py
import subprocess
import numpy as np
import matplotlib.pyplot as plt

origin = 'lower'
lines = 50

output_filename = "tridimensional_result.h5"
input_filename = "tridimensional_simulation.json"
simulation_param = '{"x_axis": {"width": 40,"delta": 0.4},"y_axis": {"width": 40,"delta": 0.4},"z_axis": {"width": 200,"delta": 0.5},"core": {"n0": 3.377, "n": 3.38, "width": 8,"x": 20, "y": 20},"beam": {"k": 5.4636,"x": 20,"y": 20,"width": 4} }'

input_file = open(input_filename, "w")
input_file.write(simulation_param)
input_file.close()

subprocess.run(["../optical_waveguide", input_filename, output_filename])

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
