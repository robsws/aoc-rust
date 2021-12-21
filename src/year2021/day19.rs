
// Pick first scanner
// While there are still unplaced scanners
//  For each placed scanner
//   For each remaining scanner
//    For each pair of beacon coords between placed and unplaced
//     For each rotation
//      Align coordinate systems with rotation and translation
//      Check that at least 12 beacons overlap between two sets of coords
//       If they do, place the scanner
// Once all scanners are placed, determine number of unique coords across all scanners