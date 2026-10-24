// Canary desk enclosure, in two printed parts.
//
// Side profile is a 75 degree angle: a screen arm leaning back 15 degrees and a
// foot running back along the desk. The shell (part 1) is open on the two
// inner faces of that angle; the cover (part 2) closes them. The screen screws
// to the cover through the board's own M2 inserts, the X6 bolts to it with
// nuts, its gas inlet facing out through a window.
//
// World axes: X across (right is +X seen from the front), Y depth (front 0,
// back +), Z up from the desk. Screen-arm frame: s up the face, n into the
// body, both from the front-bottom edge.
//
// Render one part with:  openscad -D 'part="shell"' -o shell.stl canary.scad
// Parts: "assembly", "shell", "cover".
// Reference shapes for previews: "screen_ghost", "pcb_ghost", "x6_ghost".

part = "assembly";

/* [Overall] */
tilt = 15;          // screen lean back from vertical, degrees
width = 58;
arm_len = 72;       // screen arm, along the face
foot_len = 60;      // foot, front to back
foot_t = 16;        // foot height
wall = 2;
cover_t = 2.4;
edge_r = 3;         // rounding on every outer edge
face_r = 9;         // corner radius of the screen face
fit = 0.2;          // cover-to-shell clearance per side
bezel = 1.6;

/* [Screen board] */
screen_s = 42;          // screen centre along the face
aperture_d = 46.5;      // active area is 45.68
chamfer = 1.6;
pcb_d = 47.86;
board_edge = 4.5;       // glass front to PCB back, measured with calipers
back_parts = 4.5;       // tallest part on the PCB back (connectors), estimated
pcb_front_n = bezel + board_edge - 1.6;
pcb_t = 1.6;

// Screen arm thickness, normal to the face: bezel, board, its back parts,
// 1 mm of air, then the cover.
arm_t = bezel + board_edge + back_parts + 1 + cover_t;
// The board's M2 inserts, seen from the front, relative to the screen centre
// (x, s). Read off the drawing to about 0.5 mm; check with calipers.
board_holes = [[16.5, 14.5], [16.5, -14.6], [-22.5, 0]];

/* [Sensor] */
sensor = [25.5, 23];            // X6 board, x by y
sensor_y = 41;                  // centre of the X6 along Y
sensor_holes = [[-11, -9.75], [11, 9.75]];  // the two 2.2 mm holes, diagonal, from its centre
standoff = 5.5;                 // cover underside to X6 board top: inlet sits just under the window
window = 13;                    // square window over the gas element

/* [Power] */
usbc = [8.94, 3.26];    // USB-C receptacle shell, width by height
usbc_fit = 0.15;        // clearance per side for gluing
usbc_x = -14;           // seen from the front
usbc_z = 6;             // centre height
usbc_sleeve = 5;        // sleeve depth behind the back wall

/* [Vents] */
arm_vent_s = [30, 58];  // span of the vertical vents along the face

/* [Fasteners] */
insert_d = 3.2;     // M2 heat-set insert
m2_clear = 2.4;
m2_head = 4.2;

$fn = 48;
eps = 0.01;

T = tilt;

// screen-arm frame: x = x, y = n, z = s, then lean back about X
module arm_frame() { rotate([-T, 0, 0]) children(); }

// Y of the screen arm's back face at height z
function back_y(z) = arm_t / cos(T) + z * tan(T);

module z_above(z0) { translate([-100, -100, z0]) cube([200, 300, 200]); }

// ---------------------------------------------------------------- envelope

// Rounded box in the arm frame; face corners get face_r, other edges edge_r.
module arm_body() {
    arm_frame() minkowski() {
        translate([0, edge_r, 0]) rotate([-90, 0, 0]) linear_extrude(arm_t - 2 * edge_r)
            translate([0, -(arm_len - 8) / 2]) offset(face_r - edge_r)
                square([width - 2 * face_r, arm_len + 8 - 2 * face_r], center = true);
        sphere(r = edge_r, $fn = 24);
    }
}

module foot_body() {
    minkowski() {
        translate([-(width / 2 - edge_r), edge_r, -10]) cube([width - 2 * edge_r, foot_len - 2 * edge_r, 10 + foot_t - edge_r]);
        sphere(r = edge_r, $fn = 24);
    }
}

module envelope() {
    intersection() {
        union() {
            arm_body();
            // kept behind the face so the screen arm runs flush down to the desk
            intersection() { foot_body(); arm_frame() translate([-100, edge_r, -50]) cube([200, 200, 300]); }
        }
        z_above(0);
    }
}

// ---------------------------------------------------------------- openings

// The screen arm's outline seen face-on, inset by the wall and `g`, in the
// 2D plane that arm_frame() + rotate([-90, 0, 0]) maps onto (x, s).
module arm_inner_outline(g = 0) {
    translate([0, -(arm_len - 8) / 2]) offset(face_r - wall - g)
        square([width - 2 * face_r, arm_len + 8 - 2 * face_r], center = true);
}

// The cover's region, grown by `g` on its inner edges.
module cover_region(g = 0, open = 0) {
    lat = width / 2 - wall - g;
    intersection() {
        union() {
            // screen-arm segment
            intersection() {
                arm_frame() translate([0, arm_t - cover_t - g, 0]) rotate([-90, 0, 0]) linear_extrude(cover_t + g + open)
                    arm_inner_outline(g);
                z_above(foot_t - cover_t - g);
            }
            // foot segment
            difference() {
                translate([-lat, 0, foot_t - cover_t - g]) cube([2 * lat, foot_len - wall - g, cover_t + g + open]);
                arm_frame() translate([-100, -100, -50]) cube([200, 100 + arm_t - cover_t - g, 300]);
            }
        }
        z_above(0);
    }
}

module screen_cavity() {
    intersection() {
        arm_frame() translate([0, bezel, 0]) rotate([-90, 0, 0]) linear_extrude(arm_t - cover_t - bezel + eps) arm_inner_outline(0);
        z_above(wall);
    }
}

module foot_cavity() {
    difference() {
        translate([-(width / 2 - wall), 0, wall]) cube([width - 2 * wall, foot_len - wall, foot_t]);
        arm_frame() translate([-100, -100, -50]) cube([200, 100 + arm_t, 300]);
    }
}

module aperture() {
    arm_frame() translate([0, 0, screen_s]) rotate([-90, 0, 0]) {
        translate([0, 0, -1]) cylinder(d = aperture_d, h = bezel + 2, $fn = 96);
        translate([0, 0, -eps]) cylinder(d1 = aperture_d + 2 * chamfer, d2 = aperture_d, h = chamfer, $fn = 96);
        translate([0, 0, -3]) cylinder(d = aperture_d + 2 * chamfer + 2, h = 3, $fn = 96);
    }
}

// Power in: a female USB-C receptacle glued into a fitted sleeve in the back of the foot.
module usbc_stadium(len) {
    r = (usbc[1] + 2 * usbc_fit) / 2;
    hull() for (x = [-1, 1]) translate([x * (usbc[0] / 2 + usbc_fit - r), 0, 0]) rotate([-90, 0, 0]) cylinder(r = r, h = len);
}

module usbc_sleeve() {
    translate([usbc_x - 7.5, foot_len - wall - usbc_sleeve, wall - eps]) cube([15, usbc_sleeve + eps, usbc_z + 4.5 - wall]);
}

module usbc_hole() {
    translate([usbc_x, foot_len - wall - usbc_sleeve - 1, usbc_z]) usbc_stadium(usbc_sleeve + wall + 2);
}

// USB-C points down out of the board; plug in from below to flash.
module usb_slot() {
    translate([-6.5, 3, -1]) cube([13, 10, wall + 2]);
}

// Shell bosses for the cover screws: two near the top of the arm, two at the back of the foot.
arm_bosses = [[width / 2 - wall - 2.6, arm_len - 8], [-(width / 2 - wall - 2.6), arm_len - 8]];
foot_bosses = [[width / 2 - wall - 2.6, foot_len - 8], [-(width / 2 - wall - 2.6), foot_len - 8]];
boss_h = 6;

module shell_bosses() {
    for (p = arm_bosses) arm_frame() translate([p[0], arm_t - cover_t - fit - boss_h, p[1]]) rotate([-90, 0, 0])
        difference() { cylinder(d = 5.4, h = boss_h); translate([0, 0, 2]) cylinder(d = insert_d, h = boss_h); }
    for (p = foot_bosses) translate([p[0], p[1], wall - eps]) difference() {
        cylinder(d = 5.4, h = foot_t - cover_t - fit - wall + eps);
        translate([0, 0, 2]) cylinder(d = insert_d, h = foot_t);
    }
}

// ---------------------------------------------------------------- part 1

module shell() {
    difference() {
        union() {
            difference() {
                envelope();
                screen_cavity();
                foot_cavity();
                cover_region(fit, 10);
            }
            intersection() { union() { shell_bosses(); usbc_sleeve(); } envelope(); }
        }
        aperture();
        usb_slot();
        // floor vents under the sensor
        for (x = [-10 : 4 : 10]) translate([x - 0.8, sensor_y - 10, -1]) cube([1.6, 20, wall + 2]);
        usbc_hole();
        // power and sensor leads through the divider to the board
        intersection() {
            translate([10, 0, wall + 2]) cube([8, 40, 3.5]);
            arm_frame() translate([-100, arm_t - cover_t - 1, -50]) cube([200, cover_t + 2, 300]);
        }
        // rubber feet
        for (x = [-width / 2 + 8, width / 2 - 8], y = [10, foot_len - 8])
            translate([x, y, -eps]) cylinder(d = 8.5, h = 0.8);
    }
}

// ---------------------------------------------------------------- part 2

module cover() {
    difference() {
        union() {
            intersection() { cover_region(0); envelope(); }
            // posts that hold the screen: M2 screws from outside into the board's inserts
            for (h = board_holes) arm_frame() translate([h[0], 0, screen_s + h[1]]) rotate([-90, 0, 0])
                translate([0, 0, pcb_front_n + pcb_t]) cylinder(d = 5, h = arm_t - cover_t - pcb_front_n - pcb_t + eps);
            // X6 standoffs under the foot segment
            for (h = sensor_holes) translate([h[0], sensor_y + h[1], foot_t - cover_t - standoff])
                cylinder(d = 4.6, h = standoff + eps);
        }
        for (h = board_holes) arm_frame() translate([h[0], 0, screen_s + h[1]]) rotate([-90, 0, 0]) {
            translate([0, 0, pcb_front_n]) cylinder(d = m2_clear, h = arm_t);
            translate([0, 0, arm_t - 1.2]) cylinder(d1 = m2_clear, d2 = m2_head + 0.4, h = 1.2 + eps);
        }
        for (h = sensor_holes) translate([h[0], sensor_y + h[1], 0]) {
            cylinder(d = m2_clear, h = 40);
            translate([0, 0, foot_t - 1.2]) cylinder(d1 = m2_clear, d2 = m2_head + 0.4, h = 1.2 + eps);
        }
        for (p = arm_bosses) arm_frame() translate([p[0], arm_t - cover_t - 1, p[1]]) rotate([-90, 0, 0]) {
            cylinder(d = m2_clear, h = cover_t + 2);
            translate([0, 0, cover_t + 1 - 1.2]) cylinder(d1 = m2_clear, d2 = m2_head + 0.4, h = 1.2 + eps);
        }
        for (p = foot_bosses) translate([p[0], p[1], foot_t - cover_t - 1]) {
            cylinder(d = m2_clear, h = cover_t + 2);
            translate([0, 0, cover_t + 1 - 1.2]) cylinder(d1 = m2_clear, d2 = m2_head + 0.4, h = 1.2 + eps);
        }
        // window over the gas element
        translate([-window / 2, sensor_y - window / 2, foot_t - cover_t - 1]) cube([window, window, cover_t + 2]);
        // foot vents, either side of the window
        for (x = [-19, -15, 15, 19]) translate([x - 0.8, sensor_y - 9, foot_t - cover_t - 1]) cube([1.6, 18, cover_t + 2]);
        // vertical arm vents, centred: cool air in low, board heat out high
        for (x = [-7.5, -2.5, 2.5, 7.5]) arm_frame() hull() for (s = arm_vent_s)
            translate([x, arm_t - cover_t - 1, s]) rotate([-90, 0, 0]) cylinder(d = 2, h = cover_t + 2);
    }
}

// ---------------------------------------------------------------- reference

module screen_ghost() {
    arm_frame() translate([0, bezel, screen_s]) rotate([-90, 0, 0]) cylinder(d = 48.08, h = pcb_front_n - bezel, $fn = 96);
}

module pcb_ghost() {
    arm_frame() translate([0, pcb_front_n, screen_s]) rotate([-90, 0, 0]) cylinder(d = pcb_d, h = pcb_t, $fn = 96);
    translate([-sensor[0] / 2, sensor_y - sensor[1] / 2, foot_t - cover_t - standoff - 1.2]) cube([sensor[0], sensor[1], 1.2]);
}

module x6_ghost() {
    translate([-5.75, sensor_y - 5.75, foot_t - cover_t - standoff]) difference() {
        cube([11.5, 11.5, 7.6]);
        translate([5.75, 5.75, 7]) cylinder(d = 6, h = 1);
    }
}

if (part == "shell") shell();
else if (part == "cover") cover();
else if (part == "screen_ghost") screen_ghost();
else if (part == "pcb_ghost") pcb_ghost();
else if (part == "x6_ghost") x6_ghost();
else {
    color("#e8e6df") shell();
    color("#d7d4ca") cover();
    color("#2c3a44") screen_ghost();
    color("#3f7a52") pcb_ghost();
    color("#555") x6_ghost();
}
