# optical-waveguide

<!---Esses são exemplos. Veja https://shields.io para outras pessoas ou para personalizar este conjunto de escudos. Você pode querer incluir dependências, status do projeto e informações de licença aqui--->

> It's a simulator of optical waveguides on frequency domain. Focused on rectangular geometries for photonics integrated applications.

## Optical Waveguides

Waveguides are devices builded to conduct a wave through itself. Common examples are antennas, optical fibers, transmission lines and etc. They are important devices for semiconductor industries because the current frequencies which microchips operate are high enough for the nets to became electrical waveguides.

Specially for optical applications, optical waveguides are light conductors which have the property to guide the light for long distances. The most famous device of this family is the optical fiber who is largely used on communication industry. However, the interest for optical waveguide is growing on semiconductor industry because transfering from electronic integrated devices (electrical domain) to photonic integrated devices (light domain) decreases the energy consumption and heat generation, these are the main recent difficulties for the semiconductor industry scale up the performance of the new microchips.

Given this context, this project is a simulator for rectangular optical waveguides. The rectangular geometry is an important characteristic because it’s the form used to construct the current electronic integrated devices, increasing the compatibility with the current manufacturing methods.

# Getting Started

## 💻 Prerequisites

* Rust developement enviroment. Is recommended to install from `rustup`. 
More informations https://rust-lang.github.io/rustup/installation/index.html
* `python3 >= 3.9`
* `HDF5 >= 1.12.1`
* `pip >= 21.3.1`
* `maturin >= 0.11.5`
* `Qt5 >= 5.15`

## 🚀 Installation

To install the optical_waveguide simulator, follow up:

Linux:
```
git clone https://github.com/rennelou/optical-waveguide.git
cd optical-waveguide
python -m venv .env
source .env/bin/activate
maturin develop
```

## ☕ Running a python example

On `examples` folder has some examples of using, to run one of them follow up:

```
python examples/bidimensional.py
```

![alt text](https://github.com/rennelou/optical-waveguide/blob/master/examples/bidimensional.png)

## ☕ Enabling command line interface

The `optical-waveguide` simulator has a command line interface too, to enable it follow up:

```
cargo install --path .
PATH="$PATH:$HOME/.cargo/bin"
```

Now, we have a program called `optical_waveguide` on the $HOME/.cargo/bin folder and we inserted this folder on the `PATH` enviroment variable so the OS can find the simulator. If you want to make this change persistent, add `PATH="$PATH:$HOME/.cargo/bin"` on `~/.bashrc` file.

### Running a Cli Example

The simulator cli takes two arguments. The first is a filename for the simulation description on json format, the second is the result filename which will be generated.

Create the `my_bidimensional_simulation.json` file:

```json
{
  "x_axis": {
    "width": 40,
    "delta": 0.02  
  },
  "z_axis": {
    "width": 750,
    "delta": 0.5
  },
  "core": {
    "n0": 3.377,
    "n": 3.38,

    "width"
    "x": 20
  },
  "beam": {
    "k": 5.4636,
    "alpha": 0.0,
    "x": 20,
    "width": 4
  }
}
```

Then, run:

```
optical_waveguide my_bidimensional_simulation.json my_output_file.h5
```

The result will be a HDF5 file called `my_output_file.h5` which is formed from the hierarchy below:

```
/               root
/deltas         discretization steps of the simulation grid
/core           refractive index distribution of the device simulated
/eletric_field  electrical field distribution of the simulation
/intensity      light intensity distribution of the simulation
```


## 📫 To contribute
<!---Se o seu README for longo ou se você tiver algum processo ou etapas específicas que deseja que os contribuidores sigam, considere a criação de um arquivo CONTRIBUTING.md separado--->
To contribute with this project, follow up:

1. Fork this repository.
2. Create a branch: `git checkout -b <new_branch>`.
3. Make your changes and commit them: `git commit -m '<new_commit>'`
4. Push your branch to original: `git push origin optical-waveguide / rennelou`
5. Create a pull request.

For more information how to create a pull request [creating a pull request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request).

### To do

This project still on the beta fase.

The next features:
- [ ] parallelize the algorithm
- [ ] optimize dephts_cartesian_product function
- [ ] create a composition of cores
- [ ] create a composition of beams

unfortunately this project doesn't have a total unit test cover. Some cases which the unit needs to be tested:

- [ ] test the panic conditions
- [ ] gaussian beam with geometry larger than the simulation grid

## 🤝 Contributors

Thank you:

<table>
  <tr>
    <td align="center">
      <a href="#">
        <img src="https://avatars.githubusercontent.com/u/34797226?s=400&u=2505230870aaf025b6c3e6806981d4567b594592&v=4" width="100px;" alt="Foto do rennelou no GitHub"/><br>
        <sub>
          <b>Rennê Lou</b>
        </sub>
      </a>
    </td>
  </tr>
</table>


## 😄 Be a contributor<br>

Do you want to be a contributor? Click [here](CONTRIBUTING.md) and discover how to become one.

## 📝 License

This project is under the MIT license. See the [license](LICENSE.md) file for more details.

[⬆ Back to top](#optical-waveguide)<br>
