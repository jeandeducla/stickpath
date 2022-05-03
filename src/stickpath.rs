use std::str::FromStr;

pub struct Graph(Vec<Vec<char>>);

impl Graph {
    fn top_labels(&self) -> Vec<char> {
        self.0.first().unwrap().clone()
    }

    fn get(&self, position: (usize, usize)) -> Option<&char> {
        let row = self.0.get(position.0)?;
        row.get(position.1)
    }

    pub fn is_valid(&self, h: usize, w: usize) -> bool {
        self.0.len() == h && self.0.get(0).unwrap().len() == w
    }

    fn solve_from_column(&self, column: usize) -> &char {
        let mut position = (1, column);
        loop {
            if position.0 >= self.0.len() - 1 {
                break;
            }
            let neighbors = self.neighbor_positions(position);
            position = match neighbors {
                (' ', '|', ' ') => (position.0 + 1, position.1),
                ('-', _, _) => (position.0 + 1, position.1 - 3),
                (_, _, '-') => (position.0 + 1, position.1 + 3),
                _ => {
                    position = (position.0 + 1, position.1);
                    break;
                }
            };
        }
        self.get(position).unwrap()
    }

    fn neighbor_positions(&self, position: (usize, usize)) -> (&char, &char, &char) {
        let left = match position.1.overflowing_sub(1) {
            (_, true) => &' ',
            (sub, false) => self.get((position.0, sub)).unwrap_or(&' '),
        };
        let down = match position.0.overflowing_add(1) {
            (_, true) => &' ',
            (add, false) => self.get((add, position.1)).unwrap_or(&' '),
        };
        let right = match position.1.overflowing_add(1) {
            (_, true) => &' ',
            (add, false) => self.get((position.0, add)).unwrap_or(&' '),
        };

        (left, down, right)
    }

    pub fn solve_all(&self) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        for (col, label) in self.top_labels().iter().enumerate().step_by(3) {
            let result = self.solve_from_column(col);
            results.push(format!("{}{}", label, result));
        }
        results
    }
}

impl FromStr for Graph {
    type Err = ();

    fn from_str(s: &str) -> Result<Graph, ()> {
        let mut rows = Vec::new();
        let lines = s.lines().map(|l| l.to_string()).collect::<Vec<String>>();

        // at least a top label row, a bottom label row and one connector row
        if lines.len() < 3 {
            return Err(());
        }

        let mut width: usize = 0;
        for (i, l) in lines.iter().enumerate() {
            if i == 0 {
                // first row defines width...
                width = l.chars().count();
                // ... make sure width is valid
                if width % 3 != 1 {
                    return Err(());
                }
            } else {
                // ... make sure width is the same for all rows
                if l.chars().count() != width {
                    return Err(());
                }
            }

            if i > 0 && i < lines.len() - 1 {
                // make sure there are vertical connectors where expected
                if l.chars()
                    .enumerate()
                    .filter(|(i, _)| i % 3 == 0)
                    .any(|(_, c)| c != '|')
                {
                    return Err(());
                }
                // make sure horizontal connectors are valid i.e. either "  " or "--"
                if l.chars()
                    .enumerate()
                    .filter(|(i, _)| i % 3 != 0)
                    .map(|(_, c)| c)
                    .collect::<Vec<char>>()
                    .chunks(2)
                    .any(|c| c != [' ', ' '] && c != ['-', '-'])
                {
                    return Err(());
                }
            } else {
                // make sure label rows are valid
                if l.chars()
                    .enumerate()
                    .filter(|(i, _)| i % 3 != 0)
                    .any(|(_, c)| c != ' ')
                {
                    return Err(());
                }
            }

            rows.push(l.chars().collect());
        }

        Ok(Graph(rows))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "A  B  C\n\
                     |  |  |\n\
                     |--|  |\n\
                     |  |--|\n\
                     |  |--|\n\
                     |  |  |\n\
                     1  2  3\n";

        let sticky_path = Graph::from_str(input).unwrap();

        let res = sticky_path.solve_all();
        assert_eq!(
            res,
            vec!["A2".to_string(), "B1".to_string(), "C3".to_string()]
        )
    }

    #[test]
    fn example1_whatever_labels() {
        let input = "7  \"  %\n\
                     |  |  |\n\
                     |--|  |\n\
                     |  |--|\n\
                     |  |--|\n\
                     |  |  |\n\
                     |  +  |\n";

        let sticky_path = Graph::from_str(input).unwrap();

        let res = sticky_path.solve_all();
        assert_eq!(
            res,
            vec!["7+".to_string(), "\"|".to_string(), "%|".to_string()]
        )
    }

    #[test]
    fn example2() {
        let input = "P  Q  R  S  T  U  V  W\n\
                     |  |  |  |  |--|  |  |\n\
                     |  |  |--|  |  |  |--|\n\
                     |  |--|  |--|  |  |  |\n\
                     |--|  |--|  |  |  |--|\n\
                     |--|  |  |  |  |--|  |\n\
                     |  |--|  |  |--|  |--|\n\
                     |  |  |  |--|  |--|  |\n\
                     |--|  |  |  |--|  |  |\n\
                     |  |  |--|  |  |  |  |\n\
                     |  |  |  |--|  |  |--|\n\
                     |  |  |  |  |--|  |  |\n\
                     |--|  |  |  |  |  |  |\n\
                     |--|  |--|  |  |  |--|\n\
                     |  |--|  |  |--|  |  |\n\
                     |  |  |--|  |  |  |--|\n\
                     |--|  |--|  |  |--|  |\n\
                     1  2  3  4  5  6  7  8";

        let sticky_path = Graph::from_str(input).unwrap();

        let res = sticky_path.solve_all();
        assert_eq!(
            res,
            vec![
                "P3".to_string(),
                "Q7".to_string(),
                "R8".to_string(),
                "S5".to_string(),
                "T6".to_string(),
                "U2".to_string(),
                "V4".to_string(),
                "W1".to_string(),
            ]
        )
    }

    #[test]
    fn invalid_graphs() {
        let input = "A B  C\n\
                     |  |  |\n\
                     |--|  |\n\
                     |  |--|\n\
                     |  |--|\n\
                     |  |  |\n\
                     1  2  3\n";
        assert!(Graph::from_str(input).is_err());

        let input = "A  B  C\n\
                     |  |  |\n\
                     |--*  |\n\
                     |  |--|\n\
                     |  |--|\n\
                     |  |  |\n\
                     1  2  3\n";
        assert!(Graph::from_str(input).is_err());

        let input = "A  B  C\n\
                     |  |  |\n\
                     |--|  |\n\
                     |  |- |\n\
                     |  |--|\n\
                     |  |  |\n\
                     1  2  3\n";
        assert!(Graph::from_str(input).is_err());

        let input = "A  B  C\n\
                     |  |  |\n\
                     |--|  |\n\
                     |  |--|  |\n\
                     |  |--|  |\n\
                     |  |  |--|\n\
                     1  2  3\n";
        assert!(Graph::from_str(input).is_err());

        let input = "A  B  C\n\
                     |  |  |\n\
                     |--|  |\n\
                     |  |--|\n\
                     |  |--|\n\
                     |  |  |
                     1  2  3\n";
        assert!(Graph::from_str(input).is_err());

        let input = "A  B  C\n\
                     |  |  |\n\
                     |--|  |\n\
                     |  |--|\n\
                     |  |--|\n\
                     |  |  |\n\
                     1| 2  3\n";
        assert!(Graph::from_str(input).is_err());

        let input = "A  B  C\n\
                     |  |  |\n\
                     |--|  |\n\
                     |  |--|\n\
                     |  |--|\n\
                     |  |  |\n\
                     1 2  3\n";
        assert!(Graph::from_str(input).is_err());
    }
}
