//! The hotel, through the doorway in the gallery's west wall. Specs 0029
//! and 0062.
//!
//! A lobby you arrive in, a desk, a board of keys with nothing in any hole,
//! and a sign saying every room is taken and there are always more rooms.
//! One door leaves it, west, and what is behind that door is the book the
//! sign would have to be kept in: `register`, which is Cantor's diagonal.
//!
//! **Spec 0029 built a corridor of numbered doors here and spec 0062 took
//! it out.** Two kilometres of it, forty doors drawn round wherever you
//! stood, four arrivals and a wave of occupancy running away from you down
//! the length of it. Jake played it and said it was the least interesting
//! room in the building, which the repo had been telling him for three
//! weeks in its own words: opening and closing doors does not demonstrate
//! the point, and no number of doors is the number that matters.
//!
//! What is left is the half that was always something you do. You walk the
//! book, you come out carrying a guest who is on no line of it, and you
//! hand them to the desk. The desk says yes, and it does not help.
//!
//! Where everything is, is arithmetic, and arithmetic can be checked
//! without a window.

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

use crate::room::{gallery_wall, GALLERY_WIDE, THICK};

/// A door, and how far it stands open.
pub const DOOR_TALL: f32 = 1.9;
pub const DOOR_THICK: f32 = 0.07;

/// The gap under the leaf, which is the thing the light comes under.
///
/// **The first build had none.** The leaf stood on the floor and the light
/// was a strip starting at the middle of the door's thickness, so half of
/// every strip was buried inside the leaf and the rest came out from under a
/// door that was shut to the ground. Jake walked it and said the lights clip,
/// which is what that is.
pub const DOOR_GAP: f32 = 0.012;
pub const SWINGS_TO: f32 = 1.75;

/// How wide the register's door is.
///
/// **Wider than a bedroom door, because it is the only one you walk
/// through.** Spec 0029's forty were scenery: they opened and shut and
/// nobody passed them, and this was the one way into the register. At their
/// width of 1.05 its opening was 1.04 against a body 0.90 across, which is
/// 0.07 of room either side. Driving an approach into it a frame at a time,
/// 65, 66 and 67 got through, 68 and 69 did not, and 70 did. A player who
/// bounces off it twice concludes the room is not there.
///
/// Spec 0062 took the forty away, so this is the only door width this room
/// has and the number it beat is written out above rather than named.
///
/// It is a hall of records through there, and an archway to one has no
/// reason to be the size of a bedroom door.
pub const BLANK_WIDE: f32 = 1.4;

/// How finely a room's number is drawn into its plaque.
///
/// One texture a door, made once when the building loads. Forty of them is
/// forty small textures, which is less than one cabinet's screenshot.
pub const PLAQUE_TEXELS: f32 = 64.0;

/// How deep the lobby is, west from the gallery's wall, and how wide.
///
/// **Not the size of the back room opposite.** That one is a maintenance
/// room and is the space that was left over. This is what a hotel is for:
/// you arrive in it and you stand at a desk. Every room in this building has
/// been too small on its first build, so the walkways were laid out first
/// and the furniture fitted to them.
///
/// It was laid out as the mouth of a corridor, with the desk set aside so as
/// not to stand across the way down it. Spec 0062 took the corridor out and
/// left the room its size, because what the size was for was standing room
/// and that has not changed.
pub const DEEP: f32 = 8.0;

/// And how high, which is the gallery's.
///
/// A lobby is a room you arrive in, and spec 0029 made the corridor leaving
/// it lower so that the difference said which was which. There is no
/// corridor now and this room keeps the gallery's height, which is what a
/// room you arrive in wants anyway.
pub const LOBBY_TALL: f32 = crate::room::TALL;

/// Where the lobby's inside begins, which is the far face of the gallery's
/// west wall, and where it ends.
pub fn span() -> (f32, f32) {
    let east = -GALLERY_WIDE * 0.5 - THICK;

    (east - DEEP, east)
}

/// And the same along `z`, which is the gallery's own span exactly.
///
/// **Derived and not chosen.** The first build named 7.6 and it reached past
/// the gallery's near end into the nook's floor, which three of this
/// building's own tests caught before anything was drawn. Taking the
/// gallery's span means the two rooms line up by construction and a gallery
/// that changes takes this with it.
///
/// Note this is not quite centred on [`middle_z`]: the doorways sit on the
/// back room's line, which is 0.12 south of the gallery's middle. The
/// corridor follows the doorways rather than the room, because what has to
/// line up is the thing you look through.
pub fn across(reaches: f32) -> (f32, f32) {
    (gallery_wall(reaches), -reaches)
}

/// The line everything here is built on.
///
/// The back room's doorway, which is the gallery's other one, and this takes
/// its `z` from there rather than keeping a second account: spec 0029 says
/// the two are a pair and should be built as one thing so they cannot drift.
/// Stand in one and you look through the other.
pub fn middle_z(reaches: f32) -> f32 {
    let (from, to) = crate::back::way(reaches);

    (from + to) * 0.5
}

/// The doorway in the gallery's west wall, as the two `z` it runs between.
///
/// The back room's exactly, mirrored across the gallery. One account.
pub fn way(reaches: f32) -> (f32, f32) {
    crate::back::way(reaches)
}

/// The gallery's west wall, in two pieces with the doorway between them.
///
/// Spec 0022 built it as one box and `room.rs` kept it inline. It is this
/// now, and the gallery's own code asks here for it, the same way it asks
/// `back::gallery_east` for the other side.
pub fn gallery_west(reaches: f32) -> Vec<Aabb> {
    let far = gallery_wall(reaches);
    let (_, east) = span();
    let (from, to) = way(reaches);

    vec![
        Aabb::new(
            vec3(east, 0.0, far - THICK),
            vec3(-GALLERY_WIDE * 0.5, crate::room::TALL, from),
        ),
        Aabb::new(
            vec3(east, 0.0, to),
            vec3(-GALLERY_WIDE * 0.5, crate::room::TALL, -reaches),
        ),
    ]
}

/// The lobby's floor, which laps under the gallery's rather than meeting it.
///
/// **Two floors at one height that meet edge to edge stop you dead**, and
/// `walk` reports it as nothing at all. This building found that three times
/// in one day, and the back room opposite is built the same way for the same
/// reason. So this one runs east past the gallery's west edge and under its
/// wall, and the gallery's west face is inside it.
pub fn floor(reaches: f32) -> Aabb {
    let (west, _) = span();
    let (from, to) = across(reaches);

    Aabb::new(
        vec3(west - THICK, -THICK, from - THICK),
        vec3(-GALLERY_WIDE * 0.5 + THICK, 0.0, to + THICK),
    )
}

/// Everything solid in here: the lobby's shell, the desk and the door.
///
/// The gallery's wall is not in this list. It stands already and is cut for
/// the doorway by [`gallery_west`], which `room.rs` asks for.
/// The lid over the lobby, which is what this room shows the sky.
///
/// Named rather than built inside `solid`, because spec 0040 lays a roof on
/// it and a second account of where it is would drift from the first.
/// **East to the far face of the wall it shares, not to its near one.**
/// Every other side of this lid oversails by a thickness and this one used to
/// stop flush, which left the 0.3 the gallery's west wall stands on roofed by
/// neither room. Along the solid wall nothing noticed. At the doorway, where
/// that strip is cut away, `Room::outside` probed it, found nothing over it,
/// and took the opening for a wall the sky could see: it plastered a face on
/// each side of the gap, facing each other. Both are culled from either room,
/// so the doorway looked open, and standing between them the view was plaster
/// whichever way you turned along the wall. Jake walked into the hotel and
/// found it.
///
/// Taken off `GALLERY_WIDE` rather than `east + THICK`, because that is where
/// `gallery_west` puts the wall's far face. The two agree today and they are
/// not the same question.
pub fn lid(reaches: f32) -> Aabb {
    let (west, _) = span();
    let (from, to) = across(reaches);

    Aabb::new(
        vec3(west - THICK, LOBBY_TALL, from - THICK),
        vec3(-GALLERY_WIDE * 0.5, LOBBY_TALL + THICK, to + THICK),
    )
}

/// The lobby's north wall, which is also the nook's south wall.
///
/// Named because two rooms want it. This room builds it and `standing` puts
/// it in the hotel's run, so both its faces wear the hotel's plaster, and the
/// nook's side of it is dressed by `room::nook_lining` with the grain the
/// rest of that room wears.
pub fn lobby_north(reaches: f32) -> Aabb {
    let (west, east) = span();
    let (_, to) = across(reaches);

    Aabb::new(
        vec3(west - THICK, 0.0, to),
        vec3(east, LOBBY_TALL, to + THICK),
    )
}

pub fn solid(reaches: f32) -> Vec<Aabb> {
    let (west, east) = span();
    let (from, to) = across(reaches);
    let (door_from, door_to) = book_door(reaches);

    vec![
        floor(reaches),
        desk(reaches),
        // the lobby's far wall, whole. Spec 0035 put a second two kilometre
        // hall through it, east of the desk, and spec 0062 took that hall
        // out: a comparison between two corridors is worth nothing once
        // there are no corridors.
        Aabb::new(
            vec3(west - THICK, 0.0, from - THICK),
            vec3(east, LOBBY_TALL, from),
        ),
        lobby_north(reaches),
        // and its west wall, in two pieces with the register's door between
        // them.
        //
        // **One opening in it rather than two.** Spec 0029's corridor left
        // through the whole middle of this wall, and the register's own door
        // stood a stride inside that corridor. Spec 0062 took the corridor
        // out and moved the door here, so this wall is cut once, for the one
        // way out of this room there is.
        Aabb::new(
            vec3(west - THICK, 0.0, from - THICK),
            vec3(west, LOBBY_TALL, door_from),
        ),
        Aabb::new(
            vec3(west - THICK, 0.0, door_to),
            vec3(west, LOBBY_TALL, to + THICK),
        ),
        // and the head over it, which starts at the door's own head rather
        // than at a corridor's ceiling, because what is through it now is a
        // room nearly twice this one's height and the opening is a doorway.
        Aabb::new(
            vec3(west - THICK, DOOR_TALL, door_from),
            vec3(west, LOBBY_TALL, door_to),
        ),
        lid(reaches),
        book_leaf(reaches),
    ]
}

/// Where the register's door stands in the lobby's west wall, as the two `z`
/// it runs between. Spec 0062.
///
/// On the lobby's own doorway line, so the gallery's door, this one and the
/// back room's are all on one axis: stand in any of them and you look
/// through the others.
///
/// **Its own width, wider than a numbered room's door was.** This is the one
/// door in this room anybody walks through, and at `DOOR_WIDE` its opening
/// was 1.04 against a body 0.90 across.
pub fn book_door(reaches: f32) -> (f32, f32) {
    let middle = middle_z(reaches);

    (middle - BLANK_WIDE * 0.5, middle + BLANK_WIDE * 0.5)
}

/// Its leaf where it stands, as a box: what you point at and what stops you.
///
/// **Standing open, hung on its hinge, and it does not shut.** It was a shut
/// painted door among forty shut painted doors, and Jake walked past it and
/// said it was not obviously a door you could go through. A door that is
/// drawn and not solid is a door you walk through, which is what the first
/// build of this room shipped.
///
/// Axis aligned, because everything solid in this building is, so the leaf
/// is the box its two ends fit in. Across the leaf and not around the whole
/// box: inflating the hinge-to-tip box on every side put `DOOR_THICK` of
/// solid past the hinge, inside the opening the door had just cleared.
pub fn book_leaf(reaches: f32) -> Aabb {
    let (west, _) = span();
    let (from, _) = book_door(reaches);
    let swing = SWINGS_TO;
    let hinge = vec3(west - THICK, 0.0, from);
    let tip = hinge + vec3(-swing.sin(), 0.0, swing.cos()) * BLANK_WIDE;
    let across = vec3(swing.cos(), 0.0, swing.sin()) * DOOR_THICK * 0.5;
    let corners = [hinge + across, hinge - across, tip + across, tip - across];
    let low = corners.iter().fold(corners[0], |was, is| was.min(*is));
    let high = corners.iter().fold(corners[0], |was, is| was.max(*is));

    Aabb::new(vec3(low.x, 0.0, low.z), vec3(high.x, DOOR_TALL, high.z))
}

/// How big the desk is and where it stands.
///
/// **Built rather than blocked out.** The first one was a single box in the
/// wall's own colour, which is what a thing looks like before it is built:
/// Jake stood at it and said it does not make sense to a user, and from close
/// to it filled half the frame and read as a wall. A counter is a body, a top
/// that oversails it and a bead under that, which is what the cellar's bar
/// and the pool table's rails are, and the shadow lines do the rest.
pub const DESK_WIDE: f32 = 2.6;
pub const DESK_DEEP: f32 = 0.7;
pub const DESK_TALL: f32 = 1.1;

/// How far the top oversails the body, and how thick it and the bead are.
pub const DESK_OVER: f32 = 0.06;
pub const DESK_TOP: f32 = 0.055;
pub const DESK_BEAD: f32 = 0.022;

/// Where the board of pigeonholes hangs: on the wall the desk stands
/// against, which is the low `z` side.
///
/// **Not on the desk's far face.** The first build put it at the counter's
/// `max.z`, which is the side facing into the room, so the board floated in
/// front of the desk between it and whoever was standing there.
pub fn board(reaches: f32) -> Vec3 {
    let counter = desk(reaches);

    vec3(
        counter.center().x,
        BOARD_UP,
        across(reaches).0 + BOARD_THICK * 0.5,
    )
}

/// The board of pigeonholes behind the desk, one hook to a room.
///
/// **Furniture and not a record.** Spec 0029 struck a board of keys that was
/// a second account of who is in, because two lists of one thing drift. This
/// is the same object with no job: it says what kind of place this is, and
/// what is in it tells you nothing.
pub const BOARD_WIDE: f32 = 2.2;
pub const BOARD_TALL: f32 = 1.15;
pub const BOARD_UP: f32 = 1.3;
pub const BOARD_THICK: f32 = 0.08;
pub const HOLES_ACROSS: u32 = 8;
pub const HOLES_DOWN: u32 = 4;

/// The bell on the counter, which is the one turned thing in here.
pub const BELL_WIDE: f32 = 0.11;
pub const BELL_TALL: f32 = 0.08;

/// Its profile, for `cellar::turned`: a dome on a little foot.
pub fn bell_profile(v: f32) -> (f32, f32) {
    // a quarter circle, so the dome meets the counter square and closes at
    // the top rather than coming to a point
    let along = v * BELL_TALL;
    let round = (v * std::f32::consts::FRAC_PI_2).cos();

    (along, BELL_WIDE * round.max(0.04))
}

/// Where the sign hangs, which is over the register's door.
///
/// It is the one place a person is looking when they walk in, and what it
/// says is the premise: without it the desk's numbers are reported to
/// somebody who has not been told what is unusual about them.
/// How tall a line of it is, how thick the board is, and the gap between the
/// two lines.
///
/// **No width.** The first build gave both lines one width of 2.4, which was
/// three quarters of the corridor it then hung in, and stretched each line's
/// texture to fill
/// it: sixteen characters and twenty four at the same width means two sizes
/// of letter, and Jake's picture has the top line half again as large as the
/// bottom. Each board takes its width from its own text now, through
/// `blitzkit::text::room_for`, the way the back room's gauge words do.
///
/// It also hung them 0.26 apart while each was 0.42 tall, so they overlapped.
///
/// **This is a ceiling on the line and not the line.** What sets the height
/// is `SIGN_FITS` over the longest line's aspect, which for twenty four
/// characters in a mouth 2.65 wide comes to 0.110. Raising this number does
/// nothing at all, which is worth knowing before anybody tries to make the
/// sign bigger by changing it. Held by `the_sign_hangs_in_the_band`.
pub const SIGN_TALL: f32 = 0.17;

/// How wide the longest line may be.
///
/// Spec 0029 hung the board in the corridor's mouth and took this off that
/// opening less its jambs, because a line wider than the opening had its
/// ends behind the wall: the second came out as "E ROOMS DO NOT RUN O T"
/// with THE and OUT lost. The letters are sized down from here rather than
/// up from a height, so a longer line shrinks the sign instead of running
/// off it.
///
/// Spec 0062 hangs it on the lobby's west wall over the register's door
/// instead, where there is no opening to lose a word behind and the wall is
/// six metres across. **The number is the one it was**, written out rather
/// than derived, so the lettering is the size it has always been and no kept
/// picture of this room moves for a change of siting.
pub const SIGN_FITS: f32 = 2.65;
pub const SIGN_THICK: f32 = 0.05;
pub const SIGN_GAP: f32 = 0.05;

/// How far over the door's head the board hangs.
///
/// Spec 0029 measured this down from the corridor's head and took three
/// tries. At 2.02 the board was below the door heads and across the middle
/// of the view from the lobby, which is what made it read as filling the
/// room. At six centimetres of clearance it read as touching from any angle
/// but square on, and Jake said the text runs into the ceiling.
///
/// Spec 0062 measures it up from the register door's head instead, since
/// there is no corridor to hang under, and the number is chosen so the
/// board's top lands exactly where it did: a gap you can see, well clear of
/// the head below and of the lobby's own ceiling above.
pub const SIGN_UNDER: f32 = 0.3;

/// Where the top of the sign is, which is over the register's door.
pub fn sign_top() -> f32 {
    DOOR_TALL + SIGN_UNDER
}

/// What the sign says, which is the premise.
///
/// **Nothing in this room stated it.** A person walked in, saw a corridor,
/// and was then told "The hotel is full. 1 housed", which only means anything
/// to somebody who already knows the hotel has a room for every counting
/// number and that it being full is supposed to leave no space. Jake stood at
/// the desk and said it does not make sense to a user, and this is most of
/// why.
///
/// Two lines, because the paradox needs both halves: every room is taken,
/// and there are always more rooms.
///
/// **The second line is Jake's, twice over.** It read THE ROOMS DO NOT RUN
/// OUT, which is stock language: a hotel that does not run out of rooms is a
/// hotel with a great many rooms, and that is not what this one does. He said
/// the point is that there is always a new one. These are his words for it.
pub const SIGN_SAYS: [&str; 2] = ["EVERY ROOM TAKEN", "THERE ARE ALWAYS MORE ROOMS"];

/// What the desk says while a move runs and after one.
///
/// Reporting the event rather than the state. "1 housed" is a counter, and
/// what happened was that a full hotel took another guest and nobody left,
/// which is a sentence.
pub const DESK_FULL: &str = "Full, and nobody has left.";
pub const DESK_HOUSED: &str = "housed since you came in";

/// The counter you stand at.
///
/// **Not across the way out.** The way out of this room is on the middle
/// line, so a desk on that line is a desk you walk round to get anywhere. It
/// stands against the far side instead, square to the way in, so you come
/// through the gallery's door and it is on your right with the register's
/// door straight ahead.
pub fn desk(reaches: f32) -> Aabb {
    let (west, east) = span();
    let (from, _) = across(reaches);
    let middle = (west + east) * 0.5;

    Aabb::new(
        vec3(middle - DESK_WIDE * 0.5, 0.0, from + 0.9),
        vec3(middle + DESK_WIDE * 0.5, DESK_TALL, from + 0.9 + DESK_DEEP),
    )
}

/// Where somebody waiting for a room stands.
///
/// In front of the counter on the way-in side, facing it, which is where
/// anybody who has just come through the door and wants a room would be.
/// Spec 0033.
pub fn waiting(reaches: f32) -> Vec3 {
    let desk = desk(reaches);

    vec3((desk.min.x + desk.max.x) * 0.5, 0.0, desk.max.z + 0.62)
}

/// The desk's book: how many it has housed, and how long it has left to say
/// so.
///
/// **What is left of it after spec 0062.** This held who was in every room
/// of an endless corridor, as a predicate walked backwards through the moves
/// applied to it, plus a front running away from you at six rooms a second,
/// a second wave behind that one, and a queue of four arrivals at the door.
/// All of it was machinery for a thing you watched happen.
///
/// What the room asks of the desk now is what spec 0032 always asked: that
/// it take the one guest the register has just proved it has no row for, and
/// say yes.
#[derive(Debug, Default)]
pub struct Hotel {
    /// How many have been housed since the building opened, which is the
    /// number that says nobody left.
    housed: u64,
    /// And how long the desk has left to say so.
    said_for: f32,
}

/// How long what the desk has just done stays on the board.
///
/// **A line that stays up for ever is a scoreboard.** Spec 0030 held this
/// window open with the waves it had started, so the desk got itself back
/// when they finished. There are no waves to hold it open now, so it is a
/// number, and it is about as long as somebody stands at a counter reading
/// one line.
pub const SAYS_FOR: f32 = 4.0;

impl Hotel {
    pub fn new() -> Self {
        Self::default()
    }

    /// How many have been housed since the building opened.
    pub fn housed(&self) -> u64 {
        self.housed
    }

    /// House whoever you are carrying, which is the guest the register's
    /// diagonal built and no row of the book has. Specs 0032 and 0062.
    ///
    /// **The desk says yes, and it does not help.** It has a room for them,
    /// because it has a room for anybody: the rooms are the counting numbers
    /// and it is never out of those. What it cannot do is finish the list,
    /// which is the thing that was being asked, and the way to show that is
    /// to house this one and let whoever did it walk the book again.
    pub fn house_carried(&mut self) {
        self.housed += 1;
        self.said_for = SAYS_FOR;
    }

    /// Whether the desk is still saying what it has just done.
    pub fn just_housed(&self) -> bool {
        self.said_for > 0.0
    }

    /// Count that window down.
    pub fn tick(&mut self, dt: f32) {
        self.said_for = (self.said_for - dt).max(0.0);
    }
}

/// A sconce on the wall between two doors, and how big it is.
///
/// **Every lamp sits in a fitting.** The cellar was told that once and the
/// baths did it again worse, with lamps along a ceiling and sconces on a wall
/// so the ceiling glowed at nothing and every sconce was dark. [`lamps`]
/// below is this list lifted, so there is no lamp here that is not in a
/// fitting and nobody has to remember.
pub const SCONCE: Vec3 = vec3(0.12, 0.26, 0.09);

/// How high they sit, and how far they stand off the wall.
pub const SCONCE_UP: f32 = 1.72;
pub const SCONCE_OUT: f32 = 0.07;

/// The lobby's ceiling: a grid of beams across it.
///
/// **The one surface in this room nobody had spent anything on.** It was
/// the taller of the two rooms the hotel had, and height was the whole of
/// what said which was which. A room that says it by being taller and then
/// shows you a flat grey lid is making the argument and dropping it. Jake
/// asked for this after a sweep found the lid bare, and it matters more now
/// that this room is the hotel rather than the half of it you pass through.
///
/// **A hotel's and not the cellar's.** `cellar::ceiling` is a library's:
/// coffers of 2.1 on beams of 0.2, which is close work for a room lined with
/// books. This room is arrived in, so the bays are wider and the beams are
/// deeper, and they are the hotel's own wood rather than the cellar's oak.
///
/// Both ways, because beams one way are stripes. The two sets cross and the
/// shared volume is inside both, which is hidden rather than fighting; the
/// cellar settled that already.
pub const LOBBY_COFFER: f32 = 2.6;
pub const LOBBY_BEAM: f32 = 0.26;
pub const LOBBY_BEAM_DOWN: f32 = 0.22;

pub fn lobby_ceiling(reaches: f32) -> Vec<Aabb> {
    let (west, east) = span();
    let (from, to) = across(reaches);
    let (deep, along) = (east - west, to - from);
    let (bays_x, bays_z) = (
        (deep / LOBBY_COFFER).round().max(1.0),
        (along / LOBBY_COFFER).round().max(1.0),
    );
    let (step_x, step_z) = (deep / bays_x, along / bays_z);
    let hung = LOBBY_TALL - LOBBY_BEAM_DOWN;
    let mut out = Vec::new();

    // **Cut for the doorway, the way the walls are.** The gallery's opening
    // into this room is full height, y 0 to `LOBBY_TALL` with no head over
    // it, so the beam that runs down the east wall crosses the top of it. A
    // run carried straight across an opening is the fault spec 0029 had with
    // this hotel's own skirting and rail, and `a_doorway_can_be_seen_out_of`
    // caught this one before anybody walked it: from inside the opening the
    // view into the lobby fell to a third of the view back into the gallery.
    let (door_from, door_to) = way(reaches);

    for n in 0..=(bays_x as usize) {
        let x = west + n as f32 * step_x;
        let spans: Vec<(f32, f32)> = if x + LOBBY_BEAM * 0.5 >= east {
            vec![(from, door_from), (door_to, to)]
        } else {
            vec![(from, to)]
        };

        for (a, b) in spans {
            if b - a <= LOBBY_BEAM {
                continue;
            }

            out.push(Aabb::new(
                vec3(x - LOBBY_BEAM * 0.5, hung, a),
                vec3(x + LOBBY_BEAM * 0.5, LOBBY_TALL, b),
            ));
        }
    }

    for n in 0..=(bays_z as usize) {
        let z = from + n as f32 * step_z;

        out.push(Aabb::new(
            vec3(west, hung, z - LOBBY_BEAM * 0.5),
            vec3(east, LOBBY_TALL, z + LOBBY_BEAM * 0.5),
        ));
    }

    out
}

/// The lobby's own fittings: one over the desk and one over the sign.
///
/// **The lobby had none.** The corridor was given sconces and the room you
/// arrive in was left with nothing, so the sign stating the premise and the
/// desk you work both stood in the dark. That is the same fault the corridor
/// had one round earlier, and this building has now had it three times.
pub fn lobby_sconces(reaches: f32) -> Vec<Vec3> {
    let counter = desk(reaches);
    let (west, _) = span();
    let middle = middle_z(reaches);
    let _ = counter;

    vec![
        // over the desk, on the wall it stands against, which is the low
        // `z` side: the desk faces the room and the wall is behind it
        vec3(
            counter.center().x,
            SCONCE_UP + 0.24,
            across(reaches).0 + SCONCE_OUT,
        ),
        // and one lighting the sign.
        //
        // **Above it and not in front of it.** The first one hung 0.55 out
        // into the lobby at the sign's own height, which put the fitting
        // between the reader and the board: Jake's picture has a grey box
        // through the middle of "DO NOT". A light that blocks the thing it
        // lights is worse than no light.
        vec3(west + 0.3, sign_top() + SCONCE.y * 0.5 + 0.06, middle),
    ]
}

/// How far the lobby's two reach, and how hard they burn.
///
/// **A lobby is not a corridor at night.** Spec 0029 lit the corridor at a
/// reach of 2.5, short on purpose: pools with dark between them are what say
/// how long it is. Giving the lobby the same number left a room eight metres by
/// six with two lamps that reached a quarter of the way across it, and the
/// one place anybody stands in it, where somebody waits at the desk, was 2.91
/// from the nearest and so lit by nothing at all.
///
/// Jake came in and said the shadowy figure is creepy. Painting it lighter
/// did nothing, because the problem was never the paint: a colour here is a
/// multiplier and there was no light to multiply.
pub const LOBBY_RANGE: f32 = 4.6;
pub const LOBBY_LIT: f32 = 0.95;

pub fn lobby_lamps(reaches: f32) -> Vec<Vec3> {
    lobby_sconces(reaches)
        .into_iter()
        .map(|at| at + Vec3::Y * SCONCE.y * 0.3)
        .collect()
}

/// The skirting along the foot of each wall, and the rail along the top.
///
/// A long wall with nothing breaking it is one slab. These are what break it,
/// and they are the cheapest thing in the building that does: two runs a
/// side, carried round the room between its openings.
pub const SKIRTING: f32 = 0.14;
pub const SKIRTING_OUT: f32 = 0.025;
pub const RAIL_UP: f32 = 1.02;
pub const RAIL_TALL: f32 = 0.07;

/// The architrave round a doorway: two jambs and a head.
///
/// A door that is one box in a flat wall is a brown rectangle. The frame is
/// what makes it a doorway, and it is three boxes.
pub const JAMB: f32 = 0.08;
pub const JAMB_OUT: f32 = 0.04;

/// The panels sunk into a door's face, and the handle on it.
///
/// **A door is not a slab.** Jake walked past the register's and said it does
/// not look like a door except that it is open, which was true of all forty
/// one of them: what makes a door read as a door close to is the two panels
/// and the thing you turn, and at a distance the frame already does the work.
pub const PANEL_IN: f32 = 0.11;
pub const PANEL_GAP: f32 = 0.055;
pub const PANEL_DEEP: f32 = 0.012;
pub const HANDLE: Vec3 = vec3(0.035, 0.035, 0.1);
pub const HANDLE_UP: f32 = 0.98;
pub const HANDLE_IN: f32 = 0.12;

#[cfg(test)]
mod tests {
    /// The lobby's ceiling is dressed, and dressed all the way to its walls.
    ///
    /// **The baths' lesson, applied before it could happen again.** That
    /// room's ribs stopped short of one end and left 4.24 metres of bare lid,
    /// because the dressing took its extent from something that was not the
    /// room. This takes its extent from `span` and `across`, which are the
    /// room, and this says so.
    #[test]
    fn the_lobbys_ceiling_reaches_its_walls() {
        let reaches = crate::room::Room::of(crate::games()).reaches;
        let beams = lobby_ceiling(reaches);
        let (west, east) = span();
        let (from, to) = across(reaches);

        assert!(beams.len() >= 4, "{} beams is not a ceiling", beams.len());

        let got = |lo: f32, hi: f32, of: fn(&Aabb) -> (f32, f32)| {
            let low = beams.iter().map(|b| of(b).0).fold(f32::MAX, f32::min);
            let high = beams.iter().map(|b| of(b).1).fold(f32::MIN, f32::max);

            (low - lo, hi - high)
        };
        let (west_short, east_short) = got(west, east, |b| (b.min.x, b.max.x));
        let (south_short, north_short) = got(from, to, |b| (b.min.z, b.max.z));

        for (way, short) in [
            ("west", west_short),
            ("east", east_short),
            ("south", south_short),
            ("north", north_short),
        ] {
            assert!(
                short <= LOBBY_BEAM * 0.5 + 1e-3,
                "the beams stop {:.2} short of the {} wall",
                short,
                way
            );
        }
    }

    /// And nothing it hangs crosses the way in.
    ///
    /// The gallery's opening into this room is full height, so a beam run
    /// along the east wall crosses the top of it: a bar across a doorway,
    /// which is what spec 0029's skirting and rail did to the corridor's
    /// openings before Jake walked into one. `a_doorway_can_be_seen_out_of`
    /// catches it from inside the opening; this says it in the arithmetic,
    /// where it reads as a rule rather than as a ratio of two pictures.
    #[test]
    fn no_beam_crosses_the_way_in() {
        let reaches = crate::room::Room::of(crate::games()).reaches;
        let (door_from, door_to) = way(reaches);
        let (_, east) = span();

        for beam in lobby_ceiling(reaches) {
            // the ones lying in the wall's own plane, which are thin across
            // and long along it. A beam running the other way stops at the
            // wall rather than crossing the opening, and from inside the
            // opening it is the view rather than a bar across it.
            if beam.max.x < east - 1e-3 || beam.max.x - beam.min.x > LOBBY_BEAM + 1e-3 {
                continue;
            }

            assert!(
                beam.max.z <= door_from + 1e-3 || beam.min.z >= door_to - 1e-3,
                "a beam runs z {:.2}..{:.2} across a doorway at {:.2}..{:.2}",
                beam.min.z,
                beam.max.z,
                door_from,
                door_to
            );
        }
    }

    /// And nothing it hangs comes down to where a head is.
    #[test]
    fn the_lobbys_beams_are_overhead() {
        let reaches = crate::room::Room::of(crate::games()).reaches;

        for beam in lobby_ceiling(reaches) {
            assert!(
                beam.min.y > crate::EYE + 0.3,
                "a beam hangs to {:.2}, which is in the way of a head at {:.2}",
                beam.min.y,
                crate::EYE
            );
            assert!(
                beam.max.y <= LOBBY_TALL + 1e-3,
                "a beam at {:.2} is above the ceiling it hangs from",
                beam.max.y
            );
        }
    }

    /// Spec 0029: every lamp in here sits in a fitting, and every fitting
    /// has a lamp.
    ///
    /// **Derived rather than placed twice.** The cellar lit a wall with
    /// nothing on it and the baths ran the lamps along a ceiling while the
    /// sconces stood on a wall, so every sconce was dark and the ceiling
    /// glowed at nothing. `lobby_lamps` is `lobby_sconces` lifted, which is
    /// why there cannot be one without the other.
    ///
    /// **It used to be stated over two sets.** Spec 0029's corridor had its
    /// own sconces and its own reach, and this held the split from quietly
    /// dropping one of them; spec 0062 took the corridor out, so the lobby's
    /// is the whole set and there is no split left to drop.
    #[test]
    fn every_lamp_is_in_a_fitting() {
        let reaches = reaches();
        let fittings = lobby_sconces(reaches);
        let lamps = lobby_lamps(reaches);

        assert_eq!(
            lamps.len(),
            fittings.len(),
            "there are {} lamps and {} fittings",
            lamps.len(),
            fittings.len()
        );

        for lamp in lamps.iter() {
            assert!(
                fittings.iter().any(|at| (*at - *lamp).length() < SCONCE.y),
                "a lamp at {:?} is in no fitting",
                lamp
            );
        }

        // and it reaches the one place anybody stands in it, which it did
        // not: 2.91 away with a reach of 2.5, so whoever waits at the desk
        // was lit by nothing at all
        let stands = waiting(reaches);

        assert!(
            lamps.iter().any(|at| (*at - stands).length() < LOBBY_RANGE),
            "nothing in the lobby reaches where somebody waits"
        );
    }

    /// Specs 0031 and 0062: the one door you walk through is wide enough to
    /// walk through.
    ///
    /// **It was a hit and miss passage.** At `DOOR_WIDE` the opening was
    /// 1.04 against a body 0.90 across, and the open leaf's collider ate
    /// another 0.07 of it by inflating its whole hinge-to-tip box by
    /// `DOOR_THICK` on every side, which put solid past the hinge inside the
    /// opening the door had just cleared. Driving an approach into it a
    /// frame at a time, 65, 66 and 67 got through, 68 and 69 did not, and 70
    /// did. The register is behind this door and nothing else leads there,
    /// so a player who bounces off it twice concludes the room is not there.
    ///
    /// The door is in the lobby's west wall now rather than in a corridor's
    /// south wall, so the opening runs in `z` and the leaf swings in `x`.
    /// The arithmetic is the same arithmetic with the axes swapped, which is
    /// the one thing worth checking after a move like that.
    #[test]
    fn the_one_door_you_walk_through_is_wide_enough() {
        let reaches = reaches();
        let (south, north) = book_door(reaches);
        let leaf = book_leaf(reaches);
        let body = crate::RADIUS * 2.0;

        // the opening, and what the leaf standing open takes out of it
        let clear = north - leaf.max.z.max(south);

        assert!(
            clear > body + 0.3,
            "the way into the register is {:.2} clear for a body {:.2} across",
            clear,
            body
        );

        // and the leaf is a leaf rather than the box it sweeps: it reaches
        // back past its hinge by half a thickness, which is what a leaf
        // hinged on a jamb reaches back by. A whole one is the old box
        // inflated on every side, and a bound of exactly `DOOR_THICK` lands
        // on that to within a rounding error, so it is not a bound at all.
        assert!(
            leaf.max.z - south < DOOR_THICK * 0.6,
            "the open leaf stands {:.3} inside its own doorway, which is {:.3} thick",
            leaf.max.z - south,
            DOOR_THICK
        );
    }

    /// Spec 0033: whoever is waiting at the desk is facing the desk.
    ///
    /// **It was facing the way in.** Yaw nought is `-z` and the counter is
    /// at `-z` of where they stand, and the first build turned them a half
    /// turn from that, so the one person in the lobby stood with its back to
    /// the desk watching the door. Jake came in and said the shadowy figure
    /// is creepy. It was not the dark that did that.
    #[test]
    fn whoever_is_waiting_faces_the_desk() {
        let reaches = crate::room::Room::of(crate::games()).reaches;
        let stands = waiting(reaches);
        let counter = desk(reaches);

        // the desk is behind them in `z`, which is the way yaw nought looks
        assert!(
            counter.center().z < stands.z,
            "the desk is not in front of where somebody waits"
        );

        // and they are clear of it rather than standing in it
        assert!(
            stands.z > counter.max.z,
            "whoever waits is standing inside the counter"
        );
        assert!(
            stands.z - counter.max.z < 1.0,
            "whoever waits is {:.2} back from the counter, which is not at it",
            stands.z - counter.max.z
        );
    }

    /// Specs 0029 and 0062: the sign hangs in the band between the register
    /// door's head and the lobby's ceiling, clear of both.
    ///
    /// **Both edges have been wrong.** The first build hung the top at 2.02,
    /// below the door heads and across the middle of the view in from the
    /// lobby. The fix put the top 0.06 under the corridor's head, which is a
    /// gap in the arithmetic and none at all on screen: Jake said the text
    /// runs into the ceiling. The band is what is left between the two, and
    /// the lines and the gap are sized to sit inside it rather than either
    /// one being chosen on its own.
    ///
    /// The band it has to sit in is a different band now. The corridor's
    /// head was at 2.4 and the lobby's ceiling is at 3.2, so there is more
    /// room over the sign than there was, and the edge that still bites is
    /// the door head under it.
    #[test]
    fn the_sign_hangs_in_the_band() {
        let wide = super::SIGN_SAYS
            .iter()
            .map(|says| blitzkit::text::room_for(says, 1.0).x)
            .fold(0.0f32, f32::max);

        // the same height `draw_the_hotel` works the lines out at, which for
        // these two lines is the width cap rather than `SIGN_TALL`
        let line = super::SIGN_TALL.min(super::SIGN_FITS / wide);
        let top = super::sign_top();
        let foot = top - line * 2.0 - super::SIGN_GAP;

        assert!(
            super::LOBBY_TALL - top >= 0.15,
            "the sign is {:.3} under the ceiling, which reads as touching it",
            super::LOBBY_TALL - top
        );

        assert!(
            foot >= super::DOOR_TALL,
            "the sign reaches to {:.3}, below the door head at {:.3}",
            foot,
            super::DOOR_TALL
        );
    }

    use super::*;
    use crate::room::Room;
    use crate::walk;

    fn reaches() -> f32 {
        Room::of(crate::games()).reaches
    }

    /// Spec 0029: the doorway here and the back room's are one account.
    #[test]
    fn the_two_doorways_are_one_account() {
        let reaches = reaches();

        assert_eq!(way(reaches), crate::back::way(reaches));
    }

    /// Spec 0062: the lobby is closed but for the two doorways it has.
    ///
    /// **This is the guard the cut needed.** Spec 0029's corridor left
    /// through the whole middle of the west wall and spec 0035's long hall
    /// through the far one, so taking both out meant closing two openings
    /// and cutting one new one. A wall left part open is a hole you find by
    /// walking into it, and the lid over this room is what the sky sees
    /// through such a hole: spec 0029 has that fault recorded twice
    /// already, once at this very doorway.
    ///
    /// Swept along each wall rather than checked at its corners, which is
    /// the building's own note on borrowed walls and is how both of those
    /// were missed the first time.
    #[test]
    fn one_way_out_of_the_lobby() {
        let reaches = reaches();
        let (west, east) = span();
        let (from, to) = across(reaches);
        let (door_from, door_to) = book_door(reaches);
        let (way_from, way_to) = way(reaches);
        let boxes: Vec<Aabb> = solid(reaches)
            .into_iter()
            .chain(gallery_west(reaches))
            .collect();
        let inside = |at: Vec3| boxes.iter().any(|box_| box_.contains_point(at));

        for step in 0..40 {
            let along = (step as f32 + 0.5) / 40.0;
            let z = from + (to - from) * along;
            let x = west + (east - west) * along;

            for up in [0.2f32, 1.0, LOBBY_TALL * 0.5, LOBBY_TALL - 0.1] {
                // the west wall, which the register's door is cut into
                let in_the_door = z > door_from && z < door_to && up < DOOR_TALL;

                if !in_the_door {
                    assert!(
                        inside(vec3(west - THICK * 0.5, up, z)),
                        "the lobby's west wall is open at {:.2} along, {:.2} up",
                        z,
                        up
                    );
                }

                // the east wall, which the gallery's doorway is cut into and
                // which runs the full height of the room
                if !(z > way_from && z < way_to) {
                    assert!(
                        inside(vec3(east + THICK * 0.5, up, z)),
                        "the lobby's east wall is open at {:.2} along, {:.2} up",
                        z,
                        up
                    );
                }

                // and the two with nothing in them at all
                assert!(
                    inside(vec3(x, up, from - THICK * 0.5)),
                    "the lobby's far wall is open at {:.2} along, {:.2} up",
                    x,
                    up
                );
                assert!(
                    inside(vec3(x, up, to + THICK * 0.5)),
                    "the lobby's north wall is open at {:.2} along, {:.2} up",
                    x,
                    up
                );
            }
        }
    }

    /// Spec 0062: you can walk in from the gallery and reach the head of the
    /// page.
    ///
    /// **Three legs, because one straight line is not the walk.** Spec 0029
    /// had you turn out of the corridor through a door in its side wall;
    /// this goes through the gallery's doorway, across the lobby, through
    /// the register's door and west along the book, which is the route the
    /// room is now for. The diagonal starts at the far end of the page, so a
    /// walk that stops at the threshold is a room with nothing in it.
    ///
    /// The whole building's boxes, not this room's: a walk that only knows
    /// its own room walks through the wall it shares.
    #[test]
    fn walked_from_the_gallery_into_the_register() {
        let room = Room::of(crate::games());
        let reaches = room.reaches;
        let solid = room.solid();
        let (_, east) = span();
        let middle = middle_z(reaches);
        let (page_west, page_east) = crate::register::along(reaches);
        let legs = [
            // out of the gallery's doorway into the lobby
            vec3(east - 1.0, 0.0, middle),
            // across it to the register's door
            vec3(span().0 + 0.5, 0.0, middle),
            // through the door
            vec3(page_east - 1.0, 0.0, middle),
            // and west to the head of the page
            vec3(page_west + 0.6, 0.0, middle),
        ];
        let mut at = vec3(east + 0.6, 0.0, middle);

        for (n, leg) in legs.iter().copied().enumerate() {
            at = walked(&solid, at, leg, 12.0);

            let short = vec3(leg.x - at.x, 0.0, leg.z - at.z).length();

            assert!(
                short < 0.6,
                "leg {} of the walk in stopped {:.2} short of {:?}, at {:?}",
                n,
                short,
                leg,
                at
            );
        }

        // and from there the diagonal has a full page to be read off
        assert_eq!(
            crate::register::walked(reaches, at),
            0,
            "the walk ends somewhere other than the head of the page"
        );
    }

    /// Drives a body your size from one place to another with the game's own
    /// walk, and says where it ended up.
    ///
    /// The whole building's boxes, not this room's. A walk that only knows
    /// its own room walks through the wall it shares.
    fn walked(solid: &[Aabb], from: Vec3, to: Vec3, seconds: f32) -> Vec3 {
        let step = 1.0 / 60.0;
        let mut at = from;
        let mut falling = 0.0;

        for _ in 0..(seconds / step) as u32 {
            let way = vec3(to.x - at.x, 0.0, to.z - at.z);
            let wish = if way.length_squared() > 1e-4 {
                way.normalize() * crate::SPEED
            } else {
                Vec3::ZERO
            };

            let (moved, fell) = walk::walk(at, wish, falling, crate::RADIUS, step, solid, true);

            at = moved;
            falling = fell;
        }

        at
    }
}
