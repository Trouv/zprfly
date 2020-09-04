use super::{IMExIterator, IMExVal, Quantifier};

/// An [`IMExVal`](./enum.IMExVal.html) that has been quantified, for use in a parsed
/// [`IMEx`](./struct.IMEx.html).
#[derive(PartialEq, Debug, Clone)]
pub struct QuantifiedIMExVal {
    pub val: IMExVal,
    pub quantifier: Quantifier,
    current_val: (Option<IMExVal>, bool),
}

impl QuantifiedIMExVal {
    pub fn from(val: IMExVal, quantifier: Quantifier) -> QuantifiedIMExVal {
        QuantifiedIMExVal {
            current_val: (None, true),
            val,
            quantifier,
        }
    }
}

impl<T, I> IMExIterator<T, I> for QuantifiedIMExVal
where
    T: Iterator<Item = I>,
{
    fn iterate(&mut self, iters: &mut Vec<T>) -> Option<I> {
        loop {
            match &mut self.current_val.0 {
                Some(val) => match val.iterate(iters) {
                    Some(res) => {
                        self.current_val.1 = true;
                        return Some(res);
                    }
                    None => self.current_val.0 = None,
                },
                None => match (self.quantifier.next(), self.current_val.1) {
                    (Some(_), true) => self.current_val = (Some(self.val.clone()), false),
                    _ => return None,
                },
            }
        }
    }
}
