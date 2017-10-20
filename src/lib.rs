use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/*
if y > 0:
[x, ... ,y] → [x, ... ,[x, ... ,y-1]]

if y = 0:
[x, ... ,0] → [x-1, ... ,1]

if x = 0 and y = 0 and "..." isn't entirely composed of zeros:
[0, ... ,0] → [0, ... ]

if x = 0, y = 0, and "..." is entirely composed of zeros:
[0, ...0... ,0] → 0
*/

#[derive(Debug, Clone)]
pub struct ChromaList(Vec<Vec<u16>>);

impl FromStr for ChromaList {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') || !s.ends_with(']') {
            Err(())
        } else {
            let index = s[1..].find('[');
            if let Some(i) = index {
                let list: Vec<_> = s[1..i].split(' ').map(|s| s.trim().parse::<u16>().unwrap()).collect();
                let mut list = vec![list];
                list.append(&mut ChromaList::from_str(&s[i+1..s.len()-1])?.0);
                Ok(ChromaList(list))
            } else {
                Ok(ChromaList(vec![s[1..s.len()-1]
                    .split(' ')
                    .map(|s| s.trim().parse::<u16>().unwrap())
                    .collect()]))
            }
        }
    }
}

impl Display for ChromaList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let last_list = self.0.len() - 1;

        for (j, list) in self.0.iter().enumerate() {
            let last_elem = list.len() - 1;
            write!(f, "[")?;

            for (i, n) in list.iter().enumerate() {
                write!(f, "{}", n)?;
                if j != last_list || i != last_elem {
                    write!(f, " ")?;
                }
            }
        }
        write!(f, "{}", "]".repeat(self.0.len()))
    }
}

impl Iterator for ChromaList {
    type Item = Self;

    fn next(&mut self) -> Option<Self> {
        if self.len() <= 1 {
            return None;
        }
        let ret = self.clone();

        self.next_mutation();

        Some(ret)
    }
}

impl ChromaList {
    pub fn len(&self) -> usize {
        self.0.iter().map(|l| l.len()).sum()
    }

    pub fn next_mutation(&mut self) {
        let ChromaList(ref mut lists) = *self;

        if lists.last().unwrap().is_empty() {
            lists.pop();
        }

        if lists.last().unwrap().iter().cloned().sum::<u16>() == 0 {
            lists.pop();

            if let Some(last_mut) = lists.last_mut() {
                last_mut.push(0);
            }

            return
        }

        let (x, y);

        {
            let last = lists.last_mut().unwrap();

            x = *last.first().unwrap();
            y = last.pop().unwrap();
        }

        if y > 0 {
            let mut nested_list = lists.last().unwrap().clone();
            nested_list.push(y-1);

            lists.push(nested_list);
        } else if x > 0 {
            let last = lists.last_mut().unwrap();

            *last.first_mut().unwrap() -= 1;
            last.push(1);
        }
    }
}
