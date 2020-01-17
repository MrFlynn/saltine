#[allow(dead_code)]
enum GeneratorState {
    Init,
    IncBase,
    IncNext,
}

pub struct WordGenerator {
    base: char,
    size: usize,
    elements: Vec<usize>,
    space: Vec<char>,
    state: GeneratorState,
    position: usize,
    count: u64,
}

impl WordGenerator {
    pub fn new(base_char: char, working_size: usize, working_space: Vec<char>) -> WordGenerator {
        let index = working_space.iter().position(|&i| i == base_char).unwrap();

        WordGenerator {
            base: base_char,
            size: working_size - 1,
            elements: vec![index; working_size - 1],
            space: working_space,
            state: GeneratorState::Init,
            position: 0,
            count: 0,
        }
    }

    fn output(&self) -> String {
        let mut password = String::new();
        password.push(self.base);

        for i in (0..self.size).rev() {
            let el = self.space[self.elements[i]];
            password.push(el);
        }

        password
    }

    fn is_exhausted(&self) -> bool {
        self.elements[self.position] >= (self.space.len() - 1)
    }

    fn is_done(&self) -> bool {
        return self.count >= (self.space.len().pow(self.size as u32) as u64);
    }
}

impl Iterator for WordGenerator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }

        if self.is_done() {
            return None;
        }

        loop {
            match self.state {
                GeneratorState::Init => {
                    self.state = GeneratorState::IncBase;
                    break;
                }
                GeneratorState::IncBase => {
                    if self.is_exhausted() {
                        self.elements[self.position] = 0;

                        if self.position < self.size {
                            self.position += 1;
                            self.state = GeneratorState::IncNext;
                        }
                    } else {
                        self.elements[self.position] += 1;
                        break;
                    }
                }
                GeneratorState::IncNext => {
                    if self.position >= self.size {
                        self.position = 0;
                        self.state = GeneratorState::IncBase;
                    } else if self.is_exhausted() {
                        self.elements[self.position] = 0;
                        self.position += 1;
                    } else {
                        self.elements[self.position] += 1;

                        self.state = GeneratorState::IncBase;
                        self.position = 0;

                        break;
                    }
                }
            }
        }

        self.count += 1;

        let output = self.output();
        Some(output)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(self.space.len().pow(self.size as u32)))
    }
}

#[cfg(test)]
mod tests {
    use crate::generator::WordGenerator;

    #[test]
    fn test_word_generator_simple() {
        let generator = WordGenerator::new('a', 2, vec!['a', 'b', 'c']);
        let elements: Vec<String> = generator.take(3).collect();

        assert_eq!(vec!["aa", "ab", "ac"], elements);
    }

    #[test]
    fn test_word_generator_complex() {
        let generator = WordGenerator::new('a', 3, vec!['a', 'b', 'c']);
        let elements: Vec<String> = generator.take(9).collect();

        assert_eq!(
            vec!["aaa", "aab", "aac", "aba", "abb", "abc", "aca", "acb", "acc"],
            elements
        );
    }

    #[test]
    fn test_word_generator_complex_boundary() {
        let generator = WordGenerator::new('a', 4, vec!['a', 'b', 'c']);
        let elements: Vec<String> = generator.take(20).collect();

        assert_eq!("abaa", elements[9]);
        assert_eq!("abab", elements[10]);
        assert_eq!("abba", elements[12]);
    }
}
