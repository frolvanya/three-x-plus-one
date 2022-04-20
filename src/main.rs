use plotters::prelude::*;

fn calculate_steps(mut n: i64) -> i64 {
    let mut steps = 0;

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }

        steps += 1;
    }

    steps
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("result.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("3x + 1", ("arial", 50).into_font())
        .margin(50)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(1..1000000, 0..1000)
        .unwrap();

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (1i64..=1000000i64).map(|x| (x as i32, calculate_steps(x) as i32)),
            &RED,
        ))
        .unwrap();

    Ok(())
}
