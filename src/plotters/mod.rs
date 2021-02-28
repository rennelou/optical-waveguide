use super::waveguide::eletric_field_2d::EletricField2d;
use plotters::prelude::*;

pub fn plot_waveguide_2d(es_2d: EletricField2d) {
    let root_drawing_area = BitMapBackend::new("waveguide.png", (1024, 768))
        .into_drawing_area();
    
    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .caption("Waveguide", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0.0..es_2d.dx, 0.0..es_2d.dz)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for line in es_2d.get_points() {
        chart.draw_series(LineSeries::new(
            line.iter().map(|p| (p.x, p.z + p.eletric_field)),
            &RED
        )).unwrap();
    }
}