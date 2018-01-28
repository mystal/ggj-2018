pub use ears::{Sound, AudioController, Music};

pub struct Sounds {
    //pub intro_music: Music,
    pub background_music: Music,
    pub won_level: Sound,
    pub lost_level: Sound,
    pub got_mail: Sound,
    pub bark: Sound,
}

impl Sounds {
    pub fn new() -> Self {
        let mut background_music = Music::new("assets/sounds/background_music_loop.ogg").expect("Error on loading background_music_loop.");
        background_music.set_looping(true);
        let won_level = Sound::new("assets/sounds/won_level.wav").expect("Error on loading won_level.");
        let lost_level = Sound::new("assets/sounds/lost_level.wav").expect("Error loading lost_level.");
        let got_mail = Sound::new("assets/sounds/got_mail.wav").expect("Error on loading got_mail.");
        let bark = Sound::new("assets/sounds/bark.wav").expect("Error loading bark.");
        Sounds {
            background_music,
            won_level,
            lost_level,
            got_mail,
            bark,
        }
    }

    pub fn fox_move() -> Sound {
        Sound::new("assets/sounds/fox_move.wav").expect("Error on loading fox_move.")
    }
}
