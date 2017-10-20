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
pub struct ChromaList(Vec<u16>, Option<Box<ChromaList>>);

impl FromStr for ChromaList {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') || !s.ends_with(']') {
            Err(())
        } else {
            let index = s[1..].find('[');
            if let Some(i) = index {
                let list: Vec<_> = s[1..i].split(' ').map(|s| s.trim().parse::<u16>().unwrap()).collect();
                Ok(ChromaList(list, Some(Box::new(ChromaList::from_str(&s[i+1..s.len()-1])?))))
            } else {
                Ok(ChromaList(s[1..s.len()-1]
                    .split(' ')
                    .map(|s| s.trim().parse::<u16>().unwrap())
                    .collect(),
                None))
            }
        }
    }
}

impl Display for ChromaList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[")?;
        let last_elem = self.0.len() - 1;
        for (i, n) in self.0.iter().enumerate() {
            write!(f, "{}", n)?;
            if self.1.is_some() || i != last_elem {
                write!(f, ", ")?;
            }
        }
        if let Some(ref nest) = self.1 {
            nest.fmt(f)?;
        }
        write!(f, "]")
    }
}

impl Iterator for ChromaList {
    type Item = Self;

    fn next(&mut self) -> Option<Self> {
        if self.len() <= 1 {
            return None;
        }
        let ret = self.clone();

        *self = std::mem::replace(self, ChromaList(vec![], None)).next_mutation();

        Some(ret)
    }
}

impl ChromaList {
    pub fn len(&self) -> usize {
        self.0.len() + self.1.as_ref().map(|l| l.len()).unwrap_or(0)
    }

    pub fn next_mutation(self) -> Self {
        let ChromaList(ns, nested_list) = self;

        if ns.is_empty() {
            if let Some(new_list) = nested_list {
                return *new_list;
            } else {
                unreachable!()
            }
        }

        match nested_list {
            Some(nested_list) => {
                if nested_list.1.is_none() && (nested_list.0.len() == 1 || nested_list.0.iter().sum::<u16>() == 0) {
                    let mut ns = ns;
                    ns.push(nested_list.0[0]);
                    ChromaList(ns, None)
                } else {
                    ChromaList(ns, Some(Box::new(nested_list.next_mutation())))
                }
            }
            None => {
                let mut ns = ns;

                let x = *ns.first().unwrap();
                let y = ns.pop().unwrap();

                if y > 0 {
                    let mut nested_list = ns.clone();
                    nested_list.push(y-1);
                    ChromaList(ns, Some(Box::new(ChromaList(nested_list, None))))
                } else if x > 0 {
                    *ns.first_mut().unwrap() -= 1;
                    ns.push(1);
                    ChromaList(ns, None)
                } else {
                    ChromaList(ns, None)
                }
            }
        }
    }
}
