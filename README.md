# afwaveplot

Helper functions to generate audio waveforms for display purposes.<br>
Part of the [afplay](https://github.com/emuell/afplay) crates. 

### Examples

Write mixed-down (mono) waveform from an audio file as SVG file using [hound](https://github.com/ruuda/hound) as wave file reader.

```rust
use svg::{node::element::{path::Data, Path}, Document};
use afwaveplot::mixed_down::waveform_from_buffer;

// resolution/viewBox of the resulting SVG
const WIDTH: usize = 1024;
const HEIGHT: usize = 256;
const STROKE_WIDTH: usize = 1;

// get specs and an interleaved, normalized buffer from some wavefile
let mut wave_reader = hound::WavReader::open("SOME_FILE.wav")?;
let specs = wave_reader.spec();
let buffer: Vec<f32> = wave_reader
    .samples::<i32>()
    .map(|v| v.unwrap() as f32 / (1 << (specs.bits_per_sample - 1)) as f32)
    .collect();

// generate mixed-down, mono waveform data with WIDTH as resolution
let waveform_data = waveform_from_buffer(
    &buffer,
    specs.channels as usize,
    specs.sample_rate,
    WIDTH);

// fit waveform points into our viewBox
let num_points = waveform_data.len();
let width = WIDTH as f32;
let height = HEIGHT as f32;
let scale_x = move |v| v as f32 * width / num_points as f32;
let scale_y = move |v| (v + 1.0) * height / 2.0;

// create path from waveform points
let mut data = Data::new();
data = data.move_to((scale_x(0), scale_y(waveform_data[0].min)));
for (index, point) in waveform_data.iter().enumerate() {
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

// write the document to a file
svg::save("SOME_WAVEFORM.svg", &document)?;
```

## License

afwaveplot is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
