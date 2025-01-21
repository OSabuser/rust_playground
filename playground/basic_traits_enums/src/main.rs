fn main() {

}

fn access() -> AccessError {
    let login_err = LoginError::UserNotFound;

    // AccessError::my_from(login_err)
    // Или
    login_err.my_into() //=> login_err.into
    // Обычно релиазуется From (Into работает для типов, которые реализуют From)
}

trait MyInto<U> {
    fn my_into(self) -> U;
}

impl<T, U> MyInto<U> for T where U: MyFrom<T> {
    fn my_into(self) -> U {
        U::my_from(self)
    }
}

trait MyFrom<T> {
    fn my_from(value: T) -> Self;
}

#[derive(Debug)]
enum AccessError {
    Login(LoginError),
    Data(DataError),
}

impl MyFrom<LoginError> for AccessError {
    fn my_from(value: LoginError) -> Self {
        AccessError::Login(value)
    }
}

impl MyFrom<DataError> for AccessError {
    fn my_from(value: DataError) -> Self {
        AccessError::Data(value)
    }
    
}


#[derive(Debug)]
enum LoginError {
    IncorrectEmail(String),
    UserNotFound,
}

#[derive(Debug)]
enum DataError {
    PermissionRequired,
    DataNotFound,
}
