#![crate_type = "cdylib"]

use std::mem::transmute;
use chrono::{DateTime, Utc};

type Timestamp = DateTime<Utc>;

pub struct DataFrame {
    spectrum: Vec<Spectrum>,
    waveform: Waveform,
    channel_index: usize
}



pub struct Spectrum {
    f0: f64,
    df: f64,
    values: Vec<f64>,
    centre_time: Timestamp
}

pub struct Waveform {
    t0: Timestamp,
    dt: f64,
    samples: Vec<f64>
}

fn timestamp_from_labview(timestamp: f64) -> Timestamp {
    Utc::now()
}



#[no_mangle]
pub extern "C" fn new_frame(channel_index: i32, t0_timestamp: f64, dt: f64, values: *mut f64, value_length: i32) -> *mut DataFrame {

    let values_slice = unsafe {
        let length = value_length as usize;
        std::slice::from_raw_parts(values, length)
    };

    let waveform = Waveform {
        t0: timestamp_from_labview(t0_timestamp),
        dt,
        samples: Vec::from(values_slice)
    };

    let boxed = Box::new(DataFrame {
        channel_index: channel_index as usize,
        waveform,
        spectrum: Vec::new()
    });

    unsafe { transmute(boxed) }

}

#[no_mangle]
pub extern "C" fn add_spectrum(frame_ptr: *mut DataFrame, centre_timestamp: f64, f0: f64, df: f64, values: *mut f64, value_length: i32) {

    let frame = unsafe { &mut *frame_ptr };

    let values_slice = unsafe {
        let length = value_length as usize;
        std::slice::from_raw_parts(values, length)
    };

    let spectrum = Spectrum {
        centre_time: timestamp_from_labview(centre_timestamp),
        f0,
        df,
        values: Vec::from(values_slice)
    };

    frame.spectrum.push(spectrum);
}

#[no_mangle]
pub extern "C" fn free_frame(frame_ptr: *mut DataFrame) {
    let _frame: Box<DataFrame> = unsafe { transmute(frame_ptr) };
    //Drop 
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
