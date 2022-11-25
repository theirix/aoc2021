use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::hash::Hash;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(79, 3621);

/* Impl */

static COMMON_BEACONS: usize = 12;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, PartialEq)]
struct View {
    beacons: Vec<Pos>,
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl fmt::Debug for View {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n")?;
        for b in &self.beacons {
            writeln!(f, "{},{},{}", b.x, b.y, b.z).unwrap();
        }
        write!(f, "}}")
    }
}

fn read_pos(line: &str) -> Pos {
    let nums: Vec<i32> = line
        .trim()
        .split_terminator(',')
        .map(|s| s.parse().unwrap())
        .collect();
    Pos {
        x: nums[0],
        y: nums[1],
        z: nums[2],
    }
}

enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug)]
struct Transformation {
    ix: i32,
    iy: i32,
    iz: i32,
}

fn rotate(x: i32, y: i32, angle: i32) -> (i32, i32) {
    let rad = (angle as f32).to_radians();
    let nx = ((x as f32) * rad.cos() - (y as f32) * rad.sin()).round() as i32;
    let ny = ((y as f32) * rad.cos() + (x as f32) * rad.sin()).round() as i32;
    (nx, ny)
}

fn transform(pos: &Pos, axis: &Axis, angle: i32) -> Pos {
    match axis {
        Axis::X => {
            let x = pos.x;
            let (y, z) = rotate(pos.y, pos.z, angle);
            Pos { x, y, z }
        }
        Axis::Y => {
            let (x, z) = rotate(pos.x, pos.z, -angle);
            let y = pos.y;
            Pos { x, y, z }
        }
        Axis::Z => {
            let (x, y) = rotate(pos.x, pos.y, angle);
            let z = pos.z;
            Pos { x, y, z }
        }
    }
}

fn transform3(pos: &Pos, transformation: &Transformation) -> Pos {
    let mut result = transform(pos, &Axis::X, transformation.ix);
    result = transform(&result, &Axis::Y, transformation.iy);
    result = transform(&result, &Axis::Z, transformation.iz);
    result
}

fn enumerate_transformations() -> Vec<Transformation> {
    let mut result = Vec::new();
    for ix in [0, 90] {
        for iy in [0, 90, 180, 270] {
            for iz in [0, 90, 180, 270] {
                if ix == 90 && (iy == 90 || iy == 270) {
                    continue;
                }
                let transformation = Transformation { ix, iy, iz };
                result.push(transformation);
            }
        }
    }
    result
}

#[allow(dead_code)]
fn enumerate_pos_transforms(pos: Pos) -> Vec<Pos> {
    let mut result = Vec::<Pos>::new();
    for transformation in enumerate_transformations() {
        let npos = transform3(&pos, &transformation);

        if result.contains(&npos) {
            println!("DUP {:?}\t: {:?}", &transformation, &npos);
        }
        result.push(npos);
    }
    result
}

fn apply_transform(base: &View, transformation: &Transformation) -> View {
    View {
        beacons: base
            .beacons
            .iter()
            .map(|pos| transform3(pos, transformation))
            .collect(),
    }
}

fn distance(pos1: &Pos, pos2: &Pos) -> u32 {
    (((pos1.x - pos2.x).pow(2) + (pos1.y - pos2.y).pow(2) + (pos1.z - pos2.z).pow(2)) as f32)
        .sqrt()
        .round() as u32
}

fn manhattan_distance(pos1: &Pos, pos2: &Pos) -> u32 {
    ((pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs() + (pos1.z - pos2.z).abs()) as u32
}

fn shift(pos1: &Pos, pos2: &Pos) -> Pos {
    Pos {
        x: pos2.x - pos1.x,
        y: pos2.y - pos1.y,
        z: pos2.z - pos1.z,
    }
}

fn read_scanners(lines: Vec<String>) -> Vec<View> {
    let mut scanners: Vec<View> = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("---") {
            // switch
            scanners.push(View {
                beacons: Vec::new(),
            });
            continue;
        }
        let pos = read_pos(&line);
        scanners.last_mut().unwrap().beacons.push(pos);
    }
    scanners
}

// Map from distance to pair of beacons
type Fingerprints = HashMap<u32, Vec<(Pos, Pos)>>;

fn make_fingerprints(view: &View, capacity: usize) -> Fingerprints {
    let mut fingerprints: Fingerprints = HashMap::with_capacity(capacity);
    for pos1 in &view.beacons {
        for pos2 in &view.beacons {
            if pos1 == pos2 {
                continue;
            }
            let dist = distance(pos1, pos2);
            if fingerprints.get(&dist).is_none()
                || !fingerprints[&dist]
                    .iter()
                    .any(|x| x.0 == *pos2 && x.1 == *pos1)
            {
                fingerprints
                    .entry(dist)
                    .or_insert_with(|| Vec::<(Pos, Pos)>::with_capacity(3))
                    .push((*pos1, *pos2));
            }
        }
    }
    fingerprints
}

fn check_align(pair1: &(Pos, Pos), pair2: &(Pos, Pos)) -> Option<Pos> {
    let verbose = false;
    let fw_shift = shift(&pair2.0, &pair1.0);
    let bk_shift = shift(&pair2.1, &pair1.0);
    let fw_shift2 = shift(&pair2.1, &pair1.1);
    let bk_shift2 = shift(&pair2.0, &pair1.1);
    if verbose {
        println!("pair1 {:?}", pair1);
        println!("pair2 {:?}", pair2);
        println!(
            "shift {:?} ; {:?} ; {:?} ; {:?}",
            fw_shift, fw_shift2, bk_shift, bk_shift2
        );
    }

    if fw_shift == fw_shift2 {
        return Some(fw_shift);
    }
    if bk_shift == bk_shift2 {
        return Some(bk_shift);
    }
    None
}

/*
fn detect_shift_majority(fingerprints1: &Fingerprints, fingerprints2: &Fingerprints) -> Option<Pos> {
    let verbose = true;

    let fdist1: HashSet<&u32> = fingerprints1.keys().collect();
    let fdist2: HashSet<&u32> = fingerprints2.keys().collect();

    let mut intersections: Vec<&u32> = fdist1.intersection(&fdist2).cloned().collect();
    intersections.sort();

    //let common_count = intersections.len();
    let common_count = intersections
        .iter()
        .map(|dist| fingerprints1[dist].len())
        .sum::<usize>();
    let required_edges = (COMMON_BEACONS) * ((COMMON_BEACONS) - 1) / 2;
    if verbose {
        println!(
            "Common fingerpints: {}, required {}, total {}",
            common_count,
            required_edges,
            fingerprints1.len()
        );
    }
    if common_count < required_edges as usize {
        return None;
    }

    let mut shifts_freq: HashMap<Pos, usize> = HashMap::new();

    for dist in intersections {
        let pair1 = fingerprints1[dist].first().unwrap();
        let pair2 = fingerprints2[dist].first().unwrap();
        if verbose {
            println!("MAJ pair1 {:?}", pair1);
            println!("MAJ pair2 {:?}", pair2);
            println!(
                "Look to dist {}, {} {}",
                dist,
                fingerprints1[dist].len(),
                fingerprints2[dist].len(),
            );
        }
        if let Some(shift) = check_align(pair1, pair2) {
            //println!(" reg dist {} shift {:?}", dist, shift);
            *shifts_freq.entry(shift).or_default() += 1;
        }
    }
    if !shifts_freq.is_empty() {
        let top_freq: usize = *shifts_freq.values().max().unwrap();
        if top_freq >= COMMON_BEACONS {
            let majority_shift = shifts_freq.iter().find(|(k, v)| **v == top_freq).unwrap().0;
            println!(
                "Majority shift {:?} with freq {}",
                majority_shift, top_freq
            );
            if shifts_freq.len() > 1 {
                println!("NOTE there are other shifts {:?}", shifts_freq);
            }
            return Some(*majority_shift);
        }
    }
    None
}
*/

fn detect_shift(fingerprints1: &Fingerprints, fingerprints2: &Fingerprints) -> Option<Pos> {
    let verbose = false;

    let fdist1: HashSet<&u32> = fingerprints1.keys().collect();
    let fdist2: HashSet<&u32> = fingerprints2.keys().collect();

    let mut intersections: Vec<&u32> = fdist1.intersection(&fdist2).cloned().collect();
    intersections.sort();

    //let common_count = intersections.len();
    let common_count = intersections
        .iter()
        .map(|dist| fingerprints1[dist].len())
        .sum::<usize>();
    let required_edges = (COMMON_BEACONS) * ((COMMON_BEACONS) - 1) / 2;
    if verbose {
        println!(
            "Common fingerpints: {}, required {}, total {}",
            common_count,
            required_edges,
            fingerprints1.len()
        );
    }
    if common_count < required_edges as usize {
        return None;
    }

    // Assume there could be only one shift
    // Previous version had selected a majority which is not needed for test data
    let mut shift_count = 0;
    let mut last_shift: Option<Pos> = None;

    for dist in intersections {
        let pair1 = fingerprints1[dist].first().unwrap();
        let pair2 = fingerprints2[dist].first().unwrap();
        if verbose {
            println!("MAJ pair1 {:?}", pair1);
            println!("MAJ pair2 {:?}", pair2);
            println!(
                "Look to dist {}, {} {}",
                dist,
                fingerprints1[dist].len(),
                fingerprints2[dist].len(),
            );
        }
        if let Some(shift) = check_align(pair1, pair2) {
            shift_count += 1;
            last_shift = Some(shift);
        }
    }
    if shift_count >= COMMON_BEACONS {
        return last_shift;
    }
    None
}

#[allow(dead_code)]
fn locate_scanner(base: &View, target: &View) -> Option<(Pos, Transformation)> {
    locate_scanner_ex(base, target, 0)
}

fn locate_scanner_ex(
    base: &View,
    target: &View,
    estimated_fingerprints: usize,
) -> Option<(Pos, Transformation)> {
    let fingerprints_base = make_fingerprints(base, estimated_fingerprints);

    for transformation in enumerate_transformations() {
        let rotation = apply_transform(target, &transformation);

        let fingerprints_target = make_fingerprints(&rotation, estimated_fingerprints);
        let shift = detect_shift(&fingerprints_base, &fingerprints_target);
        if let Some(ashift) = shift {
            println!("Possible shift {:?}", ashift);
            println!("Transformation was {:?}", &transformation);
            return Some((ashift, transformation));
        }
    }
    None
}

type ResolvedScanners = HashMap<u32, (Pos, Transformation)>;

fn detect_scanners(scanners: &Vec<View>) -> ResolvedScanners {
    let mut resolved_scanners: ResolvedScanners = HashMap::new();

    let count = scanners.len() as u32;

    let mut queue: VecDeque<u32> = VecDeque::new();
    let mut discovered: Vec<u32> = vec![];

    let estimated_fg_count = make_fingerprints(&scanners[0], 0).len();

    queue.push_back(0u32);
    discovered.push(0u32);
    resolved_scanners.insert(
        0,
        (
            Pos { x: 0, y: 0, z: 0 },
            Transformation {
                ix: 0,
                iy: 0,
                iz: 0,
            },
        ),
    );

    while !queue.is_empty() {
        let source = queue.pop_front().unwrap();
        //println!(
        //"Handle {}, queue {:?}, discovered {:?}",
        //source, queue, discovered
        //);

        let adjacents = (0..count).filter(|x| *x != source);

        for target in adjacents {
            if !discovered.contains(&target) {
                //println!("Check from {} to {}", source, target);
                let transformed_source =
                    apply_transform(&scanners[source as usize], &resolved_scanners[&source].1);
                if let Some((rel_loc, transformation)) = locate_scanner_ex(
                    &transformed_source,
                    &scanners[target as usize],
                    estimated_fg_count,
                ) {
                    println!("Enqueue {}", target);
                    let base_loc = resolved_scanners[&source].0;
                    let abs_loc = Pos {
                        x: rel_loc.x + base_loc.x,
                        y: rel_loc.y + base_loc.y,
                        z: rel_loc.z + base_loc.z,
                    };
                    /*
                    println!("=== Resolved target {}", target);
                    println!("Base location of {} is {:?}", source, base_loc);
                    println!("Rel location {:?}", rel_loc);
                    println!("Abs location {:?}", abs_loc);
                    println!("Transformation {:?}", transformation);*/
                    resolved_scanners.insert(target as u32, (abs_loc, transformation));
                    queue.push_back(target);
                    discovered.push(target);
                }
            }
        }
    }

    println!(
        "Found locations {}: {:?}",
        resolved_scanners.len(),
        resolved_scanners.keys()
    );
    for (k, v) in &resolved_scanners {
        println!("Scanner {} at {:?}, transformation {:?}", k, v.0, v.1);
    }

    resolved_scanners
}

fn relocate_beacons(resolved_scanners: &ResolvedScanners, scanners: &[View]) -> Vec<Pos> {
    let mut absolute_locations: HashSet<Pos> = HashSet::new();

    for (idx, scanner) in scanners.iter().enumerate() {
        for rel_beacon in &scanner.beacons {
            let (base_loc, transformation) = &resolved_scanners[&(idx as u32)];
            let rel_loc = transform3(rel_beacon, transformation);
            let abs_loc = Pos {
                x: rel_loc.x + base_loc.x,
                y: rel_loc.y + base_loc.y,
                z: rel_loc.z + base_loc.z,
            };
            absolute_locations.insert(abs_loc);
        }
    }

    absolute_locations.iter().copied().collect::<Vec<Pos>>()
}

fn total_scanners_distance(resolved_scanners: &ResolvedScanners) -> u64 {
    let mut distances: Vec<u32> = Vec::new();

    for (loc1, _) in resolved_scanners.values() {
        for (loc2, _) in resolved_scanners.values() {
            let dist = manhattan_distance(loc1, loc2);
            distances.push(dist);
        }
    }
    *distances.iter().max().unwrap_or(&0) as u64
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let scanners = read_scanners(lines);
    println!("Total scanners {:?}", scanners.len());

    let resolved = detect_scanners(&scanners);
    let abs_beacons = relocate_beacons(&resolved, &scanners);
    abs_beacons.len() as u64
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let scanners = read_scanners(lines);
    println!("Total scanners {:?}", scanners.len());

    let resolved = detect_scanners(&scanners);
    total_scanners_distance(&resolved)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn read_scanner(sample: &str) -> View {
        let views = read_scanners(sample.split_terminator('\n').map(String::from).collect());
        let first: &View = views.first().unwrap();
        first.clone()
    }

    #[test]
    fn test_rot_z0() {
        let pos = read_pos("4,1,2");
        let npos = transform(&pos, &Axis::Z, 0);
        assert_eq!(npos, pos);
    }

    #[test]
    fn test_rot_z90() {
        let pos = read_pos("4,1,2");
        let npos = transform(&pos, &Axis::Z, 90);
        assert_eq!(npos, read_pos("-1,4,2"));
    }

    #[test]
    fn test_rot_z180() {
        let pos = read_pos("4,1,2");
        let npos = transform(&pos, &Axis::Z, 180);
        assert_eq!(npos, read_pos("-4,-1,2"));
    }

    #[test]
    fn test_rot_z270() {
        let pos = read_pos("4,1,2");
        let npos = transform(&pos, &Axis::Z, 270);
        assert_eq!(npos, read_pos("1,-4,2"));
    }

    #[test]
    fn test_rot_x90() {
        let pos = read_pos("4,1,2");
        let npos = transform(&pos, &Axis::X, 90);
        assert_eq!(npos, read_pos("4,-2,1"));
    }

    #[test]
    fn test_transform3() {
        let pos = read_pos("4,1,2");
        let npos = transform3(
            &pos,
            &Transformation {
                ix: 180,
                iy: 0,
                iz: 0,
            },
        );
        assert_eq!(npos, read_pos("4,-1,-2"));
    }

    #[test]
    fn test_transform3_dual() {
        let pos = read_pos("4,1,2");
        let npos = transform3(
            &pos,
            &Transformation {
                ix: 180,
                iy: 90,
                iz: 270,
            },
        );
        // x => 4,-1,-2
        // t => -2,-1,-4 (negative sin)
        // z => -2,1,-4
        assert_eq!(npos, read_pos("-1,2,-4"));
    }

    #[test]
    fn test_enumerate_pos_transforms() {
        let pos = read_pos("4,1,2");

        let transformations = enumerate_pos_transforms(pos);
        //assert_eq!(transformations.len(), 64);
        assert_eq!(transformations.len(), 24);

        // 24 unique
        let uniq = HashSet::<&Pos>::from_iter(transformations.iter());
        assert_eq!(uniq.len(), 24);
    }

    static SAMPLE_0: &str = r#"--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7
"#;

    static SAMPLE_1: &str = r#"--- scanner 0 v2 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0
"#;

    static SAMPLE_B0: &str = r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401
"#;

    static SAMPLE_B1: &str = r#"--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390
"#;

    static SAMPLE_B2: &str = r#"--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562
"#;

    static SAMPLE_B3: &str = r#"--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596 
"#;

    static SAMPLE_B4: &str = r#"--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
"#;

    static SAMPLE_C0: &str = r#"--- scanner 0 ---
-618,-824,-621
-537,-823,-458
-447,-329,318
404,-588,-901
544,-627,-890
528,-643,409
-661,-816,-575
390,-675,-793
423,-701,434
-345,-311,381
459,-707,401
-485,-357,347
"#;

    static SAMPLE_C1: &str = r#"--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
-476,619,847
-460,603,-452
729,430,532
-322,571,750
-355,545,-477
413,935,-424
-391,539,-444
553,889,-390
"#;

    #[test]
    fn test_enumerate_transformations() {
        let transformations = enumerate_transformations();
        assert_eq!(transformations.len(), 24);
    }

    #[test]
    fn test_enumerate_peek() {
        let scanner: &View = &read_scanner(SAMPLE_0);
        let peek: &View = &read_scanner(SAMPLE_1);

        let rotations: Vec<View> = enumerate_transformations()
            .iter()
            .map(|t| apply_transform(scanner, &t))
            .collect();
        assert!(rotations.contains(peek));
    }

    #[test]
    fn test_fingerprint_print_subset() {
        let s1: &View = &read_scanner(SAMPLE_C0);

        let fingerprints = make_fingerprints(&s1, 0);
        for (k, v) in fingerprints.iter() {
            println!("dist {}: pair {:?}", k, v);
        }

        assert!(fingerprints.len() < s1.beacons.len().pow(2));
        assert_eq!(fingerprints.values().map(|x| x.len()).sum::<usize>(), 66);
    }

    #[test]
    fn test_fingerprint_print_full() {
        let s1: &View = &read_scanner(SAMPLE_B0);

        let fingerprints = make_fingerprints(&s1, 0);
        for (k, v) in fingerprints.iter() {
            println!("dist {}: pair {:?}", k, v);
        }
    }

    #[test]
    fn test_fingerprint_common_subset() {
        let s1: &View = &read_scanner(SAMPLE_C0);
        let s2: &View = &read_scanner(SAMPLE_C1);

        // Task description lacks explanation that C1 must be rotated before
        let s2x = apply_transform(
            s2,
            &Transformation {
                ix: 0,
                iy: 180,
                iz: 0,
            },
        );

        let fingerprints_s1 = make_fingerprints(&s1, 0);
        let fingerprints_s2 = make_fingerprints(&s2x, 0);

        assert!(detect_shift(&fingerprints_s1, &fingerprints_s2).is_some());
    }

    #[test]
    fn test_fingerprint_common_subset_with_rotations() {
        let s0: &View = &read_scanner(SAMPLE_C0);
        let s1: &View = &read_scanner(SAMPLE_C1);

        let shift = locate_scanner(&s0, &s1);
        assert!(shift.is_some());
        assert_eq!(
            shift.unwrap().0,
            Pos {
                x: 68,
                y: -1246,
                z: -43
            }
        );
    }

    #[test]
    fn test_fingerprint_common_full_with_rotations() {
        let s0: &View = &read_scanner(SAMPLE_B0);
        let s1: &View = &read_scanner(SAMPLE_B1);

        let shift = locate_scanner(&s0, &s1);
        assert!(shift.is_some());
        assert_eq!(
            shift.unwrap().0,
            Pos {
                x: 68,
                y: -1246,
                z: -43
            }
        );
    }

    #[test]
    fn test_fingerprint_s4_with_rotations() {
        let s0: &View = &read_scanner(SAMPLE_B0);
        let s1: &View = &read_scanner(SAMPLE_B4);

        let shift = locate_scanner(&s0, &s1);
        assert!(shift.is_some());
        assert_eq!(
            shift.unwrap().0,
            Pos {
                x: -20,
                y: -1133,
                z: 1061
            }
        );
    }

    #[test]
    fn test_fingerprint_s1to4() {
        let s0: &View = &read_scanner(SAMPLE_B1);
        let s1: &View = &read_scanner(SAMPLE_B4);

        let shift = locate_scanner(&s0, &s1);
        assert!(shift.is_some());
    }

    fn _all_views() -> Vec<View> {
        let sample = [SAMPLE_B0, SAMPLE_B1, SAMPLE_B2, SAMPLE_B3, SAMPLE_B4].join("");
        let views: Vec<View> =
            read_scanners(sample.split_terminator('\n').map(String::from).collect());
        views
    }

    #[test]
    fn test_detect_all() {
        let scanners = _all_views();
        let resolved = detect_scanners(&scanners);
        assert_eq!(resolved.len(), 5);

        assert_eq!(
            resolved[&1].0,
            Pos {
                x: 68,
                y: -1246,
                z: -43
            }
        );
        assert_eq!(
            resolved[&4].0,
            Pos {
                x: -20,
                y: -1133,
                z: 1061
            }
        );
        assert_eq!(
            resolved[&3].0,
            Pos {
                x: -92,
                y: -2380,
                z: -20
            }
        );
        assert_eq!(
            resolved[&2].0,
            Pos {
                x: 1105,
                y: -1205,
                z: 1229
            }
        );
    }

    #[test]
    fn test_detect_manual_topo() {
        let scanners = _all_views();

        assert!(locate_scanner(&scanners[0], &scanners[1]).is_some());
        assert!(locate_scanner(&scanners[1], &scanners[4]).is_some());
        assert!(locate_scanner(&scanners[4], &scanners[2]).is_some());
        assert!(locate_scanner(&scanners[1], &scanners[3]).is_some());
    }

    #[test]
    fn test_detect_s3_rel() {
        let scanners = _all_views();

        let shift = locate_scanner(&scanners[1], &scanners[3]);
        assert!(shift.is_some());
        assert_eq!(
            shift.unwrap().0,
            Pos {
                x: 160,
                y: -1134,
                z: -23,
            }
        );
    }

    #[test]
    fn test_relocate_auto() {
        let scanners = _all_views();
        let resolved = detect_scanners(&scanners);
        assert_eq!(resolved.len(), 5);
        let abs_beacons = relocate_beacons(&resolved, &scanners);
        assert_eq!(abs_beacons.len(), 79);
    }

    #[test]
    fn test_max_distance() {
        let scanners = _all_views();
        let resolved = detect_scanners(&scanners);
        assert_eq!(resolved.len(), 5);
        let total_dist = total_scanners_distance(&resolved);
        assert_eq!(total_dist, 3621);
    }
}
