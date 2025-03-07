use charming::{
    component::{Axis, Title},
    element::{ItemStyle, Padding, Symbol, TextStyle},
    series::Scatter,
    theme::Theme,
    Chart, ImageFormat, ImageRenderer,
};

fn main() {
    let malthus_data = basic_sim();
    let verhulst_data = act_data();
    let bentham_data = bentham_sim();

    plot(
        malthus_data,
        bentham_data,
        verhulst_data,
        "population_growth.png".to_string(),
    );
}

fn basic_sim() -> Vec<Vec<f64>> {
    let c = 0.0202977;
    let mut n = 3695f64;
    let dt = 1.0;

    let mut sim_data = Vec::new();
    sim_data.push(vec![1970.0, n]);
    for t in 1971..2046 {
        n = (1.0 + c) * n * dt;
        sim_data.push(vec![t as f64, n]);
    }

    return sim_data;
}

fn bentham_sim() -> Vec<Vec<f64>> {
    let a = 0.03051384;
    let e = 0.000002764856;
    let mut n = 3695f64;

    let dt = 1.0;

    let mut sim_data = Vec::new();
    sim_data.push(vec![1970.0, n]);
    for t in 1971..2046 {
        n = (1.0 + a * dt - e * n * dt) * n;
        sim_data.push(vec![t as f64, n]);
    }

    return sim_data;
}

fn act_data() -> Vec<Vec<f64>> {
    let mut act_data = Vec::new();
    let mut reader = csv::Reader::from_path("src/world_pops.csv").expect("Failed to read file");
    for record in reader.records() {
        let record = record.expect("Failed to read record");
        let year: f64 = record[0].parse().expect("Failed to parse year");
        let population: f64 = record[1].parse().expect("Failed to parse population");
        act_data.push(vec![year, population]);
    }

    return act_data;
}

fn plot(
    sim_data_1: Vec<Vec<f64>>,
    sim_data_2: Vec<Vec<f64>>,
    act_data: Vec<Vec<f64>>,
    out_path: String,
) {
    let chart = Chart::new()
        .title(
            Title::new()
                .text("Population Growth")
                .text_style(TextStyle::new().font_size(32).font_weight("bold"))
                .padding(Padding::Double(10.0, 100.0)),
        )
        .x_axis(Axis::new().min(1945).max(2055))
        .y_axis(Axis::new().log_base(10.0))
        .series(
            Scatter::new()
                .symbol_size(8)
                .symbol(Symbol::Circle)
                .item_style(ItemStyle::new().opacity(0.5))
                .name("Malthus")
                .data(sim_data_1),
        )
        .series(
            Scatter::new()
                .symbol_size(8)
                .symbol(Symbol::Rect)
                .item_style(ItemStyle::new().opacity(0.5))
                .name("Verhulst")
                .data(sim_data_2),
        )
        .series(
            Scatter::new()
                .symbol_size(8)
                .symbol(Symbol::Triangle)
                .item_style(ItemStyle::new().opacity(0.5))
                .name("Actual Data")
                .data(act_data),
        );

    ImageRenderer::new(1080, 720)
        .theme(Theme::PurplePassion)
        .save_format(ImageFormat::Png, &chart, out_path)
        .expect("Failed to save image");
}
