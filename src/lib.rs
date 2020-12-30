use std::vec;


pub trait Transposable<T>
{
    fn transpose(self) -> T;
}

impl <T,E> Transposable<Option<Result<T,E>>> for Result<Option<T>,E>{
    fn transpose(self) -> Option<Result<T,E>>{
        self.transpose()
    }
}

impl <T,E> Transposable<Result<Option<T>, E>> for Option<Result<T, E>>{
    fn transpose(self) -> Result<Option<T>, E>{
        self.transpose()
    }
}

impl <T> Transposable<Vec<Vec<T>>> for Vec<Vec<T>> {
    fn transpose(self) -> Vec<Vec<T>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Transposable;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn rust_doc_test(){
        #[derive(Debug, Eq, PartialEq)]
        struct SomeErr;

        let x: Result<Option<i32>, SomeErr> = Ok(Some(5));
        let y: Option<Result<i32, SomeErr>> = Some(Ok(5));
        assert_eq!(Transposable::transpose(x), y);
    }

    #[test]
    fn rust_doc_test_options(){
        #[derive(Debug, Eq, PartialEq)]
        struct SomeErr;
        
        let x: Result<Option<i32>, SomeErr> = Ok(Some(5));
        let y: Option<Result<i32, SomeErr>> = Some(Ok(5));
        assert_eq!(x, Transposable::transpose(y));
    }

    #[test]
    fn transpose_vector(){
        let x = vec![vec![1],vec![2]];
        let y = vec![vec![1,2]];
        assert_eq!(Transposable::transpose(x),y)
    }
}
