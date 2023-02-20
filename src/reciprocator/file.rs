use std::path::Path;

use rimd::{Event, SMF, SMFBuilder, SMFWriter, TrackEvent};

use crate::reciprocator::event;

// ConstructOutputFileName constructs a new output file name based on the input file name
pub fn construct_output_filename(filename: &str, invert: bool) -> String {
	let ext = match Path::new(filename).extension() {
		Some(ext) => match ext.to_str() {
			Some(ext) => ext,
			None => return String::new(),
		},
		None => return String::new(),
	};

	let pos = match filename.rfind(ext) {
		Some(pos) => pos,
		None => return String::new(),
	};

	if invert {
		format!("{}-inverted.{}", &filename[..pos - 1], ext)
	} else {
		format!("{}-negative.{}", &filename[..pos - 1], ext)
	}
}

// DebugMisc prints out any message that is not a NoteOn/NoteOff message
// fn debug_misc(p: *reader.Position, m: midi.Message) {
// 	if !strings.Contains(m.String(), "channel.Note") {
// 		fmt.Printf("DEBUG MISC: %v\n", m)
// 	}
// }

// DebugNote prints out NoteOn/NoteOff messages
// fn debug_note(p: *reader.Position, channel: uint8, key: uint8, velocity: uint8) {
// 	fmt.Printf("DEBUG NOTE: Position %v, Channel %v, Key %v, Velocity %v\n", p, channel, key, velocity)
// }

// DebugSMF prints out the contents of a standard midi file
pub fn debug_smf(filename: &str) {
	let tracks = match SMF::from_file(Path::new(filename)) {
		Ok(x) => x.tracks,
		Err(err) => panic!("Could not read {}: {}", &filename, err),
	};

	for track in tracks {
		for event in track.events {
			println!("{}", event);
		}
	}
}

// DebugSMFHeader prints out the contents of a standard midi file header
// fn debug_smf_header(h: smf.Header) {
// 	fmt.Println(h)
// }

// WriteFile writes an inverted set of notes to a new standard midi file
pub fn write_file(filename: &str, tonal_center_midi_key: u8, output_filename: String, invert: bool) {
	let tracks = match SMF::from_file(Path::new(&filename)) {
		Ok(x) => x.tracks,
		Err(e) => panic!("{}", e),
	};

	let mut builder = SMFBuilder::new();
	let mut current_track = builder.num_tracks();

	for track in tracks {
		builder.add_track();

		for track_event in track.events {
			let altered_event: TrackEvent = match track_event.event {
				Event::Midi(msg) => TrackEvent{
					vtime: track_event.vtime,
					event: Event::Midi(event::handle_message(msg, invert, tonal_center_midi_key)),
				},
				_ => track_event,
			};

			builder.add_event(current_track, altered_event);
		}

		current_track += 1;
	}

	let writer = SMFWriter::from_smf(builder.result());
	let result = writer.write_to_file(Path::new(&output_filename));

	match result {
		Ok(_) => println!("Success writing to {}", &output_filename),
		Err(err) => println!("Error writing to {}: {}", &output_filename, err),
	}
}