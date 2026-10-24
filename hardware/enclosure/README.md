# Enclosure

A 3D-printed desk stand for Canary, in three parts. Seen from the side it is
a 75° angle: the screen arm leans back 15° and a foot runs back along the desk.
The shell is open on the two inner faces of that angle, and two covers close
them: one behind the screen, one over the foot. The screen mounts on the
first, the air sensor on the second.

Everything is in [`canary.scad`](canary.scad). Every dimension is a named
variable at the top of the file. Units are millimetres.

## Hardware

- [Waveshare ESP32-S3-Touch-LCD-1.85](https://www.waveshare.com/esp32-s3-touch-lcd-1.85.htm),
  plain version (not the C)
- [Waveshare Environment X6](https://www.waveshare.com/environment-x6-sensor.htm)

## Parts

| Part           | What it is                                                                                                                  |
| -------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `shell`        | Rounded body: screen opening, USB-C debug opening underneath, power socket sleeve at the back, floor vents under the sensor |
| `screen_cover` | Plate behind the screen: three screw holes for the board, vertical vents                                                    |
| `sensor_cover` | Plate over the foot: standoffs for the X6, window for its gas element, pocket for its connector, vents                      |

Export the STLs with `just stl`. They land in `target/enclosure/`.
To export one part by hand:

```sh
nix run nixpkgs#openscad -- -D 'part="shell"' -o shell.stl hardware/enclosure/canary.scad
```

Open the file without `-D` to see the assembly, with the two boards shown as
reference shapes.

## Assembly

- The screen's brass M2 standoffs rest on the screen cover, and three screws
  go from outside the cover straight into them.
- The X6 hangs under the sensor cover on 4 mm standoffs, component side up,
  held by M2 bolts and nuts through its two 2.2 mm holes on the connector
  edge. Its connector points at the screen and sits in a pocket under the
  cover. The gas element comes up through the window.
- Each cover screws to the shell with four M2 screws into heat-set inserts.
- An internal wall keeps the sensor's chamber apart from the board's, so the
  ESP32's heat doesn't reach the sensor.
- The PWR, BOOT and RST buttons and the TF slot are reached with the cover
  off.

## Fasteners and extras

- 3 × M2×6 countersunk screws for the screen
- 2 × M2×10 countersunk bolts and 2 × M2 nuts for the X6
- 8 × M2×6 countersunk screws and 8 × M2 heat-set inserts for the covers
- 4 × 8 mm adhesive rubber feet
- 1 female USB-C receptacle, glued into the sleeve at the back

## Power

The board's own USB-C is for flashing and debugging only; reach it through
the opening underneath. Everyday power comes in through the glued USB-C
receptacle and is split inside to the board's VBUS and the X6's 5 V pin. Put
a Schottky diode (an SS14 or similar) in series on the board branch, so a USB
host plugged in for flashing never fights the other supply. Don't feed 5 V
into the `BAT` connector; it expects a single 3.7 V lithium cell.

## Still to confirm

- `board_edge` (4.5) was read off a caliper photo.
- The M2 hole centres on both boards (`board_holes`, `sensor_holes`) and the
  X6's gas element and connector positions were read off the drawings to
  about 0.5 mm.
