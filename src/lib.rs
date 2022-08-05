/// Ajoute 1 au nombre passé
///
/// # exemple
///
/// ```
/// use adder::ajouter_un;
/// assert_eq!(ajouter_un(3), 4);
/// ```
///
pub fn ajouter_un(x: i32) -> i32
{
    x + 1
}


#[cfg(test)]
mod tests {
    use crate::ajouter_un;

    #[test]
    fn it_works() {

        assert_eq!(ajouter_un(3), 4);
    }
}
