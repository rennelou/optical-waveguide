use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WaveguideEntity {
    n0: f64,
    n: f64,
    
    x_axis: Option<AxisEntity>,
    y_axis: Option<AxisEntity>,
    z_axis: AxisEntity,
    
    beams: Vec<GaussianBeamEntity>
}

#[derive(Serialize, Deserialize)]
struct AxisEntity {
    width: f64,
    delta: f64
}

#[derive(Serialize, Deserialize)]
struct GaussianBeamEntity {
    k: f64,
    width: f64,
    
    x: Option<f64>,
    y: Option<f64>
}

fn validate_waveguide_entity(entity: WaveguideEntity) -> bool {
    if let (Some(_), Some(_)) = (&entity.x_axis, &entity.y_axis) {
    
        entity.beams.into_iter().all( |beam| beam.x.is_some() && beam.y.is_some() )
    
    } else if let (Some(_), None) = (&entity.x_axis, &entity.y_axis) {
    
        entity.beams.into_iter().all( |beam| beam.x.is_some() && beam.y.is_none() )

    }  else if let (None, Some(_)) = (&entity.x_axis, &entity.y_axis) {
    
        entity.beams.into_iter().all( |beam| beam.x.is_none() && beam.y.is_some() )
    
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
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
            "beams": [
                {
                    "k": 5.4636,
                    "x": 20,
                    "width": 8
                }
            ]
        }"#;

        let _: WaveguideEntity = serde_json::from_str(data).unwrap();
    }
}