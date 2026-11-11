//! The rooms that are out of the building. Spec 0063.
//!
//! **They ask before they show, and that is why they are here.** Every room
//! in this arcade that works puts its idea in the world, where you meet it
//! by doing something ordinary: you walk at the horizon and never arrive,
//! you look in the mirror and you are there, you zoom the Mandelbrot with
//! your own hands. These put the idea in your head first and then show you
//! a room. Forty doors are forty doors unless you are already holding "and
//! there are infinitely many". A staircase is a staircase unless you are
//! already holding "and three of these disagree about which way is down".
//!
//! Jake played it and said so twice, the second time after spec 0062 had
//! rebuilt the hotel round the complaint: the code and logic are good, the
//! composition in capriccio does not land, and until there is a way to make
//! them feel right for a user, keep them out.
//!
//! **Parked and not deleted, which here means still compiled and still
//! tested.** A deleted room is recoverable and rotten: nothing builds it,
//! so the first engine change that touches its corner of the API breaks it
//! silently and nobody finds out until somebody tries to bring it back.
//! What is under here still answers for itself.
//!
//! `tour.rs` is the reason that matters today. Spec 0049's route planner
//! walks inside a rotated gravity frame, and the whole of that machinery is
//! [`aloft`]'s: `walked_in_frame`, `out_of_frame`, `box_out_of_frame` and
//! `in_frame`. Deleting the imaginarium would delete a walk nothing else in
//! this project knows how to do.
//!
//! **Nothing in the building reaches in here.** The test that says so is
//! `room::tests::nothing_of_the_building_is_parked`. What does reach in is
//! `tour`, for the frame walk above, and that is the one exception.
//!
//! `#![allow(dead_code)]` because that is what parked means: these are
//! public items that nothing outside their own tests calls, on purpose, and
//! the warning would otherwise be the only thing standing between this and
//! a gate that fails for being right.
#![allow(dead_code)]

/// Hilbert's hotel: a corridor with a room for every number, a desk that
/// can always take one more, and the two waves that make that true. Specs
/// 0029, 0030 and 0034.
pub mod aleph;
/// The imaginarium: one room, three mutually perpendicular downs, and the
/// doors between them. Specs 0037, 0038, 0047 and 0048.
pub mod aloft;
/// Cantor's diagonal, written across a wall and walked. Specs 0031 and
/// 0032.
pub mod register;
/// The same rooms further apart: door n at the nth Fibonacci metre. Spec
/// 0035.
pub mod twine;
/// One line somebody types, which was the answer to the watchman's riddle
/// and is now nobody's. Spec 0059.
pub mod typed;
