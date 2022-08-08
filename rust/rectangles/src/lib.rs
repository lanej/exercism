const CORNER: &'static char = &'+';
type Point = (usize, usize);
type Rectangle = (Point, Point, Point, Point);

pub fn count(lines: &[&str]) -> u32 {
    let mut corners: Vec<Point> = Vec::new();
    let mut rectangles: Vec<Rectangle> = Vec::new();

    for (i, row) in lines.iter().enumerate() {
        for (j, val) in row.chars().enumerate() {
            if val == *CORNER {
                corners.push((i, j));
            }
        }
    }

    corners.iter().for_each(|tl|
        corners
            .iter()
            .filter(|(x, y)| &tl.0 == x && &tl.1 > y)
            .for_each(|tr| {
                corners
                    .iter()
                    .filter(|(x, y)| &tr.0 > x && &tr.1 == y)
                    .for_each(|br| {
                        corners
                            .iter()
                            .filter(|(x, y)| &br.1 == y && &br.0 == x)
                            .for_each(|p| rectangles.push((*tl, *tr, *br, *p)))
                    })
            })
    );

    dbg!(&rectangles);

    return rectangles.len().try_into().unwrap();
}
