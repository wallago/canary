# Enclosure

A 3D-printed desk stand for Canary, in two parts. Seen from the side it is a
75° angle: the screen arm leans back 15° and a foot runs back along the desk.
The shell is open on the two inner faces of that angle, and the cover closes
them. The screen and the air sensor both mount on the cover.

Everything is in [`canary.scad`](canary.scad). Every dimension is a named
variable at the top of the file. Units are millimetres.

## Hardware

- [Waveshare ESP32-S3-Touch-LCD-1.85](https://www.waveshare.com/esp32-s3-touch-lcd-1.85.htm),
  plain version (not the C)
- [Waveshare Environment X6](https://www.waveshare.com/environment-x6-sensor.htm)

## Parts

| Part    | What it is                                                                                                                  |
| ------- | --------------------------------------------------------------------------------------------------------------------------- |
| `shell` | Rounded body: screen opening, USB-C debug opening underneath, power socket sleeve at the back, floor vents under the sensor |
| `cover` | Bent plate closing the inner faces: posts that hold the screen, standoffs for the X6, vertical vents, sensor window         |

Export the STLs with `just stl`. They land in `target/enclosure/`.
To export one part by hand:

```sh
nix run nixpkgs#openscad -- -D 'part="shell"' -o shell.stl hardware/enclosure/canary.scad
```

Open the file without `-D` to see the assembly, with the two boards shown as
reference shapes.

## Assembly

- The screen screws to the cover through its own threaded M2 holes, from
  outside the cover.
- The X6 hangs under the foot of the cover on M2 bolts and nuts, component
  side up, so its gas inlet sits in the window and faces out.
- The cover screws to the shell with four M2 screws into heat-set inserts.
- An internal wall keeps the sensor's chamber apart from the board's, so the
  ESP32's heat doesn't reach the sensor.
- The PWR, BOOT and RST buttons and the TF slot are reached with the cover
  off.

## Fasteners and extras

- 3 × M2 countersunk screws for the screen, about 10 mm long
- 2 × M2×12 countersunk bolts and 2 × M2 nuts for the X6
- 4 × M2×6 countersunk screws and 4 × M2 heat-set inserts for the cover
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

- `board_edge` (4.5) was read off a caliper photo; `back_parts` (4.5) is an
  estimate of the tallest part on the back of the board.
- The M2 hole centres on both boards (`board_holes`, `sensor_holes`) were
  read off the drawings to about 0.5 mm.
