import h5py
import math
import numpy as np
import matplotlib
import matplotlib.pyplot as plt

matplotlib.use('Qt5Agg')

filename = "../tools/target/release/compared_gaussian_4.h5"

with h5py.File(filename, "r") as f:

    for key in f.keys():
        print("Key %s" % key)

    number_of_dots = 30

    reference_areas = f['areas_reference'][()]
    data_areas = f['areas_data'][()]
    diff_areas = f['areas_diff'][()]

    len = min(reference_areas.size, data_areas.size, diff_areas.size)

    throttle = int(len/ number_of_dots)
    reference = []
    data = []
    diff = []
    for i in range(len):
        if i % throttle == 0:
            reference.append(reference_areas[i])
            data.append(data_areas[i])
            diff.append(diff_areas[i])

    x = range(number_of_dots)

    fig1, ax = plt.subplots()
    ax.plot(x, reference, 'b--', label='beamlab')
    ax.plot(x, data, 'g^', label='resultados')
    ax.plot(x, diff, 'ro', label='distância')
    ax.set_ylabel('Área')
    ax.set_xlabel('Passo')
    ax.legend()

    plt.show()
