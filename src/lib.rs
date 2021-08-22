extern crate tsf_sys;

use tsf_sys::*;
use std::ffi::{CString, c_void};
use std::os::raw::c_int;

/// Object holding a pointer to a tsf instance.
///
/// This object manages the lifecycle of the tsf instance, remembering to free it when dropped.
///
/// Method signatures reflect their mutability, i.e. if a function would mutate the underlying C
/// structure, then it requires a mutable pointer. This includes basically all of the methods.
///
/// Whilst `Tsf` implements `Send` and `Sync`, requiring a mutable pointer to use many of the
/// methods ensures that two threads cannot mutate the underlying C object at once. Often, however,
/// this is exactly what you want to do: one thread calls `channel_note_on` and `channel_note_off`
/// whilst another thread (often driven by the audio device or some API into it) will be requesting
/// data via `render_float`. For this, you will need to consider some kind of synchronization
/// solution.
///
/// A simple solution involves moving `Tsf` into a `Mutex<Tsf>`. Then, when each thread
/// wants to use it, they can lock their `Mutex` to acquire a guard which dereferences to
/// `&mut Tsf`. The downside is that each thread may block as it waits to acquire the lock.
///
/// Another solution is to have the render thread maintain ownership of `Tsf`, and this thread has a
/// `Receiver` half of an mpsc channel which it periodically checks for messages (e.g. once between
/// each call to `render_float`). These messages contain instructions to be carried out on a
/// `&mut Tsf` (which the render thread has access to and no other threads do). The messages are
/// sent from other threads which want to mutate `Tsf` with e.g. `channel_note_on` commands. The
/// key advantage of this approach is that the render thread is in full control, and never blocks
/// waiting for a lock. It can choose to process as many or as few messages as it likes at a time.
pub struct Tsf {
    /// The unsafe pointer to tsf itself.
    tsf_ptr: *mut tsf_sys::tsf,
    /// We cache the output mode that we last configured TSF with. This is None iff `set_output` has
    /// not been called.
    output_mode: Option<OutputMode>
}

impl Drop for Tsf {
    fn drop(&mut self) {
        unsafe { tsf_close(self.tsf_ptr) };
    }
}

impl Tsf {
    fn new(tsf_ptr: *mut tsf) -> Self {
        Tsf {
            tsf_ptr,
            output_mode : None
        }
    }

    /// Load from a filename of a soundfont (.sf2) file.
    // TODO: Should really take a path or something like that
    pub fn load_filename<T: Into<Vec<u8>>>(filename: T) -> Tsf {
        let filename_cstring = CString::new(filename).expect("filename wasn't a valid C string - did it have an internal 0 byte?");
        let tsf_ptr: *mut tsf = unsafe { tsf_load_filename(filename_cstring.as_ptr()) };
        Tsf::new(tsf_ptr)
    }

    /// Load from memory.
    pub fn load_memory<T: Into<Vec<u8>>>(buffer: T) -> Tsf {
        let vec = buffer.into();
        let tsf_ptr: *mut tsf = unsafe { tsf_load_memory(
            vec.as_ptr() as *const c_void,
            vec.len() as c_int) };
        Tsf::new(tsf_ptr)
    }

    /// Free the memory related to this TSF instance.
    pub fn close(self) {
        // Don't actually need to do anything. This function takes ownership (by move) of the TSF
        // struct and then drops it, and the Drop code does the rest.
    }

    /// Setup the parameters for the voice render methods.
    /// * `mode`: if mono or stereo and how stereo channel data is ordered
    /// * `sample_rate`: the number of samples per second (output frequency)
    /// * `global_gain_db`: volume gain in decibels (>0 means higher, <0 means lower)
    pub fn set_output(&mut self, mode: OutputMode, sample_rate: u16, global_gain_db: f32) {
        let converted_mode = match mode {
            OutputMode::StereoInterleaved => TSFOutputMode_TSF_STEREO_INTERLEAVED,
            OutputMode::StereoUnweaved => TSFOutputMode_TSF_STEREO_UNWEAVED,
            OutputMode::Mono => TSFOutputMode_TSF_MONO,
        };
        unsafe { tsf_set_output(self.tsf_ptr, converted_mode, sample_rate as c_int, global_gain_db) };

        // Remember the output mode that was set.
        self.output_mode = Some(mode);
    }

    /// Render the specified number of samples as a float array.
    ///
    /// Panics if called before `set_output`.
    ///
    /// Returns a buffer of size: samples * number of output channels (the number of output channels
    /// was specified by `Tsf::set_output`).
    pub fn render_float(&mut self, samples: usize) -> Vec<f32> {
        let output_channels = match self.output_mode.as_ref().expect("set_output not yet called") {
            OutputMode::StereoInterleaved => 2,
            OutputMode::StereoUnweaved => 2,
            OutputMode::Mono => 1,
        };

        let mut dst: Vec<f32> = Vec::with_capacity(samples * output_channels);
        let dst_ptr: *mut f32 = dst.as_mut_ptr();
        unsafe {
            tsf_render_float(self.tsf_ptr, dst_ptr, samples as c_int, 0);
            dst.set_len(samples*output_channels);
        }
        dst
    }

    pub fn channel_note_on(&mut self, channel: u16, key: u8, vel: f32) {
        assert!(key <= 127u8, "key must be between 0 and 127");
        assert!(vel >= 0f32 && vel <= 1f32, "vel must be between 0.0 and 1.0");
        unsafe { tsf_channel_note_on(self.tsf_ptr, channel as c_int, key as c_int, vel) };
    }

    pub fn channel_note_off(&mut self, channel: u16, key: u8) {
        assert!(key <= 127u8, "key must be between 0 and 127");
        unsafe { tsf_channel_note_off(self.tsf_ptr, channel as c_int, key as c_int) };
    }

    pub fn channel_note_off_all(&mut self, channel: u16) {
        unsafe { tsf_channel_note_off_all(self.tsf_ptr, channel as c_int) };
    }

    pub fn note_off_all(&mut self) {
        unsafe { tsf_note_off_all(self.tsf_ptr) };
    }

    pub fn channel_set_preset_number(&mut self, channel: u16, preset_number: u16, mididrums: bool) {
        unsafe { tsf_channel_set_presetnumber(self.tsf_ptr, channel as c_int, preset_number as c_int, if mididrums { 1 } else { 0 }) };
    }

    pub fn get_preset_count(&mut self) -> u16 {
        unsafe { tsf_get_presetcount(self.tsf_ptr) as u16 }
    }
}

pub enum OutputMode {
    StereoInterleaved,
    StereoUnweaved,
    Mono
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_filename_and_render_c3() {
        let mut tsf = Tsf::load_filename("test_resources/sinewave.sf2");

        assert_eq!(1, tsf.get_preset_count());

        let sample_rate = 48000;
        let samples = (sample_rate / 10) as usize;  // render 0.1s at a time

        tsf.set_output(OutputMode::Mono, sample_rate, 0f32);

        // Render some silence
        let silence_samples = tsf.render_float(samples);
        assert!(silence_samples.into_iter().all(|x| x==0f32), "Didn't get silence");

        // Start a note and render it. We need to initialize the channel with a preset
        tsf.channel_set_preset_number(0, 0, false);
        tsf.channel_note_on(0, 48, 1f32);

        let note_on_samples = tsf.render_float(4800);
        assert!(note_on_samples.into_iter().any(|x| x!=0f32), "Got silence");

        // End a note - expect it to ring for a little while.
        tsf.channel_note_off(0, 48);
        let note_off_samples = tsf.render_float(4800);
        assert!(note_off_samples.into_iter().any(|x| x!=0f32), "Got silence");

        // Skip ahead a long time. There's no way the note rings for 10s right? Well 10s at 48000
        // samples per second is 480k samples. Let's render that many and discard them.
        for _ in 0..100 {
            tsf.render_float(samples);
        }

        // Now check we are back to silence
        let silence_after_samples = tsf.render_float(samples);
        assert!(silence_after_samples.into_iter().all(|x| x==0f32), "Didn't get silence");

        // Note that we do not need to call `close` - letting tsf fall out of scope / fall off the
        // stack is sufficient.
    }
}
