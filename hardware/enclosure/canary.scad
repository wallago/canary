// Canary desk enclosure, in three printed parts.
//
// Side profile is a 75 degree angle: a screen arm leaning back 15 degrees and a
// foot running back along the desk. The shell is open on the two inner faces
// of that angle; two covers close them. The screen screws to the screen cover
// through the board's own brass M2 standoffs, which rest on the cover. The X6
// bolts to the sensor cover with M2 nuts, its gas inlet facing out through a
// window and its connector toward the screen.
//
// World axes: X across (right is +X seen from the front), Y depth (front 0,
// back +), Z up from the desk. Screen-arm frame: s up the face, n into the
// body, both from the front-bottom edge.
//
// Render one part with:  openscad -D 'part="shell"' -o shell.stl canary.scad
// Parts: "assembly", "shell", "screen_cover", "sensor_cover".
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
standoff_len = 4;       // the board's brass M2 standoffs, PCB back to tip, measured
pcb_front_n = bezel + board_edge - 1.6;
pcb_t = 1.6;

// Screen arm thickness, normal to the face: bezel, board, its brass
// standoffs (their tips rest on the cover), then the cover.
arm_t = bezel + board_edge + standoff_len + cover_t;
// The board's M2 inserts, seen from the front, relative to the screen centre
// (x, s). Read off the drawing; the single hole moved 1.5 mm in after a test fit.
board_holes = [[16.5, 14.5], [16.5, -14.6], [-21, 0]];

/* [Sensor] */
// X6 frame, as on its drawing's top view: x along the 25.5 side with the
// connector at +x, y along the 23 side, from the board centre. The connector
// points at the screen, so +x is world -Y.
sensor = [25.5, 23];
sensor_y = 40.5;                // centre of the X6 along Y
vent_y = 41;                    // centre of the foot vents along Y
sensor_holes = [[11, -9.5], [11, 9.5]];     // the two 2.2 mm (M2) holes, on the connector edge
sensor_inlet = [-5, 0];         // gas element centre, from a test fit
sensor_conn = [8.25, 12.75, 11, 4.85];      // connector: x from, x to, width, height
standoff = 4;                   // cover underside to X6 board top
window = 13;                    // square window over the gas element
plug_len = 8;                   // room for the plug beyond the board edge

/* [Power] */
// Power comes in through a red USB-C breakout lying on the floor under the
// X6, its receptacle facing out of the back wall. Breakout frame: x across,
// y from the receptacle edge of its PCB into the foot, z up from the floor.
breakout = [21.6, 12.7, 1.6];       // PCB: along x, along y, thickness
breakout_holes = [[-7.5, 3.3], [7.5, 3.3]];     // its two holes, read off a photo; check with calipers
usbc = [8.94, 3.26];    // receptacle shell, width by height, on top of the PCB
usbc_fit = 0.3;         // clearance per side in the back wall
usbc_overhang = 0.8;    // receptacle front past the PCB edge, estimated
plug_recess = [13, 7.5, 1.2];       // outside pocket so the plug's overmould reaches the receptacle
solder_relief = 0.6;    // floor pocket under the PCB for solder joints

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

// X6 frame on the foot: origin at the board centre, z = 0 on the board's top
// face, which faces up into the cover.
module x6_frame() { translate([0, sensor_y, foot_t - cover_t - standoff]) rotate([0, 0, -90]) children(); }

// The two covers meet at the back face of the screen arm.
module sensor_side() { intersection() { arm_frame() translate([-100, arm_t + fit, -50]) cube([200, 200, 300]); z_above(0); } }
module screen_side() { arm_frame() translate([-100, -100, -50]) cube([200, 100 + arm_t, 300]); }

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

// Runs into the screen cavity: no wall between the board and the sensor.
module foot_cavity() {
    difference() {
        translate([-(width / 2 - wall), 0, wall]) cube([width - 2 * wall, foot_len - wall, foot_t]);
        arm_frame() translate([-100, -100, -50]) cube([200, 100 + arm_t - cover_t - eps, 300]);
    }
}

module aperture() {
    arm_frame() translate([0, 0, screen_s]) rotate([-90, 0, 0]) {
        translate([0, 0, -1]) cylinder(d = aperture_d, h = bezel + 2, $fn = 96);
        translate([0, 0, -eps]) cylinder(d1 = aperture_d + 2 * chamfer, d2 = aperture_d, h = chamfer, $fn = 96);
        translate([0, 0, -3]) cylinder(d = aperture_d + 2 * chamfer + 2, h = 3, $fn = 96);
    }
}

// Power in: the breakout's receptacle edge sits against the inside of the back wall.
module breakout_frame() { translate([0, foot_len - wall - fit, wall]) mirror([0, 1, 0]) children(); }

module stadium(w, h, len) {
    r = h / 2;
    hull() for (x = [-1, 1]) translate([x * (w / 2 - r), 0, 0]) rotate([90, 0, 0]) cylinder(r = r, h = len);
}

module usbc_port() {
    zc = wall + breakout[2] + usbc[1] / 2;
    translate([0, foot_len + 1, zc]) {
        stadium(usbc[0] + 2 * usbc_fit, usbc[1] + 2 * usbc_fit, wall + fit + 2);
        stadium(plug_recess[0], plug_recess[1], 1 + plug_recess[2]);
    }
}

// The breakout rests on the floor, held by two M2 bolts from below with nuts on top.
module breakout_seat() {
    breakout_frame() {
        difference() {
            translate([-breakout[0] / 2 - fit, -fit, -solder_relief]) cube([breakout[0] + 2 * fit, breakout[1] + 2 * fit, solder_relief + eps]);
            for (h = breakout_holes) translate([h[0], h[1], -1]) cylinder(d = 5, h = 2);
        }
        for (h = breakout_holes) translate([h[0], h[1], -wall - 1]) {
            cylinder(d = m2_clear, h = wall + 2);
            cylinder(d1 = m2_head + 0.4, d2 = m2_clear, h = 1 + 1.2);
        }
    }
}

// Shell bosses for the cover screws: four for each cover, at its corners.
boss_x = width / 2 - wall - 2.6;
arm_bosses = [for (x = [-boss_x, boss_x], s = [22, arm_len - 8]) [x, s]];
foot_bosses = [for (x = [-boss_x, boss_x], y = [22, foot_len - 8]) [x, y]];
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
            intersection() { shell_bosses(); envelope(); }
        }
        aperture();
        // floor vents under the sensor, clear of the breakout
        for (x = [-10 : 4 : 10]) translate([x - 0.8, vent_y - 10, -1]) cube([1.6, 13, wall + 2]);
        usbc_port();
        breakout_seat();
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
            // X6 standoffs under the foot segment
            x6_frame() for (h = sensor_holes) translate([h[0], h[1], 0]) cylinder(d = 4.6, h = standoff + eps);
        }
        // M2 screws from outside straight into the screen board's brass standoffs
        for (h = board_holes) arm_frame() translate([h[0], 0, screen_s + h[1]]) rotate([-90, 0, 0]) {
            translate([0, 0, arm_t - cover_t - 1]) cylinder(d = m2_clear, h = cover_t + 2);
            translate([0, 0, arm_t - 1.2]) cylinder(d1 = m2_clear, d2 = m2_head + 0.4, h = 1.2 + eps);
        }
        x6_frame() for (h = sensor_holes) translate([h[0], h[1], -1]) {
            cylinder(d = m2_clear, h = 40);
            translate([0, 0, 1 + standoff + cover_t - 1.2]) cylinder(d1 = m2_clear, d2 = m2_head + 0.4, h = 1.2 + eps);
        }
        for (p = arm_bosses) arm_frame() translate([p[0], arm_t - cover_t - 1, p[1]]) rotate([-90, 0, 0]) {
            cylinder(d = m2_clear, h = cover_t + 2);
            translate([0, 0, cover_t + 1 - 1.2]) cylinder(d1 = m2_clear, d2 = m2_head + 0.4, h = 1.2 + eps);
        }
        for (p = foot_bosses) translate([p[0], p[1], foot_t - cover_t - 1]) {
            cylinder(d = m2_clear, h = cover_t + 2);
            translate([0, 0, cover_t + 1 - 1.2]) cylinder(d1 = m2_clear, d2 = m2_head + 0.4, h = 1.2 + eps);
        }
        x6_frame() {
            // window over the gas element
            translate([sensor_inlet[0] - window / 2, -window / 2, standoff - 1]) cube([window, window, cover_t + 2]);
            // the connector is taller than the standoffs: a blind pocket under the cover for it and the plug
            translate([sensor_conn[0] - 0.5, -(sensor_conn[2] + 1) / 2, standoff - 1])
                cube([sensor_conn[1] - sensor_conn[0] + plug_len + 0.5, sensor_conn[2] + 1, sensor_conn[3] + 0.3 - standoff + 1]);
        }
        // foot vents, either side of the window
        for (x = [-19, -15, 15, 19]) translate([x - 0.8, vent_y - 9, foot_t - cover_t - 1]) cube([1.6, 18, cover_t + 2]);
        // vertical arm vents, centred: cool air in low, board heat out high
        for (x = [-7.5, -2.5, 2.5, 7.5]) arm_frame() hull() for (s = arm_vent_s)
            translate([x, arm_t - cover_t - 1, s]) rotate([-90, 0, 0]) cylinder(d = 2, h = cover_t + 2);
    }
}

module screen_cover() { intersection() { cover(); screen_side(); } }
module sensor_cover() { intersection() { cover(); sensor_side(); } }

// ---------------------------------------------------------------- reference

module screen_ghost() {
    arm_frame() translate([0, bezel, screen_s]) rotate([-90, 0, 0]) cylinder(d = 48.08, h = pcb_front_n - bezel, $fn = 96);
}

module pcb_ghost() {
    arm_frame() translate([0, pcb_front_n, screen_s]) rotate([-90, 0, 0]) cylinder(d = pcb_d, h = pcb_t, $fn = 96);
    x6_frame() translate([-sensor[0] / 2, -sensor[1] / 2, -1.2]) cube([sensor[0], sensor[1], 1.2]);
    breakout_frame() {
        translate([-breakout[0] / 2, 0, 0]) cube(breakout);
        translate([-usbc[0] / 2, -usbc_overhang, breakout[2]]) cube([usbc[0], 7.4, usbc[1]]);
    }
}

module x6_ghost() {
    x6_frame() {
        translate([sensor_inlet[0] - 5.75, -5.75, 0]) difference() {
            cube([11.5, 11.5, 7.6]);
            translate([5.75, 5.75, 7]) cylinder(d = 6, h = 1);
        }
        translate([sensor_conn[0], -sensor_conn[2] / 2, 0])
            cube([sensor_conn[1] - sensor_conn[0], sensor_conn[2], sensor_conn[3]]);
    }
}

if (part == "shell") shell();
else if (part == "screen_cover") screen_cover();
else if (part == "sensor_cover") sensor_cover();
else if (part == "screen_ghost") screen_ghost();
else if (part == "pcb_ghost") pcb_ghost();
else if (part == "x6_ghost") x6_ghost();
else {
    color("#e8e6df") shell();
    color("#d7d4ca") screen_cover();
    color("#cfcabd") sensor_cover();
    color("#2c3a44") screen_ghost();
    color("#3f7a52") pcb_ghost();
    color("#555") x6_ghost();
}
