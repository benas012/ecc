use crate::field::{add_mod, inv_mod, modp, mul_mod, sub_mod};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Curve {
    pub a: i128,
    pub b: i128,
    pub p: i128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point {
    Infinity,
    Affine { x: i128, y: i128 },
}

impl Curve {
    pub fn new(a: i128, b: i128, p: i128) -> Result<Self, String> {
        if p <= 3 {
            return Err("p must be > 3".to_string());
        }

        let discr = modp(4 * modp(a * a * a, p) + 27 * modp(b * b, p), p);
        if discr == 0 {
            return Err("Singular curve: 4a^3 + 27b^2 ≡ 0 mod p".to_string());
        }

        Ok(Self { a, b, p })
    }

    pub fn is_on_curve(&self, point: Point) -> bool {
        match point {
            Point::Infinity => true,
            Point::Affine { x, y } => {
                let lhs = mul_mod(y, y, self.p);
                let rhs = modp(x * x * x + self.a * x + self.b, self.p);
                lhs == rhs
            }
        }
    }

    pub fn negate(&self, point: Point) -> Point {
        match point {
            Point::Infinity => Point::Infinity,
            Point::Affine { x, y } => Point::Affine {
                x,
                y: modp(-y, self.p),
            },
        }
    }

    pub fn add(&self, p1: Point, p2: Point) -> Result<Point, String> {
        if !self.is_on_curve(p1) {
            return Err("p1 is not on curve".to_string());
        }
        if !self.is_on_curve(p2) {
            return Err("p2 is not on curve".to_string());
        }

        match (p1, p2) {
            (Point::Infinity, q) => Ok(q),
            (p, Point::Infinity) => Ok(p),

            (Point::Affine { x: x1, y: y1 }, Point::Affine { x: x2, y: y2 }) => {
                if x1 == x2 && modp(y1 + y2, self.p) == 0 {
                    return Ok(Point::Infinity);
                }

                let lambda = if x1 == x2 && y1 == y2 {
                    if modp(y1, self.p) == 0 {
                        return Ok(Point::Infinity);
                    }
                    let num = add_mod(mul_mod(3, mul_mod(x1, x1, self.p), self.p), self.a, self.p);
                    let den = inv_mod(mul_mod(2, y1, self.p), self.p)?;
                    mul_mod(num, den, self.p)
                } else {
                    let num = sub_mod(y2, y1, self.p);
                    let den = inv_mod(sub_mod(x2, x1, self.p), self.p)?;
                    mul_mod(num, den, self.p)
                };

                let x3 = sub_mod(sub_mod(mul_mod(lambda, lambda, self.p), x1, self.p), x2, self.p);
                let y3 = sub_mod(mul_mod(lambda, sub_mod(x1, x3, self.p), self.p), y1, self.p);

                let r = Point::Affine { x: x3, y: y3 };
                debug_assert!(self.is_on_curve(r));
                Ok(r)
            }
        }
    }

    pub fn double(&self, p: Point) -> Result<Point, String> {
        self.add(p, p)
    }

    pub fn scalar_mul(&self, mut k: u128, point: Point) -> Result<Point, String> {
        if !self.is_on_curve(point) {
            return Err("Point is not on curve".to_string());
        }

        let mut result = Point::Infinity;
        let mut addend = point;

        while k > 0 {
            if k & 1 == 1 {
                result = self.add(result, addend)?;
            }
            addend = self.double(addend)?;
            k >>= 1;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_creation() {
        let curve = Curve::new(1, 1, 23).unwrap();
        assert_eq!(curve.a, 1);
    }

    #[test]
    fn test_point_on_curve() {
        let curve = Curve::new(1, 1, 23).unwrap();
        assert!(curve.is_on_curve(Point::Affine { x: 0, y: 1 }));
    }

    #[test]
    fn test_add_inverse() {
        let curve = Curve::new(1, 1, 23).unwrap();
        let p = Point::Affine { x: 0, y: 1 };
        let neg = curve.negate(p);
        assert_eq!(curve.add(p, neg).unwrap(), Point::Infinity);
    }

    #[test]
    fn test_double_example() {
        let curve = Curve::new(1, 1, 23).unwrap();
        let p = Point::Affine { x: 0, y: 1 };
        assert_eq!(curve.double(p).unwrap(), Point::Affine { x: 6, y: 19 });
    }

    #[test]
    fn test_scalar_example() {
        let curve = Curve::new(1, 1, 23).unwrap();
        let p = Point::Affine { x: 0, y: 1 };
        assert_eq!(curve.scalar_mul(3, p).unwrap(), Point::Affine { x: 3, y: 13 });
    }
}