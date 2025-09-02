use project_euler::project_euler_solution;

//https://www.had2know.org/academics/nearly-equilateral-heronian-triangles.html

project_euler_solution!(094);

/// Almost Equilateral Triangles
///
/// It is easily proved that no equilateral triangle exists with integral length
/// sides and integral area. However, the almost equilateral triangle 5-5-5 has
/// an area of 12 square units.
///
/// We shall define an almost equilateral triangle to be a triangle for which
/// two sides are equal and the third differs by no more than one unit.
///
/// Find the sum of the perimeters of all almost equilateral triangles with
/// integral side lengths and area and whose perimeters do not exceed one
/// billion (1 000 000 000).
fn project_euler_094() -> u64
{
    const UPPER_LIMIT: u64 = 1_000_000_000;

    let case2 =
        Case2::new().take_while(|tri| tri.iter().sum::<u64>() < UPPER_LIMIT);
    let case3 =
        Case3::new().take_while(|tri| tri.iter().sum::<u64>() < UPPER_LIMIT);

    case2
        .chain(case3)
        .map(|tri| tri.iter().sum::<u64>())
        .sum()
}

// https://www.had2know.org/academics/nearly-equilateral-heronian-triangles.html
// We only need the case 2 and 3.
/// Case II: Sides n, n, and n+1
struct Case2
{
    vals: [u64; 3], // store last three terms needed for calculation
    index: usize,
}

impl Case2
{
    const fn new() -> Self
    {
        Case2 {
            vals: [5, 65, 901], /* V(n) sequence for Case II nearly
                                 * equilateral triangles,
                                 * where V(1) = 5, V(2) = 65, V(3) = 901,
                                 * => the shorter side length of the nth
                                 * nearly equilateral Heronian triangle in
                                 * Case II */
            index: 0, /* Start at 0 to iterate through initial sequence
                       * values before recurrence */
        }
    }
}

impl Iterator for Case2
{
    type Item = [u64; 3];

    fn next(&mut self) -> Option<Self::Item>
    {
        let result = match self.index
        {
            0 => Some([5, 5, 6]),
            1 => Some([65, 65, 66]),
            2 => Some([901, 901, 902]),
            _ =>
            {
                // V(n+3) = 15V(n+2) - 15V(n+1) + V(n)
                let next_val =
                    15 * self.vals[2] - 15 * self.vals[1] + self.vals[0];
                self.vals[0] = self.vals[1];
                self.vals[1] = self.vals[2];
                self.vals[2] = next_val;
                Some([next_val, next_val, next_val + 1])
            },
        };
        self.index += 1;
        result
    }
}

/// Case III: Sides n, n+1, and n+1
struct Case3
{
    vals: [u64; 3],
    index: usize,
}

impl Case3
{
    const fn new() -> Self
    {
        Case3 {
            vals: [16, 240, 3360], /* W(n) sequence for Case III nearly
                                    * equilateral triangles,
                                    * where W(1) = 16, W(2) = 240, W(3) =
                                    * 3360 => the shortest side length of the
                                    * nth nearly equilateral Heronian
                                    * triangle in Case III */
            index: 0,
        }
    }
}

impl Iterator for Case3
{
    type Item = [u64; 3];

    fn next(&mut self) -> Option<Self::Item>
    {
        let result = match self.index
        {
            0 => Some([16, 17, 17]),
            1 => Some([240, 241, 241]),
            2 => Some([3360, 3361, 3361]),
            _ =>
            {
                // W(n+3) = 15W(n+2) - 15W(n+1) + W(n)
                let next_val =
                    15 * self.vals[2] - 15 * self.vals[1] + self.vals[0];
                self.vals[0] = self.vals[1];
                self.vals[1] = self.vals[2];
                self.vals[2] = next_val;
                Some([next_val, next_val + 1, next_val + 1])
            },
        };
        self.index += 1;
        result
    }
}
