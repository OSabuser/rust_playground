fn main() {
    let email = String::from("akimov@mail.ru");

    match access_data(email) {
        Some(data) => {
            println!("{:?}", data);
        },
        None => {
            println!("No data found");
        }
    }
}

fn access_data(email: String) -> Option<Data>{

    // let user = match login(email) {
    //     Some(user) => user,
    //     None => {
    //         return None;
    //     }
        
    // };
    // Или
    let user = login(email)?;// Если None, то возвратит None (и выйдет из функции ), иначе вернет User

    //get_user_data(&user).map(|data| Data {data: data});// Some(Data {data: data}) or None 
    // Или 
    
    // Или
    Some(Data{data: get_user_data(&user)?}) 
  
}

#[derive(Debug)]
struct Data {
    data: [u8; 16],
}

fn get_user_data(user: &User) -> Option<[u8; 16]> {
    let data_found = true;

    if data_found {
        Some([0;16])
    } else {
        None
        
    }
}

fn login(email: String) -> Option<User> {
    
    let is_user_found = true; // User finding logic

    if is_user_found {
        Some(User {email: email})
    } else {
        None
    }
}

struct User {
    email: String,
}