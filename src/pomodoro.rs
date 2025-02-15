use rodio::{buffer::SamplesBuffer, OutputStream, Sink};


use crate::utils::*;

pub fn pomodoro_loop() {
    clear_screen();
    println!("Enter duration of work session in minutes:");
    // let work_duration_secs: i32 = get_input().trim().parse::<i32>().unwrap() * 60;
    let work_duration_secs: i32 = 5;
    clear_screen();
    println!("Enter duration of break session in minutes:");
    let break_duration_secs: i32 = get_input().trim().parse::<i32>().unwrap() * 60;
    loop {
        work_loop(work_duration_secs);
        println!("Press any key to start break session..., or press q to exit");
        let input: String = get_input();
        if input.trim() == "q" {
            break;
        }
        break_loop(break_duration_secs);
        println!("Press any key to start work session..., or press q to exit");
        let input: String = get_input();
        if input.trim() == "q" {
            break;
        }
    }
}

fn play_sound(duration_secs: f32, frequency: f32, amplitude: f32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink: Sink = Sink::try_new(&stream_handle).unwrap();
    
    
    let sample_rate = 44100;
    let num_samples = (sample_rate as f32 * duration_secs) as usize;
    let samples: Vec<f32> = (0..num_samples)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            let envelope = (-20.0 * t).exp();
            (2.0 * std::f32::consts::PI * frequency * t).sin() * envelope * amplitude
        })
        .collect();

    // Create a single-channel audio source from the samples.
    let source = SamplesBuffer::new(1, sample_rate, samples);
    sink.append(source);
    sink.sleep_until_end();
}

fn play_working_sound() {
    play_sound(0.1, 800.0, 0.1);
}

fn play_break_sound() {
    play_sound(0.1, 400.0, 0.1);
}

fn work_loop(work_duration_secs: i32) {
    let mut work_time_count: i32 = 0;
    loop {
        clear_screen();
        work_time_count += 1;
        if work_time_count == work_duration_secs.try_into().unwrap() {
            println!("Work session over! Time for a break!");
            break;
        }
        play_working_sound();
        clear_screen();
        println!("Work session in progress... {} seconds left", work_duration_secs - work_time_count);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn break_loop(break_duration_secs: i32) {
    let mut break_time_count: i32 = 0;
    loop {
        clear_screen();
        break_time_count += 1;
        if break_time_count == break_duration_secs.try_into().unwrap() {
            println!("Break session over! Time to get back to work!");
            break;
        }
        play_break_sound();
        println!("Break session in progress... {} seconds left", break_duration_secs - break_time_count);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}