use pyo3::prelude::*;

#[pyclass]
struct NNSplit {
    splitter: nnsplit::NNSplit,
}

#[pymethods]
impl NNSplit {
    #[new]
    fn new() -> PyResult<Self> {
        let options = nnsplit::NNSplitOptions::default();
        let splitter = nnsplit::NNSplit::load("en", options).unwrap();
        Ok(NNSplit { splitter })
    }

    #[staticmethod]
    fn load(lang: &str) -> PyResult<Self> {
        let options = nnsplit::NNSplitOptions::default();
        let splitter = nnsplit::NNSplit::load(lang, options).unwrap();
        Ok(NNSplit { splitter })
    }

    fn split(&self, texts: Vec<String>) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let parts = self.splitter.split(
            texts
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        );
        for splits in parts.iter() {
            let mut sentences = Vec::new();
            for sentence in splits.iter() {
                sentences.push(sentence.text().to_string());
            }
            result.push(sentences);
        }
        result
    }
}

#[pymodule]
fn nnsplit_unblocked(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NNSplit>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let texts = vec![
            "Hello, world! How are you?".to_string(),
            "I am fine, thank you.".to_string(),
        ];
        let splitter = NNSplit::new().unwrap();
        let result = splitter.split(texts);
        assert_eq!(
            result,
            vec![
                vec!["Hello, world! ".to_string(), "How are you?".to_string()],
                vec!["I am fine, thank you.".to_string()],
            ]
        );
    }
}
