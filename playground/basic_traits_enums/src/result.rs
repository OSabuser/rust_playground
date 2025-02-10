fn main() {
    let email = String::from("akimov@mail.ru");

    match access_data(email) {
        Ok(data) => {
            println!("Data:{:?}", data);
        }
        Err(AccessError::Login(LoginError::IncorrectEmail(email))) => {
            println!("Incorrect email: {}", email);
        }
        Err(AccessError::Login(LoginError::UserNotFound)) => {
            println!("User not found");
        }
        Err(AccessError::Data(DataError::PermissionRequired)) => {
            println!("Permission required");
        }
        Err(e) => {
            process_err(e);
        }
    }
}

fn process_err<E: Error>(err: E) {
    println!("Error: {}", err);
    eprintln!("Error: {:?}", err);
    panic!("Error: {}", err);
}

fn access_data(email: String) -> Result<Data, AccessError> {
    
    // Нужно реализовать From для LoginError и DataError (преобразование типов)
    let user = login(email)?;
    // Или
    // let user = match login(email) {
    //     Ok(user) => user,
    //     Err(err) => {
    //         return Err(err.into());
    //     }
    // };


    // get_user_data(&user)
    //     .map(|data| Data { data: data }) // Ok (Data {data: data}) or Err
    //     .map_err(|err| AccessError::Data(err)) // Err
    // Или
    Ok(Data {
        data: get_user_data(&user)?,
    })
}

#[derive(Debug)]
struct Data {
    data: [u8; 16],
}

fn get_user_data(user: &User) -> Result<[u8; 16], DataError> {
    let data_found = true; // User finding logic
    let permission = true; // Permission logic

    if !permission {
        return Err(DataError::PermissionRequired);
    }

    if data_found {
        Ok([0; 16])
    } else {
        Err(DataError::DataNotFound)
    }
}

fn login(email: String) -> Result<User, LoginError> {
    let email_valid = false; // Email validation logic

    if !email_valid {
        return Err(LoginError::IncorrectEmail(email));
    }

    let user_found = true; // User finding logic

    if user_found {
        Ok(User { email: email })
    } else {
        Err(LoginError::UserNotFound)
    }
}

struct User {
    email: String,
}

#[derive(Debug)]
enum AccessError {
    Login(LoginError),
    Data(DataError),
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

impl From<LoginError> for AccessError {
    fn from(err: LoginError) -> Self {
        AccessError::Login(err)
        //Self::Login(err)
    }
}

impl From<DataError> for AccessError {
    fn from(err: DataError) -> Self {
        AccessError::Data(err)
    }
}



impl Error for AccessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AccessError::Login(err) => Some(err),
            AccessError::Data(err) => Some(err),
        }
    }
}

impl Error for LoginError {}

impl Error for DataError {}

impl Display for AccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessError::Login(err) => write!(f, "Login error: {}", err),
            AccessError::Data(err) => write!(f, "Data error: {}", err),
        }
    }
}

impl Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginError::IncorrectEmail(email) => write!(f, "Incorrect email: {}", email),
            LoginError::UserNotFound => write!(f, "User not found"),
        }
    }
}

impl Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::PermissionRequired => write!(f, "Permission required"),
            DataError::DataNotFound => write!(f, "Data not found"),
        }
    }
}

// use my_mod::MyEnum::{Variant1, Variant2};

use core::panic;
use std::{error::Error, fmt::Display};

use my_mod::{MyEnum, MyEnum::*};

fn some_fn() -> MyEnum {
    Variant1
}

mod my_mod {
    pub enum MyEnum {
        Variant1,
        Variant2,
    }
}
