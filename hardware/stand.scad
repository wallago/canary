// canary desk stand: round LCD board on a tilted face, sensor pocketed behind it.
//
// Units are mm. Nothing downstream is hard-coded -- every number lives in the
// parameter blocks below, so re-fitting the part is an edit here and a re-render.
//
//   render:  openscad -o stand.stl hardware/stand.scad
//   preview: openscad hardware/stand.scad

/* [Board -- MEASURE THESE, the defaults are invented] */

// Waveshare ESP32-S3-Touch-LCD-1.85C. Waveshare publishes the outline drawing
// as an image only, so none of these came from a datasheet. Calipers first.
board_d          = 42.0;  // PCB outside diameter
board_t          = 1.6;   // PCB thickness -- unused by the geometry, kept so the
                          // screw length below can be worked out from one place
board_back_clear = 6.0;   // tallest thing on the back: module, MX1.25, IPEX
board_bolt_r     = 17.0;  // bolt-circle radius, board centre to hole centre
board_bolt_a     = [45, 135, 225, 315];  // hole angles, degrees CCW from local +X
usb_angle        = 270;   // bearing of the USB-C port, same angle convention

/* [Sensor -- MEASURE THESE, the defaults are invented] */

// The 9600-baud UART sensor. The firmware only logs its frames raw, so the
// part is unidentified and all four of these are placeholders.
sensor_w      = 40.0;  // across the stand's width
sensor_h      = 30.0;  // up the stand's face
sensor_d      = 15.0;  // into the stand
sensor_boss_h = 4.0;   // how far the sensor stands off the pocket floor
sensor_holes  = [[-15, -10], [15, -10], [-15, 10], [15, 10]];  // from pocket centre

/* [Stand] */

tilt        = 15;   // degrees the face leans back from vertical
wall        = 2.4;  // nominal wall, ~6 perimeters at 0.4mm
fit         = 0.4;  // clearance added around both parts
face_margin = 4.0;  // plastic left around the board on the face
base_t      = 4.0;  // desk plate thickness
base_lip    = 5.0;  // how far the plate reaches past the body

/* [Heat-set inserts] */

// Sized for a ruthex RX-M2x4 or equivalent. Brands differ in knurl diameter and
// length, so check your own before printing: a bore 0.2mm out is the difference
// between a flush insert and a bulged boss.
insert_bore   = 3.2;  // bore the insert melts into
insert_len    = 4.0;  // insert length
insert_relief = 1.5;  // extra depth so displaced plastic has somewhere to go
boss_wall     = 1.6;  // minimum plastic around the bore

/* [Cable route] */

cable_w = 12.0;  // slot from the board pocket through to the sensor pocket
cable_h = 6.0;

/* [Hidden] */

$fn = 96;
eps = 0.01;

// -- derived ---------------------------------------------------------------

// A boss is the bore plus its surrounding wall; the guidance floor is 5mm OD
// for M2, which 3.2 + 2*1.6 clears.
boss_d     = insert_bore + 2 * boss_wall;
bore_depth = insert_len + insert_relief;
boss_embed = 1.0;  // how far a boss sinks into its own pocket floor

// Pocket openings, board and sensor plus their running clearance.
board_pocket_d  = board_d + 2 * fit;
sensor_pocket_w = sensor_w + 2 * fit;
sensor_pocket_h = sensor_h + 2 * fit;
sensor_pocket_d = sensor_d + sensor_boss_h;

// Material between the two pocket floors. The sensor bosses are short and bore
// *towards* the board pocket, so the divider has to absorb whatever the bore
// overshoots the boss by, and still leave boss_wall behind it.
divider = max(wall, bore_depth + boss_wall - sensor_boss_h);

// The slab: wide enough for whichever part is wider, long enough for the
// board, and thick enough to stack both pockets back to back. The sensor
// pocket opens flush at the back face, so nothing is budgeted behind it.
face_w   = max(board_d, sensor_w) + 2 * fit + 2 * wall + 2 * face_margin;
face_len = board_d + 2 * fit + 2 * wall + 2 * face_margin;
body_d   = board_back_clear + divider + sensor_pocket_d;

// Board centre in face coordinates, and the sensor sharing that axis.
board_cx = face_w / 2;
board_cy = face_len / 2;

// Pocket floors, as local z. The sensor's is the far side of its pocket: the
// pocket opens at the back face (-body_d) and runs inwards, so its floor is
// the most positive z of the cut, not the least.
board_floor  = -board_back_clear;
sensor_floor = -body_d + sensor_pocket_d;

// How far back the slab's top-rear corner reaches, so the plate can outrun it.
slab_back = face_len * sin(tilt) + body_d * cos(tilt);

// Every hole as an explicit [x, y] pair, so an irregular real-world pattern
// just replaces the list rather than fighting the bolt-circle assumption.
function bolt_circle(r, angles) = [for (t = angles) [r * cos(t), r * sin(t)]];
board_holes = bolt_circle(board_bolt_r, board_bolt_a);

// -- frame -----------------------------------------------------------------

// Face coordinates: origin at the bottom front edge of the slab, local +Y runs
// up the face, local +Z points out of it, local +X is the stand's width.
// rotate([90 - tilt]) sends local +Y to (sin t, cos t) and local +Z to
// (-cos t, sin t), which is exactly the leaning face and its outward normal.
module on_face() {
  translate([0, 0, base_t]) rotate([90 - tilt, 0, 0]) children();
}

// -- solids ----------------------------------------------------------------

// The leaning slab, front face at local z = 0, back face at local z = -body_d.
module slab() {
  on_face() translate([0, 0, -body_d]) cube([face_w, face_len, body_d]);
}

// Desk plate, reaching past the slab at both ends so the lean has something
// to sit on rather than tipping onto its front edge.
module base() {
  translate([0, -base_lip, 0])
    cube([face_w, slab_back + 2 * base_lip, base_t]);
}

// Pillars the board lands on, standing from the pocket floor up to the face.
// The insert goes in from the front, flush with the boss top.
module board_bosses() {
  for (h = board_holes)
    translate([board_cx + h[0], board_cy + h[1], board_floor - boss_embed])
      cylinder(d = boss_d, h = board_back_clear + boss_embed);
}

// Same idea behind, but rising off the sensor pocket floor toward the back.
module sensor_bosses() {
  for (h = sensor_holes)
    translate([board_cx + h[0], board_cy + h[1], sensor_floor - sensor_boss_h])
      cylinder(d = boss_d, h = sensor_boss_h + boss_embed);
}

// -- cuts ------------------------------------------------------------------

// Clearance pocket for everything sticking off the back of the PCB.
module board_pocket() {
  translate([board_cx, board_cy, board_floor])
    cylinder(d = board_pocket_d, h = board_back_clear + eps);
}

// Open to the back, so the sensor drops in from behind and breathes.
module sensor_pocket() {
  translate([board_cx - sensor_pocket_w / 2,
             board_cy - sensor_pocket_h / 2,
             -body_d - eps])
    cube([sensor_pocket_w, sensor_pocket_h, sensor_pocket_d + eps]);
}

// Blind bores for the inserts, each sunk from its own boss top. Straight-sided,
// not tapered: the insert cuts its own grip as it melts in.
module insert_bores() {
  for (h = board_holes)
    translate([board_cx + h[0], board_cy + h[1], -bore_depth])
      cylinder(d = insert_bore, h = bore_depth + eps);
  for (h = sensor_holes)
    translate([board_cx + h[0], board_cy + h[1], sensor_floor - sensor_boss_h - eps])
      cylinder(d = insert_bore, h = bore_depth + eps);
}

// Slot through the divider, for the UART pigtail.
module cable_slot() {
  translate([board_cx - cable_w / 2,
             board_cy - cable_h / 2,
             sensor_floor - eps])
    cube([cable_w, cable_h, divider + 2 * eps]);
}

// Notch out to the rim so the USB-C plug clears the pocket wall.
module usb_notch() {
  translate([board_cx, board_cy, board_floor])
    rotate([0, 0, usb_angle])
      translate([0, -cable_w / 2, 0])
        cube([face_len, cable_w, board_back_clear + eps]);
}

// -- assembly --------------------------------------------------------------

module stand() {
  // Trimmed at the desk: the slab's rear bottom corner leans below z = 0 by
  // design, and the intersection turns that overshoot into a flat footprint.
  intersection() {
    translate([-face_w, -face_len, 0]) cube([3 * face_w, 3 * face_len, 3 * body_d + face_len]);

    difference() {
      union() {
        // Pockets are hollowed out first; the bosses go in afterwards, or the
        // pocket cut would take them with it -- they stand inside its walls.
        difference() {
          union() {
            slab();
            base();
          }
          on_face() board_pocket();
          on_face() sensor_pocket();
          on_face() cable_slot();
          on_face() usb_notch();
        }
        on_face() board_bosses();
        on_face() sensor_bosses();
      }
      on_face() insert_bores();
    }
  }
}

stand();
