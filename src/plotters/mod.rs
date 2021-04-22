use super::waveguide::eletric_field_2d::EletricField2d;
use super::array::Array2d;
use super::waveguide::core_waveguide::Core;
use plotters::prelude::*;

pub fn plot_waveguide_2d(g: Array2d, es_2d: EletricField2d, r: impl Core, n0: f64, lines: usize) {
    let dx = g.get(0).d;
    let dz = g.get(1).d;

    let zsteps = g.get(1).steps;
    
    let root_drawing_area = BitMapBackend::new("waveguide.png", (1024, 768))
        .into_drawing_area();
    
    root_drawing_area.fill(&WHITE).unwrap();

    let root_drawing_area = root_drawing_area.titled("Image Title", ("sans-serif", 40)).unwrap();
    let (upper, lower) = root_drawing_area.split_vertically(512);

    let x_axis = (0.0f64..dx).step(dx/1000.0);

    let mut chart = ChartBuilder::on(&upper)
        .margin(10)
        .caption("Waveguide", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(x_axis.clone(), 0.0..dz)
        .unwrap();

        chart.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()
        .unwrap();

    let throttle = zsteps / lines; 

    for (i, line) in es_2d.get_points().enumerate() {
        if i % throttle == 0 {
            chart.draw_series(LineSeries::new(
                line.map(|p| (p.x, p.z + p.eletric_field)),
                &RED
            ))
            .unwrap();
        }
    }
    
    let mut cc = ChartBuilder::on(&lower)
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Refraction Index", ("sans-serif", 40))
        .build_cartesian_2d(x_axis.clone(), 0.0f64..4.0f64)
        .unwrap();

    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()
        .unwrap();

    cc.draw_series(LineSeries::new(
        x_axis.clone().values().map(|x| (x, r.get_n(x, 0.0, n0))),
        &BLUE,
    ))
    .unwrap()
    .label("Cosine")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    cc.configure_series_labels().border_style(&BLACK).draw().unwrap();

}