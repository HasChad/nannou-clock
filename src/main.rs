#![windows_subsystem = "windows"]

use chrono::prelude::*;
use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    let full_radian = deg_to_rad(360.0);
    let second = Local::now().second();
    let minute = Local::now().minute();
    let hour = if Local::now().hour() > 12 {
        Local::now().hour() - 12
    } else {
        Local::now().hour()
    };
    let sec_degree = (second * 6) as f32;
    let min_degree = (minute * 6) as f32;
    let hour_degree = (hour * 30) as f32;

    let start = pt2(0.0, 0.0);

    draw.background().color(BLACK);

    //Main Dots
    draw.ellipse().w_h(10.0, 10.0).x_y(0.0, 250.0);
    draw.ellipse().w_h(10.0, 10.0).x_y(0.0, -250.0);
    draw.ellipse().w_h(10.0, 10.0).x_y(250.0, 0.0);
    draw.ellipse().w_h(10.0, 10.0).x_y(-250.0, 0.0);

    //Side Dots
    draw.ellipse().w_h(8.0, 8.0).x_y(
        deg_to_rad(30.0).cos() * 250.0,
        deg_to_rad(30.0).sin() * 250.0,
    );

    draw.ellipse().w_h(8.0, 8.0).x_y(
        deg_to_rad(60.0).cos() * 250.0,
        deg_to_rad(60.0).sin() * 250.0,
    );

    draw.ellipse().w_h(8.0, 8.0).x_y(
        deg_to_rad(120.0).cos() * 250.0,
        deg_to_rad(120.0).sin() * 250.0,
    );

    draw.ellipse().w_h(8.0, 8.0).x_y(
        deg_to_rad(150.0).cos() * 250.0,
        deg_to_rad(150.0).sin() * 250.0,
    );

    draw.ellipse().w_h(8.0, 8.0).x_y(
        deg_to_rad(210.0).cos() * 250.0,
        deg_to_rad(210.0).sin() * 250.0,
    );

    draw.ellipse().w_h(8.0, 8.0).x_y(
        deg_to_rad(240.0).cos() * 250.0,
        deg_to_rad(240.0).sin() * 250.0,
    );

    draw.ellipse().w_h(8.0, 8.0).x_y(
        deg_to_rad(270.0).cos() * 250.0,
        deg_to_rad(270.0).sin() * 250.0,
    );

    draw.ellipse().w_h(8.0, 8.0).x_y(
        deg_to_rad(300.0).cos() * 250.0,
        deg_to_rad(300.0).sin() * 250.0,
    );

    draw.ellipse().w_h(8.0, 8.0).x_y(
        deg_to_rad(330.0).cos() * 250.0,
        deg_to_rad(330.0).sin() * 250.0,
    );

    //minutes line
    draw.line()
        .start(start)
        .end(pt2(0.0, 200.0))
        .weight(10.0)
        .color(BLUE)
        .rotate(full_radian - deg_to_rad(min_degree));

    //hourse line
    draw.line()
        .start(start)
        .end(pt2(0.0, 150.0))
        .weight(10.0)
        .color(YELLOW)
        .rotate(full_radian - deg_to_rad(hour_degree));

    draw.ellipse().w_h(10.0, 10.0).color(YELLOW);

    //seconds line
    draw.line()
        .start(start)
        .end(pt2(0.0, 200.0))
        .weight(5.0)
        .color(RED)
        .rotate(full_radian - deg_to_rad(sec_degree));

    draw.ellipse().w_h(5.0, 5.0).color(RED);

    // * Seconds Arc
    let second_radius = 250.0;
    let points = (0..=second * 6).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * second_radius;
        let y = radian.cos() * second_radius;
        (pt2(x, y), RED)
    });
    draw.polyline().weight(10.0).points_colored(points);

    draw.ellipse()
        .w_h(10.0, 10.0)
        .x_y(
            deg_to_rad(sec_degree).sin() * second_radius,
            deg_to_rad(sec_degree).cos() * second_radius,
        )
        .color(RED);
    draw.ellipse().w_h(10.0, 10.0).x_y(0.0, 250.0).color(RED);

    // * Minutes Arc
    let minute_radius = 230.0;
    let points = (0..=minute * 6).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * minute_radius;
        let y = radian.cos() * minute_radius;
        (pt2(x, y), BLUE)
    });
    draw.polyline().weight(10.0).points_colored(points);

    draw.ellipse()
        .w_h(10.0, 10.0)
        .x_y(
            deg_to_rad(min_degree).sin() * minute_radius,
            deg_to_rad(min_degree).cos() * minute_radius,
        )
        .color(BLUE);
    draw.ellipse().w_h(10.0, 10.0).x_y(0.0, 230.0).color(BLUE);

    // * Hours Arc
    let hours_radius = 210.0;

    let points = (0..=hour * 30).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * hours_radius;
        let y = radian.cos() * hours_radius;
        (pt2(x, y), YELLOW)
    });
    draw.polyline().weight(10.0).points_colored(points);

    draw.ellipse()
        .w_h(10.0, 10.0)
        .x_y(
            deg_to_rad(hour_degree).sin() * hours_radius,
            deg_to_rad(hour_degree).cos() * hours_radius,
        )
        .color(YELLOW);
    draw.ellipse().w_h(10.0, 10.0).x_y(0.0, 210.0).color(YELLOW);

    draw.to_frame(app, &frame).unwrap();
}
