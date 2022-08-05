pub mod mathlib {
    /// Ajoute 1 au nombre passé
    ///
    /// # exemple
    ///
    /// ```
    /// use gabadder::mathlib::ajouter_un;
    /// assert_eq!(ajouter_un(3), 4);
    /// ```
    ///
    pub fn ajouter_un(x: i32) -> i32 {
        x + 1
    }

    /// division de x et y
    ///
    /// # exemple
    ///
    /// ```
    /// use gabadder::mathlib::diviser;
    /// assert_eq!(diviser(3.0, 2.0), 1.5);
    /// ```
    ///
    pub fn diviser(x: f64, y: f64) -> f64 {
        x / y
    }

    /// Retranche 1 au nombre passé
    ///

    ///
    fn retrancher_un(x: i32) -> i32 {
        x - 1
    }
}

#[cfg(test)]
mod tests {
    use crate::mathlib::{ajouter_un, diviser};

    #[test]
    fn it_works() {
        assert_eq!(ajouter_un(3), 4);
    }

    #[test]
    fn it_divide() {
        assert_eq!(diviser(3.0, 2.0), 1.5);
    }
}
