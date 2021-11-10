use super::fdmbpm::{WaveguideSimulation, beam, boundary_codition, cores, grid, slab::{self, Slab}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
struct WaveguideEntity {
    x_axis: Option<AxisEntity>,
    y_axis: Option<AxisEntity>,
    z_axis: AxisEntity,
    
    core: CoreEntity,

    beam: GaussianBeamEntity
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct AxisEntity {
    width: f64,
    delta: f64
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct CoreEntity {
    n0: f64,
    n: f64,

    width: f64,
    
    x: Option<f64>,
    y: Option<f64>
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct GaussianBeamEntity {
    k: f64,
    alpha: f64,
    width: f64,
    
    x: Option<f64>,
    y: Option<f64>
}

pub fn get_simulation(serialized: &str) -> Box<dyn WaveguideSimulation> {
    
    let w: WaveguideEntity = serde_json::from_str(serialized).unwrap();
        
    if let (&Some(x_axis), &Some(y_axis)) = (&w.x_axis, &w.y_axis) {
        Box::new(get3d_simulation(w, x_axis, y_axis))
    } else if let (&Some(x_axis), None) = (&w.x_axis, &w.y_axis) {
        Box::new(get2d_xsimulation(w, x_axis))
    } else if let (None, &Some(y_axis)) = (&w.x_axis, &w.y_axis) {
        Box::new(get2d_ysimulation(w, y_axis))
    } else {
        panic!("simulation needs at last one dimension parameters")
    }
}

fn get3d_simulation(entity: WaveguideEntity, x_axis: AxisEntity, y_axis: AxisEntity) -> Slab<3,2> {

    let dx = x_axis.width;
    let xdelta = x_axis.delta;
	let dy = y_axis.width;
	let ydelta = y_axis.delta;
	let dz = entity.z_axis.width;
    let zdelta = entity.z_axis.delta;

    let grid = grid::new3(dx, xdelta, dy, ydelta, dz, zdelta);

    if let (Some(x_core), Some(_y_core)) = (entity.core.x, entity.core.y) {
        let width = entity.core.width;
        let n0 = entity.core.n0;
        let n = entity.core.n;

        // Tem uma gambi no core 3d que centraliza o nucleo e por isso so considera um valor de posicao
        //let core_center = [y_core, x_core];
        let core = cores::rectilinear::new_3d(n, n0, x_core, width);
	
        let beam = entity.beam;
        if let (Some(x_beam), Some(y_beam)) = (beam.x, beam.y) {
            let k0 = beam.k;
            let alpha = beam.alpha;
            let w = beam.width;
            let beam_center = [y_beam, x_beam];
            
            let beam = beam::gaussian(beam_center, 1.0, w, k0, alpha);

            slab::new(grid.clone(), Box::new(core.clone()), beam, boundary_codition::transparent)
        
        } else {
            panic!("3D simulation must have all x and y positions for beam")
        }    
    } else {
        panic!("3D simulation must have all x and y positions for Core")
    }
}

fn get2d_xsimulation(entity: WaveguideEntity, x_axis: AxisEntity) -> Slab<2,1> {

    let dx = x_axis.width;
    let xdelta = x_axis.delta;
	let dz = entity.z_axis.width;
    let zdelta = entity.z_axis.delta;

    let grid = grid::new2(dx, xdelta, dz, zdelta);

    if let Some(x_core) = entity.core.x {
        let width = entity.core.width;
        let n0 = entity.core.n0;
        let n = entity.core.n;

        let core = cores::rectilinear::new_2d(n, n0, x_core, width);
	
        let beam = entity.beam;
        if let Some(x_beam) = beam.x {
            let k0 = beam.k;
            let w = beam.width;
            let beam_center = [x_beam];
            
            let beam = beam::gaussian(beam_center, 1.0, w, k0, 0.0);

            slab::new(grid.clone(), Box::new(core.clone()), beam, boundary_codition::transparent)
        
        } else {
            panic!("2D simulation in X must have x position for beam")
        }    
    } else {
        panic!("2D simulation for X must have x position for Core")
    }
}

fn get2d_ysimulation(entity: WaveguideEntity, y_axis: AxisEntity) -> Slab<2,1> {

    let dy = y_axis.width;
    let ydelta = y_axis.delta;
	let dz = entity.z_axis.width;
    let zdelta = entity.z_axis.delta;

    let grid = grid::new2(dy, ydelta, dz, zdelta);

    if let Some(y_core) = entity.core.y {
        let width = entity.core.width;
        let n0 = entity.core.n0;
        let n = entity.core.n;

        let core = cores::rectilinear::new_2d(n, n0, y_core, width);
	
        let beam = entity.beam;
        if let Some(y_beam) = beam.y {
            let k0 = beam.k;
            let w = beam.width;
            let beam_center = [y_beam];
            
            let beam = beam::gaussian(beam_center, 1.0, w, k0, 0.0);

            slab::new(grid.clone(), Box::new(core.clone()), beam, boundary_codition::transparent)
        
        } else {
            panic!("2D simulation in Y must have y position beam")
        }   

    } else {
        panic!("2D simulation for Y must have y position for Core")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_axis() {
        let data = r#"
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
                
                "width": 8,

                "x": 20
            },
            "beam": {
                "k": 5.4636,
                "alpha": 0.0,
                "x": 20,
                "width": 4
            }
        }"#;

        get_simulation(data);
    }

    #[test]
    fn two_axis() {
        let data = r#"
        {
            "n0": 3.377,
            "n": 3.38,
            "x_axis": {
                "width": 40,
                "delta": 0.02  
            },
            "y_axis": {
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
                
                "width": 8,

                "x": 20,
                "y": 20
            },
            "beam": {
                "k": 5.4636,
                "alpha": 0.0,
                "x": 20,
                "y": 20,
                "width": 4
            }
        }"#;

        get_simulation(data);
    }

    #[test]
    fn two_axis_onedimensional_beam() {
        let data = r#"
        {
            "n0": 3.377,
            "n": 3.38,
            "x_axis": {
                "width": 40,
                "delta": 0.02  
            },
            "y_axis": {
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
                
                "width": 8,

                "x": 20,
                "y": 20
            },
            "beam": {
                "k": 5.4636,
                "alpha": 0.0,
                "x": 20,
                "width": 4
            }
        }"#;

        let result = std::panic::catch_unwind(|| {
            get_simulation(data)
        });
        assert!(result.is_err())
    }

    #[test]
    fn x_axis_and_y_core() {
        let data = r#"
        {
            "n0": 3.377,
            "n": 3.38,
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
                
                "width": 8,

                "y": 20
            },
            "beam": {
                "k": 5.4636,
                "alpha": 0.0,
                "x": 20,
                "width": 4
            }
        }"#;

        let result = std::panic::catch_unwind(|| {
            get_simulation(data)
        });
        assert!(result.is_err())
    }

    #[test]
    fn y_axis_and_x_beam() {
        let data = r#"
        {
            "n0": 3.377,
            "n": 3.38,
            "y_axis": {
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
                
                "width": 8,

                "x": 20,
                "y": 20
            },
            "beam": {
                "k": 5.4636,
                "alpha": 0.0,
                "x": 20,
                "width": 4
            }
        }"#;

        let result = std::panic::catch_unwind(|| {
            get_simulation(data)
        });
        assert!(result.is_err())
    }
}