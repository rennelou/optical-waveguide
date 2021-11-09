import h5py
import optical_waveguide
import numpy as np
import matplotlib
matplotlib.use('Qt5Agg')
import matplotlib.pyplot as plt

lines = 50
origin = 'lower'

def main():
    output_filename = "bidimensional_result.h5"
    
    x_axis = optical_waveguide.get_axis(40, 0.02)
    z_axis = optical_waveguide.get_axis(750, 0.5)
    core = optical_waveguide.get_core(3.377, 3.38, 8, 20)
    beam = optical_waveguide.get_beam(5.4636, 0.0, 4, 20)
    
    simulation = optical_waveguide.get_simulation(core, beam, z_axis, x_axis)
    optical_waveguide.run(simulation, output_filename)

    with h5py.File(output_filename, "r") as f:

        [ydelta, xdelta] = f['deltas'][()]
        data = f['intensity'][()]

        xlen = data[0].size
        ylen = data.size / xlen

        x = np.arange(0.0, xlen*xdelta, xdelta)
        y = np.arange(0.0, ylen*ydelta, ydelta)

        X, Y = np.meshgrid(x, y)

        cs1 = plt.contourf(X, Y, data, origin=origin)
        cbar = plt.colorbar(cs1)  #barra lateral de intensidade
        cbar.ax.set_ylabel('intensity')

        core = f['core'][()]
        cs3 = plt.contour(X, Y, core, origin=origin)

        plt.show()

if __name__ == '__main__':
    main()