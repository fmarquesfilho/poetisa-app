use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::DefaultBackend,
	},
	sound::static_sound::StaticSoundData,
	clock::ClockSpeed,
};

pub fn play_audio() -> Result<(), Box<dyn std::error::Error>> {
    const TEMPO: f64 = 120.0;

    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let mut clock = manager.add_clock(ClockSpeed::TicksPerMinute(TEMPO))?;
    let sound_data_1 = StaticSoundData::from_file("src/audio/hihat.ogg")?
        .start_time(clock.time() + 2);
    manager.play(sound_data_1)?;
    let sound_data_2 = StaticSoundData::from_file("src/audio/snare.ogg")?
        .start_time(clock.time() + 4);
    manager.play(sound_data_2)?;
    clock.start();

    std::thread::sleep(std::time::Duration::from_millis(10000));

    Ok(())
}
