import h5py
import math
import numpy as np
import matplotlib
matplotlib.use('Qt5Agg')
import matplotlib.pyplot as plt

filename = "../tools/target/release/compared_gaussian_8.h5"

with h5py.File(filename, "r") as f:

    for key in f.keys():
        print("Key %s" % key)
    
    reference_areas = f['areas_reference'][()]
    reference_areas_len = reference_areas.size
    reference_areas_x = np.arange(0.0, reference_areas_len, 1.0)

    data_areas = f['areas_data'][()]
    data_areas_len = data_areas.size
    data_areas_x = np.arange(0.0, data_areas_len, 1.0)

    diff_areas = f['areas_diff'][()]
    diff_areas_len = diff_areas.size
    diff_areas_x = np.arange(0.0, diff_areas_len, 1.0)

    fig1, cs2 = plt.subplots()
    cs2.plot(reference_areas_x, reference_areas)
    cs2.plot(data_areas_x, data_areas)
    cs2.plot(diff_areas_x, diff_areas)
    cs2.set_ylabel('error')
    
    plt.show()
