use crate::month::Month;

#[derive(Debug)]
pub struct MonthGenerator {
    month: Option<Month>,
}

impl MonthGenerator {
    pub fn new(start: Month) -> Self {
        MonthGenerator { month: Some(start) }
    }

    pub fn nth_prev(&mut self, mut n: usize) -> Option<Month> {
        loop {
            match self.prev() {
                Some(x) => {
                    if n == 0 {
                        return Some(x);
                    }
                }
                None => return None,
            }
            n -= 1;
        }
    }

    fn prev(&mut self) -> Option<Month> {
        let month = match &self.month {
            Some(m) => m,
            None => return None,
        };

        let mut year = month.year();
        let mut next_month_number = month.month_number() - 1;
        if next_month_number < 1 {
            next_month_number = 12;
            year -= 1;
        }

        let result = self.month.clone();
        self.month = Month::new(year, next_month_number);

        result
    }
}

impl Iterator for MonthGenerator {
    type Item = Month;

    fn next(&mut self) -> Option<Self::Item> {
        let month = match &self.month {
            Some(m) => m,
            None => return None,
        };

        let mut year = month.year();
        let mut next_month_number = month.month_number() + 1;
        if next_month_number > 12 {
            next_month_number = 1;
            year += 1;
        }

        let result = self.month.clone();
        self.month = Month::new(year, next_month_number);

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn february_after_january() {
        let jan = Month::new(2019, 1).unwrap();
        let mut iter = MonthGenerator::new(jan);

        let actual = iter.nth(1);

        assert_eq!(Month::new(2019, 2), actual);
    }

    #[test]
    fn march_two_months_after_january() {
        let jan = Month::new(2019, 1).unwrap();
        let mut iter = MonthGenerator::new(jan);

        let actual = iter.nth(2);

        assert_eq!(Month::new(2019, 3), actual);
    }

    #[test]
    fn january_after_december() {
        let dec = Month::new(2019, 12).unwrap();
        let mut iter = MonthGenerator::new(dec);

        let actual = iter.nth(1);

        assert_eq!(Month::new(2020, 1), actual);
    }

    #[test]
    fn january_before_february() {
        let feb = Month::new(2019, 2).unwrap();
        let mut iter = MonthGenerator::new(feb);

        iter.prev();
        let actual = iter.prev();

        assert_eq!(Month::new(2019, 1), actual);
    }

    #[test]
    fn december_before_january() {
        let jan = Month::new(2019, 1).unwrap();
        let mut iter = MonthGenerator::new(jan);

        let actual = iter.nth_prev(1);

        assert_eq!(Month::new(2018, 12), actual);
    }

    #[test]
    fn january_two_months_before_march() {
        let march = Month::new(2019, 3).unwrap();
        let mut iter = MonthGenerator::new(march);

        let actual = iter.nth_prev(2);

        assert_eq!(Month::new(2019, 1), actual);
    }
}
