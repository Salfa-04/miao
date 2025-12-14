//! Canon in D Major Melody for Buzzer

const NOTE_G5: u32 = 784;
const NOTE_E5: u32 = 659;
const NOTE_F5: u32 = 698;
const NOTE_G4: u32 = 392;
const NOTE_A4: u32 = 440;
const NOTE_B4: u32 = 494;
const NOTE_C5: u32 = 523;
const NOTE_D5: u32 = 587;
const NOTE_E4: u32 = 330;
const NOTE_F4: u32 = 349;
const NOTE_D4: u32 = 294;
const NOTE_C4: u32 = 262;

pub const TUNE: &[(u32, u16)] = &[
    (NOTE_G5, 400), // 1 * 400ms
    (NOTE_E5, 200), // 0.5 * 400ms
    (NOTE_F5, 200), // 0.5 * 400ms
    (NOTE_G5, 400), // 1 * 400ms
    (NOTE_E5, 200), // 0.5 * 400ms
    (NOTE_F5, 200), // 0.5 * 400ms
    (NOTE_G5, 200), // 0.5 * 400ms
    (NOTE_G4, 200), // 0.5 * 400ms
    (NOTE_A4, 200), // 0.5 * 400ms
    (NOTE_B4, 200), // 0.5 * 400ms
    (NOTE_C5, 200), // 0.5 * 400ms
    (NOTE_D5, 200), // 0.5 * 400ms
    (NOTE_E5, 200), // 0.5 * 400ms
    (NOTE_F5, 200), // 0.5 * 400ms
    (NOTE_E5, 400), // 1 * 400ms
    (NOTE_C5, 200), // 0.5 * 400ms
    (NOTE_D5, 200), // 0.5 * 400ms
    (NOTE_E5, 400), // 1 * 400ms
    (NOTE_E4, 200), // 0.5 * 400ms
    (NOTE_F4, 200), // 0.5 * 400ms
    (NOTE_G4, 200), // 0.5 * 400ms
    (NOTE_A4, 200), // 0.5 * 400ms
    (NOTE_G4, 200), // 0.5 * 400ms
    (NOTE_F4, 200), // 0.5 * 400ms
    (NOTE_G4, 200), // 0.5 * 400ms
    (NOTE_E4, 200), // 0.5 * 400ms
    (NOTE_F4, 200), // 0.5 * 400ms
    (NOTE_G4, 200), // 0.5 * 400ms
    (NOTE_F4, 400), // 1 * 400ms
    (NOTE_A4, 200), // 0.5 * 400ms
    (NOTE_G4, 200), // 0.5 * 400ms
    (NOTE_F4, 400), // 1 * 400ms
    (NOTE_E4, 200), // 0.5 * 400ms
    (NOTE_D4, 200), // 0.5 * 400ms
    (NOTE_E4, 200), // 0.5 * 400ms
    (NOTE_D4, 200), // 0.5 * 400ms
    (NOTE_C4, 200), // 0.5 * 400ms
    (NOTE_D4, 200), // 0.5 * 400ms
    (NOTE_E4, 200), // 0.5 * 400ms
    (NOTE_F4, 200), // 0.5 * 400ms
    (NOTE_G4, 200), // 0.5 * 400ms
    (NOTE_A4, 200), // 0.5 * 400ms
    (NOTE_F4, 400), // 1 * 400ms
    (NOTE_A4, 200), // 0.5 * 400ms
    (NOTE_G4, 200), // 0.5 * 400ms
    (NOTE_A4, 400), // 1 * 400ms
    (NOTE_B4, 200), // 0.5 * 400ms
    (NOTE_C5, 200), // 0.5 * 400ms
    (NOTE_G4, 200), // 0.5 * 400ms
    (NOTE_A4, 200), // 0.5 * 400ms
    (NOTE_B4, 200), // 0.5 * 400ms
    (NOTE_C5, 200), // 0.5 * 400ms
    (NOTE_D5, 200), // 0.5 * 400ms
    (NOTE_E5, 200), // 0.5 * 400ms
    (NOTE_F5, 200), // 0.5 * 400ms
    (NOTE_G5, 200), // 0.5 * 400ms
];
