use std::vec;

#[derive(Debug, Clone)]
pub(crate) struct SepVec<T: Clone, U: Clone + Into<String>> {
    vector: Vec<T>,
    separator: U,
}
impl<T: Clone, U: Clone> SepVec<T, U>
where
    String: From<U>,
{
    pub(crate) fn new<I>(starting: I, separator: U) -> Self
    where
        I: IntoIterator,
        Vec<T>: FromIterator<<I as IntoIterator>::Item>,
    {
        let vec = starting.into_iter().collect::<Vec<T>>();

        Self {
            vector: vec,
            separator,
        }
    }

    pub(crate) fn len_vector(&self) -> usize {
        self.vector.len()
    }

    pub(crate) fn len_total(&self) -> usize {
        (self.vector.len() * 2) - 1
    }

    pub(crate) fn inner_vec(&self) -> Vec<T> {
        self.vector.clone()
    }
}

impl<T, U> SepVec<T, U>
where
    T: Clone,
    String: From<U>,
    U: Into<T> + Clone,
{
    pub fn to_vec(&self) -> Vec<T> {
        let vector = self.vector.clone();

        let mut final_vector: Vec<T> = vec![];
        let len = vector.len();
        for x in vector.into_iter().zip(0..len) {
            final_vector.push(x.0);
            if x.1 != len - 1 {
                final_vector.push(Into::<T>::into(self.separator.clone()));
            };
        }

        return final_vector;
    }
}

impl<T: Clone, U: Clone> ToString for SepVec<T, U>
where
    String: From<U>,
    T: Into<String>,
{
    fn to_string(&self) -> String {
        let vector = self.vector.clone();
        let len = vector.len();
        let mut final_string = String::new();
        for item in vector.into_iter().zip(0..len) {
            let x: String = item.0.into();
            final_string.push_str(x.as_str());
            if item.1 != len - 1 {
                final_string.push_str(Into::<String>::into(self.separator.clone()).as_str());
            }
        }

        final_string
    }
}
