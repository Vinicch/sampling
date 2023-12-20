use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let host = cpal::default_host();
    let output = host
        .default_output_device()
        .expect("no output device available");

    let config = output.default_output_config().unwrap();
    let channels = config.channels() as usize;
    let sample_rate = config.sample_rate().0 as f32;

    let mut sample_clock = 0f32;
    let stream = output
        .build_output_stream(
            &config.config(),
            move |data: &mut [f32], _| {
                for frame in data.chunks_mut(channels) {
                    sample_clock = (sample_clock + 1.0) % sample_rate;
                    let value =
                        (sample_clock * 110.0 * 2.0 * std::f32::consts::PI / sample_rate).sin();

                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            move |err| eprintln!("{err}"),
            None,
        )
        .unwrap();

    stream.play().unwrap();

    std::thread::park();
}
