# Enclosure

A 3D-printed desk stand for Canary, in four parts. Seen from the side it is
a 75° angle: the screen arm leans back 15° and a foot runs back along the desk.
The shell is open on the two inner faces of that angle, and two covers close
them: one behind the screen, one over the foot. The screen mounts on the
first, the air sensor on the second. The shell prints as two halves that
screw together from underneath.

Everything is in [`canary.scad`](canary.scad). Every dimension is a named
variable at the top of the file. Units are millimetres.

## Hardware

- [Waveshare ESP32-S3-Touch-LCD-1.85](https://www.waveshare.com/esp32-s3-touch-lcd-1.85.htm),
  plain version (not the C)
- [Waveshare Environment X6](https://www.waveshare.com/environment-x6-sensor.htm)

## Parts

| Part           | What it is                                                                                                                  |
| -------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `shell_arm`    | Front half of the body: screen opening, two blocks reaching back over the foot floor for the joint screws                   |
| `shell_foot`   | Back half of the body: USB-C port at the back, seat for the USB-C breakout, floor vents under the sensor                    |
| `screen_cover` | Plate behind the screen: three screw holes for the board, vertical vents                                                    |
| `sensor_cover` | Plate over the foot: standoffs for the X6, window for its gas element, pocket for its connector, vents                      |

Print `shell_arm` face-down and `shell_foot` floor-down; neither needs
support.

Export the STLs with `just stl`. They land in `target/enclosure/`.
To export one part by hand:

```sh
nix run nixpkgs#openscad -- -D 'part="shell_arm"' -o shell_arm.stl hardware/enclosure/canary.scad
```

Open the file without `-D` to see the assembly, with the two boards shown as
reference shapes.

## Assembly

- The screen's brass M2 standoffs rest on the screen cover, and three screws
  go from outside the cover straight into them. The glass sits 1 mm behind
  the face, so the face never presses on the panel.
- The X6 hangs under the sensor cover on 4 mm standoffs, component side up,
  held by M2 bolts and nuts through its two 2.2 mm holes on the connector
  edge. Its connector points at the screen and sits in a pocket under the
  cover. The gas element comes up through the window.
- Each cover screws to the shell with four M2 screws into heat-set inserts.
- The shell halves join with two M2 screws from underneath, up through the
  foot floor into heat-set inserts in the arm half's blocks. Fit those
  inserts before joining the halves.
- Inside, the screen arm and the foot are one open space, with no wall
  between the board and the sensor.
- The USB-C breakout lies on the floor of the foot, under the X6, its
  receptacle in the port at the back. Two M3 bolts from below hold it, with
  nuts on top.
- The PWR, BOOT and RST buttons and the TF slot are reached with the cover
  off.

## Fasteners and extras

- 3 × M2×6 countersunk screws for the screen
- 2 × M2×10 countersunk bolts and 2 × M2 nuts for the X6
- 8 × M2×6 countersunk screws and 8 × M2 heat-set inserts for the covers
- 2 × M2×6 countersunk screws and 2 × M2 heat-set inserts to join the shell
  halves
- 4 × 8 mm adhesive rubber feet
- 1 USB-C female breakout, 21.6 × 12.7 mm, and 2 × M3×6 countersunk bolts
  and 2 × M3 nuts for it
- 1 short male USB-C pigtail, right-angle, into the board's own port

## Power

Everything comes in through the breakout at the back. Its VBUS and GND
split inside: one branch to the X6's 5 V and GND, the other through the
pigtail into the board's USB-C, together with D+ and D-, so flashing over
the same port still works. Don't feed 5 V into the `BAT` connector; it
expects a single 3.7 V lithium cell.

## Still to confirm

- `board_edge` (4.5) was read off a caliper photo.
- The M2 hole centres on both boards (`board_holes`, `sensor_holes`) and the
  X6's gas element and connector positions were read off the drawings to
  about 0.5 mm.
- The breakout's hole centres (`breakout_holes`) and how far its receptacle
  overhangs the PCB (`usbc_overhang`) were read off a product photo.
