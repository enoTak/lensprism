
pub trait Lens {
    type S;
    type A;
    fn getter(s: &Self::S) -> Self::A;
    // fn setter(s: &Self::S) -> Box<dyn Fn(Self::A) -> Self::S>;
    fn setter(s: &Self::S, a: Self::A) -> Self::S;

    fn get(s: &Self::S) ->  Self::A {
        return Self::getter(s);
    }

    fn set(s: &Self::S, a: Self::A) -> Self::S {
        return Self::setter(s, a);
    }

    fn modify(s: &Self::S, f: impl Fn(Self::A) -> Self::A) -> Self::S {
        return Self::set(s, f(Self::get(s)));
    }
}

#[cfg(test)]
mod tests {
    use super::{Lens};

    struct User {
        id: i32,
        name: String,
    }

    impl Lens for User {
        type S = User;
        type A = i32;
        fn getter(u: &User) -> i32 {
            return u.id;
        }
        fn setter(u: &User, id: i32) -> User {
            return User {
                id: id,
                name: u.name.clone()
            };
        }
    }

    #[test]
    fn it_works() {
        let user = User {
            id: 32,
            name: "Taro".to_string(),
        };
    
        let result = User::get(&user);
        assert_eq!(result, 32);
        let new_user = User::set(&user, 64);
        let result = User::get(&new_user);
        assert_eq!(result, 64);

        let add = |x: i32| x + 2;
        let new_user = User::modify(&user, add);
        let result = User::get(&new_user);
        assert_eq!(result, 34);
    }
}

