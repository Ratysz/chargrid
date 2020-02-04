// Run audio on a separate thread, to prevent issue on windows
// when also using winit to create windows.
// More info: https://github.com/RustAudio/cpal/pull/348

use crate::common;
use crate::Error;
use prototty_audio::{AudioPlayer, AudioProperties};
use rodio::source::Source;
use rodio::{Decoder, Sink};
use std::io::Cursor;
use std::sync::mpsc;
use std::thread;

pub struct NativeAudioPlayer {
    _audio_thread: thread::JoinHandle<()>,
    sender: mpsc::Sender<(NativeSound, AudioProperties)>,
}

impl NativeAudioPlayer {
    pub fn try_new_default_device() -> Result<Self, Error> {
        let (sender, receiver): (_, mpsc::Receiver<(NativeSound, AudioProperties)>) = mpsc::channel();
        let (init_sender, init_receiver) = mpsc::channel();
        let _audio_thread = std::thread::spawn(move || {
            if let Some(device) = common::output_device() {
                init_sender.send(Ok(())).unwrap();
                for (sound, properties) in receiver {
                    let sink = Sink::new(&device);
                    let source = Decoder::new(Cursor::new(sound.bytes))
                        .unwrap()
                        .amplify(properties.volume);
                    sink.append(source);
                    sink.detach();
                }
                log::info!("dedicated audio thread stopped");
            } else {
                init_sender.send(Err(Error::NoOutputDevice)).unwrap();
            }
        });
        let () = init_receiver
            .recv()
            .expect("unable to get status of dedicated audio thread")?;
        Ok(Self { _audio_thread, sender })
    }

    pub fn new_default_device() -> Self {
        Self::try_new_default_device().unwrap()
    }

    pub fn play(&self, sound: &NativeSound, properties: AudioProperties) {
        let result = self.sender.send((NativeSound { bytes: sound.bytes }, properties));
        match result {
            Ok(()) => (),
            Err(_) => log::error!("can't play audio because dedicated audio thread has stopped"),
        }
    }
}

#[derive(Clone)]
pub struct NativeSound {
    bytes: &'static [u8],
}

impl NativeSound {
    pub fn new(bytes: &'static [u8]) -> Self {
        Self { bytes }
    }
}

impl AudioPlayer for NativeAudioPlayer {
    type Sound = NativeSound;
    fn play(&self, sound: &Self::Sound, properties: AudioProperties) {
        NativeAudioPlayer::play(self, sound, properties)
    }
    fn load_sound(&self, bytes: &'static [u8]) -> Self::Sound {
        NativeSound::new(bytes)
    }
}