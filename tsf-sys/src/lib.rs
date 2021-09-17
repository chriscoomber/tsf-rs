#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::{c_void, CString};
    use std::os::raw::c_int;

    use super::*;

    const MINIMAL_SOUND_FONT: &'static [u8] = &[
        b'R',b'I',b'F',b'F',220,1,0,0,b's',b'f',b'b',b'k',
        b'L',b'I',b'S',b'T',88,1,0,0,b'p',b'd',b't',b'a',
        b'p',b'h',b'd',b'r',76,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,255,0,255,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,
        b'p',b'b',b'a',b'g',8,0,0,0,0,0,0,0,1,0,0,0,b'p',b'm',b'o',b'd',10,0,0,0,0,0,0,0,0,0,0,0,0,0,b'p',b'g',b'e',b'n',8,0,0,0,41,0,0,0,0,0,0,0,
        b'i',b'n',b's',b't',44,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,
        b'i',b'b',b'a',b'g',8,0,0,0,0,0,0,0,2,0,0,0,b'i',b'm',b'o',b'd',10,0,0,0,0,0,0,0,0,0,0,0,0,0,
        b'i',b'g',b'e',b'n',12,0,0,0,54,0,1,0,53,0,0,0,0,0,0,0,
        b's',b'h',b'd',b'r',92,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,50,0,0,0,0,0,0,0,49,0,0,0,34,86,0,0,60,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        b'L',b'I',b'S',b'T',112,0,0,0,b's',b'd',b't',b'a',b's',b'm',b'p',b'l',100,0,0,0,86,0,119,3,31,7,147,10,43,14,169,17,58,21,189,24,73,28,204,31,73,35,249,38,46,42,71,46,250,48,150,53,242,55,126,60,151,63,108,66,126,72,207,
        70,86,83,100,72,74,100,163,39,241,163,59,175,59,179,9,179,134,187,6,186,2,194,5,194,15,200,6,202,96,206,159,209,35,213,213,216,45,220,221,223,76,227,221,230,91,234,242,237,105,241,8,245,118,248,32,252
    ];

    #[test]
    fn load_memory() {
        unsafe {
            let sf_ptr = MINIMAL_SOUND_FONT.as_ptr();
            let sf_len = MINIMAL_SOUND_FONT.len();
            let tsf = tsf_load_memory(sf_ptr as *const c_void, sf_len as c_int);

            let preset_count = tsf_get_presetcount(tsf);
            assert_eq!(1, preset_count);

            tsf_close(tsf);
        }
    }

    #[test]
    fn load_filename_and_render_c3() {
        unsafe {
            let filename = CString::new("test_resources/sinewave.sf2").unwrap();
            let tsf = tsf_load_filename(filename.as_ptr());

            let preset_count = tsf_get_presetcount(tsf);
            assert_eq!(1, preset_count);

            let sample_rate = 44100;
            let samples = sample_rate / 10;  // Sample 0.1s below
            let output_mode = TSFOutputMode_TSF_MONO;
            let channels = 1;  // Mono has one channel
            let note: c_int = 48;  // MIDI note 48 is C3, i.e. one octave below middle C

            tsf_set_output(tsf, output_mode, sample_rate as c_int, 0f32);

            {
                let mut dst: Vec<f32> = Vec::with_capacity(samples * channels);
                let dst_ptr: *mut f32 = dst.as_mut_ptr();
                tsf_render_float(tsf, dst_ptr, samples as c_int, 0);
                dst.set_len(samples * channels);
                // Should get silence
                assert!(dst.iter().all(|x| *x == 0f32), "Didn't get silence");
            }

            tsf_note_on(tsf, 0 as c_int, note, 1.0f32);

            {
                let mut dst: Vec<f32> = Vec::with_capacity(samples * channels);
                let dst_ptr: *mut f32 = dst.as_mut_ptr();
                tsf_render_float(tsf, dst_ptr, samples as c_int, 0);
                dst.set_len(samples * channels);
                // Shouldn't get silence.
                assert!(dst.iter().any(|x| *x != 0f32), "Got silence");

                // Just in case you wanted to analyze the floats yourself to double check that we
                // get a MIDI note 48, which is a C3 (i.e. ~130.81Hz), we output a .csv.
                // Since this data covers 0.1s, you should see around 13 peaks/troughs.
                let (_, csv) = dst.iter().fold((0, String::new()), |acc, x| {
                    let (next_note_index, string_builder) = acc;
                    (next_note_index + 1, string_builder + &format!("{},{}\n", next_note_index as f32 / sample_rate as f32, x))
                });
                std::fs::create_dir_all("test_outputs").expect("Failed to create test_outputs directory - is there a non-directory already with that name?");
                std::fs::write("test_outputs/load_soundfont_from_file_and_render_c3.csv", csv).expect("Failed to write file");
            }

            tsf_close(tsf);
        }
    }
}