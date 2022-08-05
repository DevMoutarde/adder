
pub mod mathlib
{
    /// Ajoute 1 au nombre passé
    ///
    /// # exemple
    ///
    /// ```
    /// use gabadder::mathlib::ajouter_un;
    /// assert_eq!(ajouter_un(3), 4);
    /// ```
    ///
    pub fn ajouter_un(x: i32) -> i32
    {
        x + 1
    }

    /// Retranche 1 au nombre passé
    ///

    ///
    fn retrancher_un(x: i32) -> i32
    {
        x -1
    }
}



#[cfg(test)]
mod tests {
    use crate::mathlib::ajouter_un;

    #[test]
    fn it_works() {

        assert_eq!(ajouter_un(3), 4);
    }
}
