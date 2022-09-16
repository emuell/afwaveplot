use svg::{
    node::element::{path::Data, Path},
    Document,
};

use afwaveplot::mixed_down::waveform_from_buffer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // resolution of the resulting SVG
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 256;
    const STROKE_WIDTH: usize = 1;

    // generate mono waveforms of files in our assets folder
    let asset_test_files = vec![
        ("YuaiLoop", "wav"), // very long & stereo -> downsample
        ("AKWF_saw", "wav"), // very short & mono -> upsample
    ];

    for (file_name, extension) in asset_test_files {
        let mut wave_reader = hound::WavReader::open(format!("assets/{file_name}.{extension}"))?;
        let specs = wave_reader.spec();
        let buffer: Vec<f32> = wave_reader
            .samples::<i32>()
            .map(|v| v.unwrap() as f32 / (1 << (specs.bits_per_sample - 1)) as f32)
            .collect();

        // generate mono waveform data from file
        let channel_data =
            waveform_from_buffer(&buffer, specs.channels as usize, specs.sample_rate, WIDTH);

        // fit points into our viewBox
        let num_points = channel_data.len();
        let width = WIDTH as f32;
        let height = HEIGHT as f32;

        let scale_x = move |v| v as f32 * width / num_points as f32;
        let scale_y = move |v| (v + 1.0) * height / 2.0;

        // create path from waveform points
        let mut data = Data::new();
        data = data.move_to((scale_x(0), scale_y(channel_data[0].min)));
        for (index, point) in channel_data.iter().enumerate() {
            let x = scale_x(index);
            data = data
                .line_to((x, scale_y(point.min)))
                .line_to((x, scale_y(point.max)));
        }
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", STROKE_WIDTH)
            .set("d", data);

        // create svg document and add the path
        let mut document = Document::new().set("viewBox", (0, 0, WIDTH, HEIGHT));
        document = document.add(path);

        // write svg document next to audio file
        svg::save(format!("assets/{file_name}.svg").as_str(), &document)
            .map_err(|err| err.to_string())?;
    }
    Ok(())
}
